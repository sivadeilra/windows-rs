use super::*;

pub(crate) fn gen_all(inputs: &ImplementInputs) -> proc_macro2::TokenStream {
    let generics = &inputs.generics;
    let impl_ident = &inputs.impl_ident;
    let constraints = &inputs.constraints;
    let original_ident = &inputs.original_type.ident;

    // Contains tokens that will be added to the impl block for the generated Foo_Impl type.
    let mut impl_items: Vec<syn::ImplItem> = Vec::new();

    impl_items.extend(gen_vtable_consts(inputs));

    if let Some(ref base_class_info) = inputs.base_class_info {
        // Emit a method which allows the app to go from &Derived_Impl to &Base_Impl.
        let base_field_ident = &base_class_info.field_ident;
        let base_ty = &base_class_info.field_ty;
        impl_items.push(parse_quote! {
            /// Provides access to the base object.
            // TODO: Handle generics for the base type.
            pub fn base(&self) -> &#base_ty {
                &self.#base_field_ident
            }
            // Do NOT provide a method for returning &mut #base_ty. Doing so would allow for
            // breaking memory safety, because it would allow any caller to swap instances of
            // the Foo_Impl. This may be fixable by moving the reference count field out of the
            // _Impl types.
            //
            // It might be safe to return Pin<&mut Foo_Impl>.
            // It would be safe to directly return &mut Foo (not &mut Foo_Impl!)
        });
    }

    let mut original_impl_items = quote!();
    original_impl_items.extend(gen::gen_into_outer(&inputs).into_token_stream());

    // Static COM objects have a lot of constraints. They can't be generic (open parameters),
    // because that would be meaningless (an open generic type cannot have a known representation).
    //
    // Right now, we can't generate static COM objects that have base classes because we rely on
    // boxing and then unboxing during construction of aggregated types.
    if inputs.base_class_info.is_none() && !inputs.is_generic {
        original_impl_items.extend(quote! {
            /// This converts a partially-constructed COM object (in the sense that it contains
            /// application state but does not yet have vtable and reference count constructed)
            /// into a `StaticComObject`. This allows the COM object to be stored in static
            /// (global) variables.
            pub const fn into_static(self) -> ::windows_core::StaticComObject<Self> {
                ::windows_core::StaticComObject::from_outer(self.into_outer())
            }
        });
    }

    let mut tokens = quote! {
        impl #generics #impl_ident::#generics where #constraints {
            #(#impl_items)*
        }

        impl #generics #original_ident::#generics where #constraints {
            #original_impl_items
        }

        impl #generics ::core::ops::Deref for #impl_ident::#generics where #constraints {
            type Target = #original_ident::#generics;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.this
            }
        }

        // We intentionally do not provide a DerefMut impl, due to paranoia around soundness.
    };

    for item in gen::gen_impl_com_object_interfaces(&inputs) {
        tokens.extend(item.into_token_stream());
    }

    tokens.extend(gen::gen_impl_struct(&inputs).into_token_stream());
    tokens.extend(gen::gen_iunknown_impl(&inputs).into_token_stream());
    tokens.extend(gen::gen_impl_com_object_inner(&inputs).into_token_stream());
    tokens.extend(gen::gen_impl_into_com_object(&inputs).into_token_stream());

    for item in gen::gen_conversions(&inputs) {
        tokens.extend(item.into_token_stream());
    }

    for interface_chain in inputs.interface_chains.iter() {
        tokens.extend(gen::gen_impl_as_impl(&inputs, interface_chain).into_token_stream());
    }

    tokens
}

// Generates the structure definition for the `Foo_Impl` type.
fn gen_impl_struct(inputs: &ImplementInputs) -> syn::ItemStruct {
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
            header: ::windows_core::ComObjectHeader,
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
fn gen_iunknown_impl(inputs: &ImplementInputs) -> syn::ItemImpl {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;
    let original_ident = &inputs.original_type.ident;

    let query_interface_fn = gen_query_interface(inputs);

    let trust_level = proc_macro2::Literal::usize_unsuffixed(inputs.trust_level);

    let header_fn: syn::ImplItemFn = if let Some(_) = inputs.base_class_info {
        parse_quote! {
            #[inline(always)]
            fn header(&self) -> &::windows_core::ComObjectHeader {
                self.base().header()
            }
        }
    } else {
        parse_quote! {
            #[inline(always)]
            fn header(&self) -> &::windows_core::ComObjectHeader {
                &self.header
            }
        }
    };

    let identity_interface_fn: syn::ImplItemFn = if let Some(ref base) = inputs.base_class_info {
        let base_field = &base.field_ident;
        parse_quote! {
            fn identity_interface(&self) -> &&'static ::windows_core::IInspectable_Vtbl {
                self.#base_field.identity_interface()
            }
        }
    } else {
        parse_quote! {
            fn identity_interface(&self) -> &&'static ::windows_core::IInspectable_Vtbl {
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
            #header_fn

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
        unsafe fn query_interface_this(
            &self,
            iid: *const ::windows_core::GUID,
            interface: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            unsafe {
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
}

/// Generates the implementation of `ComObjectInner`.
fn gen_impl_com_object_inner(inputs: &ImplementInputs) -> syn::ItemImpl {
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

fn gen_impl_into_com_object(inputs: &ImplementInputs) -> syn::ItemImpl {
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

/// Generates a set of associated constants that describe the vtables for this type.
///
/// e.g. `const VTABLE_INTERFACE4_IFOO: IFoo_Vtbl = ...`
fn gen_vtable_consts(inputs: &ImplementInputs) -> Vec<syn::ImplItem> {
    let mut items = Vec::new();

    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;

    // let identity_type = if let Some(first) = attributes.implement.first() {
    //     first.to_ident()
    // } else {
    //     quote! { ::windows_core::IInspectable }
    // };
    let identity_type = quote! { ::windows_core::IInspectable };

    if inputs.base_class_info.is_none() {
        items.push(parse_quote! {
            const VTABLE_IDENTITY: ::windows_core::IInspectable_Vtbl =
                ::windows_core::IInspectable_Vtbl::new::<
                    #impl_ident::#generics,
                    #identity_type,
                    0, // #chain_offset_expression,
                >();
        });
    }

    for (interface_index, interface_chain) in inputs.interface_chains.iter().enumerate() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let chain_ident = &interface_chain.field_ident;
        let vtable_const_ident = &interface_chain.vtable_const_ident;

        let chain_offset_expression = if inputs.is_generic {
            let chain_offset_in_pointers: isize = -1 - interface_index as isize;
            quote!(#chain_offset_in_pointers)
        } else {
            quote! {
                // The nested { ... } scope is necessary; do not remove it.
                {
                    -((::core::mem::offset_of!(
                        // <Self as ::windows_core::ComObjectInner>::Outer,
                        #impl_ident::#generics,
                        #chain_ident) / ::core::mem::size_of::<*const u8>()) as isize)
                }
            }
        };

        items.push(parse_quote! {
            const #vtable_const_ident: #vtbl_ty = #vtbl_ty::new::<
                #impl_ident::#generics,
                #chain_offset_expression,
            >();
        });
    }

    items
}

fn gen_into_outer(inputs: &ImplementInputs) -> syn::TraitItem {
    // let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let impl_ident = &inputs.impl_ident;

    let mut vtbl_initializers = quote!();

    if inputs.base_class_info.is_none() {
        vtbl_initializers.extend(quote! {
            identity: &#impl_ident::#generics::VTABLE_IDENTITY,
        });
    }

    for interface_chain in inputs.interface_chains.iter() {
        let vtbl_field_ident = &interface_chain.field_ident;
        let vtable_const_ident = &interface_chain.vtable_const_ident;

        vtbl_initializers.extend(quote! {
            #vtbl_field_ident: &#impl_ident::#generics::#vtable_const_ident,
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
        // If the type is generic then into_outer() cannot be a const fn.
        let maybe_const = if inputs.is_generic {
            quote!()
        } else {
            quote!(const)
        };

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
            #maybe_const fn into_outer(self) -> #impl_ident::#generics {


                #impl_ident::#generics {
                    header: ::windows_core::ComObjectHeader {
                        count: ::windows_core::imp::WeakRefCount::new(),
                        destructor: |this: *mut ::core::ffi::c_void| {
                            unsafe {
                                let self_ = this as *mut #impl_ident::#generics;
                                _ = ::windows_core::imp::Box::from_raw(self_);
                            }
                        },
                        query_interface: |
                            this: *const ::core::ffi::c_void,
                            iid: *const ::windows_core::GUID,
                            interface: *mut *mut ::core::ffi::c_void,
                        | -> ::windows_core::HRESULT {
                            unsafe {
                                let self_ = &*(this as *mut #impl_ident::#generics);
                                ::windows_core::IUnknownImpl::query_interface_this(self_, iid, interface)
                            }
                        }
                    },
                    #vtbl_initializers
                    this: self,
                }
            }
        }
    }
}

/// Generates `From`-based conversions
fn gen_conversions(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items = Vec::new();

    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;

    // These conversions only work for non-aggregated types.
    if inputs.base_class_info.is_none() {
        items.push(parse_quote! {
            impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
                #[inline(always)]
                fn from(this: #original_ident::#generics) -> Self {
                    let com_object = ::windows_core::ComObject::new(this);
                    com_object.into_interface()
                }
            }
        });

        items.push(parse_quote! {
            impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IInspectable where #constraints {
                #[inline(always)]
                fn from(this: #original_ident::#generics) -> Self {
                    let com_object = ::windows_core::ComObject::new(this);
                    com_object.into_interface()
                }
            }
        });

        for interface_chain in inputs.interface_chains.iter() {
            let interface_ident = interface_chain.implement.to_ident();

            items.push(parse_quote! {
                impl #generics ::core::convert::From<#original_ident::#generics> for #interface_ident where #constraints {
                    #[inline(always)]
                    fn from(this: #original_ident::#generics) -> Self {
                        let com_object = ::windows_core::ComObject::new(this);
                        com_object.into_interface()
                    }
                }
            });
        }
    }

    items
}

/// Generates the `ComObjectInterface` implementation for each interface chain
fn gen_impl_com_object_interfaces(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items = Vec::new();

    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    items.push(parse_quote! {
        impl #generics ::windows_core::ComObjectInterface<::windows_core::IUnknown> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
                unsafe {
                    let interface_ptr: &&'static ::windows_core::IInspectable_Vtbl =
                        ::windows_core::IUnknownImpl::identity_interface(self);
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }
    });

    items.push(parse_quote! {
        impl #generics ::windows_core::ComObjectInterface<::windows_core::IInspectable> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IInspectable> {
                unsafe {
                    let interface_ptr: &&'static ::windows_core::IInspectable_Vtbl =
                        ::windows_core::IUnknownImpl::identity_interface(self);
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }
    });

    for interface_chain in inputs.interface_chains.iter() {
        let chain_field = &interface_chain.field_ident;
        let interface_ident = interface_chain.implement.to_ident();

        items.push(parse_quote! {
            impl #generics ::windows_core::ComObjectInterface<#interface_ident> for #impl_ident::#generics where #constraints {
                #[inline(always)]
                fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, #interface_ident> {
                    unsafe {
                        ::core::mem::transmute(&self.#chain_field)
                    }
                }
            }
        });
    }

    items
}

fn gen_impl_as_impl(inputs: &ImplementInputs, interface_chain: &InterfaceChain) -> syn::Item {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let interface_ident = interface_chain.implement.to_ident();
    let original_ident = &inputs.original_type.ident;

    parse_quote! {
        impl #generics ::windows_core::AsImpl<#original_ident::#generics> for #interface_ident where #constraints {
            // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
            // is guaranteed to live at least as long as `self`.
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics> {
                // TODO: This is super sus
                todo!()
                /*
                let this = ::windows_core::Interface::as_raw(self);
                // Subtract away the vtable offset plus 1, for the `identity` field, to get
                // to the impl struct which contains that original implementation type.
                let this = (this as *mut *mut ::core::ffi::c_void).sub(1 + #offset) as *mut #impl_ident::#generics;
                ::core::ptr::NonNull::new_unchecked(::core::ptr::addr_of!((*this).this) as *const #original_ident::#generics as *mut #original_ident::#generics)
                */
            }
        }
    }
}
