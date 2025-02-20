use super::*;

// Generates the structure definition for the `Foo_Impl` type.
pub(crate) fn gen_impl_struct(inputs: &ImplementInputs) -> syn::ItemStruct {
    let vis = &inputs.original_type.vis;
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let original_ident = &inputs.original_type.ident;

    let mut impl_fields = quote!();

    if let Some(ref base) = inputs.base_class_info {
        let base_ty = &base.field_ty;
        impl_fields.extend(quote! {
            base: #base_ty,
        });
    }

    if inputs.base_class_info.is_none() {
        impl_fields.extend(quote! {
            identity: &'static ::windows_core::IInspectable_Vtbl,
        });
    }

    for interface_chain in inputs.interface_chains.iter() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let chain_field_ident = &interface_chain.field_ident;
        impl_fields.extend(quote! {
            #chain_field_ident: &'static #vtbl_ty,
        });
    }

    impl_fields.extend(quote! {
        this: #original_ident::#generics,
    });

    if inputs.base_class_info.is_none() {
        impl_fields.extend(quote! {
            count: ::windows_core::imp::WeakRefCount,
        });
    }

    parse_verify(quote! {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #vis struct #impl_ident #generics where #constraints {
            #impl_fields
        }
    })
}

/// Generates the `IUnknownImpl` implementation for the `Foo_Impl` type.
pub(crate) fn gen_iunknown_impl(inputs: &ImplementInputs) -> syn::ItemImpl {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;
    let original_ident = &inputs.original_type.ident;

    let query_interface_fn = gen_query_interface(inputs);

    let trust_level = proc_macro2::Literal::usize_unsuffixed(inputs.trust_level);

    // If generating a derived class, then the AddRef implementation just defers to the base.
    let get_count_fn: syn::ImplItemFn = if let Some(_) = inputs.base_class_info {
        parse_quote! {
            #[inline(always)]
            unsafe fn count_field(&self) -> &::windows_core::imp::WeakRefCount {
                unsafe {
                    self.base().count_field()
                }
            }
        }
    } else {
        parse_quote! {
            #[inline(always)]
            unsafe fn count_field(&self) -> &::windows_core::imp::WeakRefCount {
                &self.count
            }
        }
    };

    let identity_interface_fn: syn::ImplItemFn = if let Some(ref base) = inputs.base_class_info {
        let base_field = &base.field_ident;
        parse_quote! {
            fn identity_interface(&self) -> &::windows_core::IInspectable_Vtbl {
                self.#base_field.identity_interface()
            }
        }
    } else {
        parse_quote! {
            fn identity_interface(&self) -> &::windows_core::IInspectable_Vtbl {
                &self.identity
            }
        }
    };

    parse_quote! {
        impl #generics ::windows_core::IUnknownImpl for #impl_ident::#generics where #constraints {
            type Impl = #original_ident::#generics;

            #[inline(always)]
            fn get_impl(&self) -> &Self::Impl {
                &self.this
            }

            #[inline(always)]
            fn get_impl_mut(&mut self) -> &mut Self::Impl {
                &mut self.this
            }

            #[inline(always)]
            fn into_inner(self) -> Self::Impl {
                self.this
            }

            #identity_interface_fn
            #query_interface_fn
            #get_count_fn

            #[inline(always)]
            unsafe fn Release(self_: *mut Self) -> u32 {
                unsafe {
                    let remaining = ::windows_core::IUnknownImpl::count_field(&*self_).release();
                    if remaining == 0 {
                        _ = ::windows_core::imp::Box::from_raw(self_);
                    }
                    remaining
                }
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
                unsafe {
                    if value.is_null() {
                        return ::windows_core::imp::E_POINTER;
                    }
                    *value = #trust_level;
                    ::windows_core::HRESULT(0)
                }
            }

            fn to_object(&self) -> ::windows_core::ComObject<Self::Impl> {
                unsafe {
                    self.count_field().add_ref();
                    ::windows_core::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(self as *const Self as *mut Self)
                    )
                }
            }

            // const INNER_OFFSET_IN_BYTES: usize = ::core::mem::offset_of!(Self, this);
        }
    }
}

/// Generates the `query_interface` method.
fn gen_query_interface(inputs: &ImplementInputs) -> syn::ImplItemFn {
    let queries = inputs.interface_chains.iter().map(|interface_chain| {
        let chain_ty = interface_chain.implement.to_vtbl_ident();
        let chain_field = &interface_chain.field_ident;
        quote! {
            if #chain_ty::matches(&iid) {
                break 'found &self.#chain_field as *const _ as *const ::core::ffi::c_void;
            }
        }
    });

    // Dynamic casting requires that the object not contain non-static lifetimes.
    let enable_dyn_casting = inputs.original_type.generics.lifetimes().count() == 0;
    let dynamic_cast_query = if enable_dyn_casting {
        quote! {
            if iid == ::windows_core::DYNAMIC_CAST_IID {
                // DYNAMIC_CAST_IID is special. We _do not_ increase the reference count for this pseudo-interface.
                // Also, instead of returning an interface pointer, we simply write the `&dyn Any` directly to the
                // 'interface' pointer. Since the size of `&dyn Any` is 2 pointers, not one, the caller must be
                // prepared for this. This is not a normal QueryInterface call.
                //
                // See the `Interface::cast_to_any` method, which is the only caller that should use DYNAMIC_CAST_ID.
                (interface as *mut *const dyn core::any::Any).write(self as &dyn ::core::any::Any as *const dyn ::core::any::Any);
                return ::windows_core::HRESULT(0);
            }
        }
    } else {
        quote!()
    };

    let base_query = if let Some(ref base) = inputs.base_class_info {
        let base_field = &base.field_ident;
        quote! {
            if ::windows_core::IUnknownImpl::query_interface(&self.#base_field, iid, interface).0 >= 0 {
                return ::windows_core::HRESULT(0);
            }
        }
    } else {
        quote!()
    };

    let identity_query = if inputs.base_class_info.is_none() {
        quote! {
            if iid == <::windows_core::IUnknown as ::windows_core::Interface>::IID
            || iid == <::windows_core::IInspectable as ::windows_core::Interface>::IID
            || iid == <::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID {
                break 'found &self.identity as *const _ as *const ::core::ffi::c_void;
            }
        }
    } else {
        quote!()
    };

    let tear_off_query = if inputs.base_class_info.is_none() {
        quote! {
            let tear_off_ptr = count_field.query(&iid, &self.identity as *const _ as *mut _);
            if !tear_off_ptr.is_null() {
                *interface = tear_off_ptr;
                return ::windows_core::HRESULT(0);
            }
        }
    } else {
        quote!()
    };

    parse_quote! {
        unsafe fn query_interface(&self, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            #base_query

            if iid.is_null() || interface.is_null() {
                return ::windows_core::imp::E_POINTER;
            }

            let iid = *iid;
            let count_field = ::windows_core::IUnknownImpl::count_field(self);

            let interface_ptr: *const ::core::ffi::c_void = 'found: {
                #identity_query
                #(#queries)*
                #dynamic_cast_query
                #tear_off_query

                *interface = ::core::ptr::null_mut();
                return ::windows_core::imp::E_NOINTERFACE;
            };

            debug_assert!(!interface_ptr.is_null());
            *interface = interface_ptr as *mut ::core::ffi::c_void;
            count_field.add_ref();
            return ::windows_core::HRESULT(0);
        }
    }
}

/// Generates the implementation of `ComObjectInner`.
pub(crate) fn gen_impl_com_object_inner(inputs: &ImplementInputs) -> syn::ItemImpl {
    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    parse_quote! {
        impl #generics ::windows_core::ComObjectInner for #original_ident::#generics where #constraints {
            type Outer = #impl_ident::#generics;
        }
    }
}

pub(crate) fn gen_impl_into_com_object(inputs: &ImplementInputs) -> syn::ItemImpl {
    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    if let Some(ref base) = inputs.base_class_info {
        let base_type = &base.field_ty;
        parse_quote! {
            impl #generics ::windows_core::IntoAggregatedComObject for #original_ident::#generics where #constraints {
                type Base = #base_type;

                // IMPORTANT! This function handles assembling the "boxed" type of a COM object.
                // It immediately moves the box into a heap allocation (box) and returns only a ComObject
                // reference that points to it. We intentionally _do not_ expose any owned instances of
                // Foo_Impl to safe Rust code, because doing so would allow unsound behavior in safe Rust
                // code, due to the adjustments of the reference count that Foo_Impl permits.
                //
                // This is why this function returns ComObject<Self> instead of returning #impl_ident.

                fn into_object(self, base: Self::Base) -> ::windows_core::ComObject<Self> {
                    let boxed = ::windows_core::imp::Box::<#impl_ident::#generics>::new(self.into_outer(base));
                    unsafe {
                        let ptr = ::windows_core::imp::Box::into_raw(boxed);
                        ::windows_core::ComObject::from_raw(
                            ::core::ptr::NonNull::new_unchecked(ptr)
                        )
                    }
                }
            }
        }
    } else {
        parse_quote! {
            impl #generics ::windows_core::IntoComObject for #original_ident::#generics where #constraints {
                // IMPORTANT! This function handles assembling the "boxed" type of a COM object.
                // It immediately moves the box into a heap allocation (box) and returns only a ComObject
                // reference that points to it. We intentionally _do not_ expose any owned instances of
                // Foo_Impl to safe Rust code, because doing so would allow unsound behavior in safe Rust
                // code, due to the adjustments of the reference count that Foo_Impl permits.
                //
                // This is why this function returns ComObject<Self> instead of returning #impl_ident.

                fn into_object(self) -> ::windows_core::ComObject<Self> {
                    let boxed = ::windows_core::imp::Box::<#impl_ident::#generics>::new(self.into_outer());
                    unsafe {
                        let ptr = ::windows_core::imp::Box::into_raw(boxed);
                        ::windows_core::ComObject::from_raw(
                            ::core::ptr::NonNull::new_unchecked(ptr)
                        )
                    }
                }
            }
        }
    }
}

pub(crate) fn gen_into_outer(inputs: &ImplementInputs) -> syn::TraitItemFn {
    // let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    // let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    // let identity_type = if let Some(first) = attributes.implement.first() {
    //     first.to_ident()
    // } else {
    //     quote! { ::windows_core::IInspectable }
    // };

    let identity_type = quote! { ::windows_core::IInspectable };

    let mut vtbl_initializers = quote!();

    if inputs.base_class_info.is_none() {
        vtbl_initializers.extend(quote! {
            identity: {
                const CHAIN_FIELD_BYTE_OFFSET: usize = ::core::mem::offset_of!(#impl_ident::#generics, identity);
                const CHAIN_FIELD_POINTER_OFFSET: isize = {
                    if CHAIN_FIELD_BYTE_OFFSET % ::core::mem::size_of::<*const u8>() != 0 {
                        panic!("Interface chain offset is not properly aligned");
                    }
                    -((CHAIN_FIELD_BYTE_OFFSET / ::core::mem::size_of::<*const u8>()) as isize)
                };

                static VTABLE: ::windows_core::IInspectable_Vtbl =
                    ::windows_core::IInspectable_Vtbl::new::<#impl_ident, #identity_type, 0>();
                &VTABLE
            },
        });
    }

    for interface_chain in inputs.interface_chains.iter() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let vtbl_field_ident = &interface_chain.field_ident;
        let chain_ident = &interface_chain.field_ident;

        vtbl_initializers.extend(quote! {
            #vtbl_field_ident: {
                const CHAIN_FIELD_BYTE_OFFSET: usize = ::core::mem::offset_of!(#impl_ident::#generics, #chain_ident);
                const CHAIN_FIELD_POINTER_OFFSET: isize = {
                    if CHAIN_FIELD_BYTE_OFFSET % ::core::mem::size_of::<*const u8>() != 0 {
                        panic!("Interface chain offset is not properly aligned");
                    }
                    -((CHAIN_FIELD_BYTE_OFFSET / ::core::mem::size_of::<*const u8>()) as isize)
                };

                static VTABLE: #vtbl_ty = #vtbl_ty::new::<#impl_ident, CHAIN_FIELD_POINTER_OFFSET>();
                &VTABLE
            },
        });
    }

    if let Some(ref base) = inputs.base_class_info {
        let base_type = &base.field_ty;
        parse_quote! {
            // This constructs an "outer" object. This should only be used by the implementation
            // of the outer object, never by application code.
            //
            // The callers of this function (`into_static` and `into_object`) are both responsible
            // for maintaining one of our invariants: Application code never has an owned instance
            // of the outer (implementation) type. into_static() maintains this invariant by
            // returning a wrapped StaticComObject value, which owns its contents but never gives
            // application code a way to mutably access its contents. This prevents the refcount
            // shearing problem.
            //
            // TODO: Make it impossible for app code to call this function, by placing it in a
            // module and marking this as private to the module.
            #[inline(always)]
            const fn into_outer(self, base: #base_type) -> #impl_ident::#generics {
                #impl_ident::#generics {
                    #vtbl_initializers
                    this: self,
                    base,
                }
            }
        }
    } else {
        parse_quote! {
            // This constructs an "outer" object. This should only be used by the implementation
            // of the outer object, never by application code.
            //
            // The callers of this function (`into_static` and `into_object`) are both responsible
            // for maintaining one of our invariants: Application code never has an owned instance
            // of the outer (implementation) type. into_static() maintains this invariant by
            // returning a wrapped StaticComObject value, which owns its contents but never gives
            // application code a way to mutably access its contents. This prevents the refcount
            // shearing problem.
            //
            // TODO: Make it impossible for app code to call this function, by placing it in a
            // module and marking this as private to the module.
            #[inline(always)]
            const fn into_outer(self) -> #impl_ident::#generics {
                #impl_ident::#generics {
                    #vtbl_initializers
                    this: self,
                    count: ::windows_core::imp::WeakRefCount::new(),
                }
            }
        }
    }
}
