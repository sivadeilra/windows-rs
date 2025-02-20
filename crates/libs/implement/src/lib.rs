//! Implement COM interfaces for Rust types.
//!
//! Take a look at [macro@implement] for an example.
//!
//! Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>

use quote::{quote, ToTokens};
use syn::parse_quote;

mod gen;

#[cfg(test)]
mod tests;

/// Implements one or more COM interfaces.
///
/// # Example
/// ```rust,no_run
/// use windows_core::*;
///
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value_Impl {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// let object: IValue = ComObject::new(Value(123)).into_interface();
/// // Call interface methods...
/// ```
#[proc_macro_attribute]
pub fn implement(
    attributes: proc_macro::TokenStream,
    type_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    implement_core(attributes.into(), type_tokens.into()).into()
}

struct ImplementInputs {
    interface_chains: Vec<InterfaceChain>,
    base_class_info: Option<BaseClassInfo>,
    original_type: syn::ItemStruct,

    trust_level: usize,

    /// The identifier of the `Foo_Impl` type.
    impl_ident: syn::Ident,

    /// The list of constraints needed for this `Foo_Impl` type.
    constraints: proc_macro2::TokenStream,

    /// The list of generic parameters for this `Foo_Impl` type, including `<` and `>`.
    /// If there are no generics, this contains `<>`.
    generics: proc_macro2::TokenStream,
}

struct InterfaceChain {
    field_ident: syn::Ident,
    implement: ImplementType,
}

struct BaseClassInfo {
    field_ident: syn::Ident,
    /// A type specifier for the `Base_Impl` type.
    field_ty: syn::Type,
}

/// Check the user type definition for a field marked with `#[base]`. If we find one, then
/// _remove_ the attribute from the field and return it.
///
/// If a field is marked with `#[base]`, then the type of that field must specify an "outer"
/// implementation type, e.g. `Base_Impl`, _not_ a `Base` type. This function cannot verify that;
/// instead, we generate code and rely on the compiler to verify our requirements.
fn parse_base_class_info(
    original_type: &mut syn::ItemStruct,
) -> Result<Option<BaseClassInfo>, syn::Error> {
    // Declaring a base class requires using named field syntax.
    // We ignore struct definitions that don't use named field syntax (i.e. types and unit).
    let syn::Fields::Named(ref mut named) = original_type.fields else {
        return Ok(None);
    };

    let mut base_class_info: Option<BaseClassInfo> = None;

    let mut new_fields = syn::punctuated::Punctuated::new();

    for field_pair in core::mem::take(&mut named.named).into_pairs() {
        let (mut field, field_punct) = field_pair.into_tuple();

        let field_ident: syn::Ident = field.ident.clone().unwrap();
        if let Some(base_attr_index) = field.attrs.iter().position(|a| a.path().is_ident("base")) {
            if base_class_info.is_some() {
                return Err(syn::Error::new(
                    field_ident.span(),
                    "cannot declare more than one field with #[base]",
                ));
            } else {
                // Remove the attribute, so that it is not emitted later.
                field.attrs.remove(base_attr_index);
                base_class_info = Some(BaseClassInfo {
                    field_ident,
                    field_ty: field.ty,
                });
            }
        } else {
            // Normal field.
            new_fields.push_value(field);
            if let Some(p) = field_punct {
                new_fields.push_punct(p);
            }
        }
    }

    named.named = new_fields;
    Ok(base_class_info)
}

fn implement_core(
    attributes: proc_macro2::TokenStream,
    type_tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let attributes = syn::parse2::<ImplementAttributes>(attributes).unwrap();
    // let interfaces_len = proc_macro2::Literal::usize_unsuffixed(attributes.implement.len());

    let mut original_type = syn::parse2::<syn::ItemStruct>(type_tokens).unwrap();
    let base_class_info = parse_base_class_info(&mut original_type).unwrap();

    let original_ident = &original_type.ident.clone();
    let mut constraints = quote! {};

    if let Some(where_clause) = &original_type.generics.where_clause {
        where_clause.predicates.to_tokens(&mut constraints);
    }

    let generics = if original_type.generics.lt_token.is_some() {
        let mut params = quote! {};
        original_type.generics.params.to_tokens(&mut params);
        quote! { <#params> }
    } else {
        quote! { <> }
    };

    // The identifier of the Foo_Impl struct for this type.
    let impl_ident = quote::format_ident!("{}_Impl", original_ident);

    let inputs = ImplementInputs {
        interface_chains: convert_implements_to_interface_chains(attributes.implement),
        trust_level: attributes.trust_level,
        base_class_info,
        original_type,
        constraints,
        generics,
        impl_ident,
    };
    // let impl_ident = &inputs.impl_ident;

    let generics = &inputs.generics;
    let impl_ident = &inputs.impl_ident;
    let constraints = &inputs.constraints;
    let conversions = inputs.interface_chains.iter().enumerate().map(|(enumerate, interface_chain)| {
        let interface_ident = interface_chain.implement.to_ident();
        let chain_field = &interface_chain.field_ident;
        let offset = proc_macro2::Literal::usize_unsuffixed(enumerate);
        _ = offset;

        quote! {
            // TODO: make conditional on non-aggregate
            /*
            impl #generics ::core::convert::From<#original_ident::#generics> for #interface_ident where #constraints {
                #[inline(always)]
                fn from(this: #original_ident::#generics) -> Self {
                    let com_object = ::windows_core::ComObject::new(this);
                    com_object.into_interface()
                }
            }
            */

            impl #generics ::windows_core::ComObjectInterface<#interface_ident> for #impl_ident::#generics where #constraints {
                #[inline(always)]
                fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, #interface_ident> {
                    unsafe {
                        ::core::mem::transmute(&self.#chain_field)
                    }
                }
            }

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
    });

    // Contains tokens that will be added to the impl block for the generated Foo_Impl type.
    let mut impl_items = quote!();

    if let Some(ref base_class_info) = inputs.base_class_info {
        // Emit a method which allows the app to go from &Derived_Impl to &Base_Impl.
        let base_field_ident = &base_class_info.field_ident;
        let base_ty = &base_class_info.field_ty;
        impl_items.extend(quote! {
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

    if inputs.base_class_info.is_none() {
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
            #impl_items
        }

        impl #generics #original_ident::#generics where #constraints {

            #original_impl_items
        }

        /*
        impl #generics #original_ident::#generics where #constraints {
            /// Try casting as the provided interface
            ///
            /// # Safety
            ///
            /// This function can only be safely called if `self` has been heap allocated and pinned using
            /// the mechanisms provided by `implement` macro.
            #[inline(always)]
            unsafe fn cast<I: ::windows_core::Interface>(&self) -> ::windows_core::Result<I> {
                let boxed = (self as *const _ as *const *mut ::core::ffi::c_void).sub(1 + #interfaces_len) as *mut #impl_ident::#generics;
                let mut result = ::core::ptr::null_mut();
                _ = <#impl_ident::#generics as ::windows_core::IUnknownImpl>::query_interface(&*boxed, &I::IID, &mut result);
                ::windows_core::Type::from_abi(result)
            }
        }
        */

        // TODO: make conditional on non-aggregated
        /*
        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IInspectable where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }
        */

        impl #generics ::windows_core::ComObjectInterface<::windows_core::IUnknown> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
                unsafe {
                    let interface_ptr: &&'static ::windows_core::IInspectable_Vtbl = self.identity_interface();
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }

        impl #generics ::windows_core::ComObjectInterface<::windows_core::IInspectable> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IInspectable> {
                unsafe {
                    let interface_ptr: &&'static ::windows_core::IInspectable_Vtbl = self.identity_interface();
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }

        impl #generics ::windows_core::AsImpl<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
            // is guaranteed to live at least as long as `self`.
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics> {
                // TODO: This is super sus
                todo!()
                /*
                unsafe {
                    let this = ::windows_core::Interface::as_raw(self);
                    // Subtract away the vtable offset plus 1, for the `identity` field, to get
                    // to the impl struct which contains that original implementation type.
                    let this = (this as *mut *mut ::core::ffi::c_void).sub(1) as *mut #impl_ident::#generics;
                    ::core::ptr::NonNull::new_unchecked(::core::ptr::addr_of!((*this).this) as *const #original_ident::#generics as *mut #original_ident::#generics)
                }
                */
            }
        }

        impl #generics ::core::ops::Deref for #impl_ident::#generics where #constraints {
            type Target = #original_ident::#generics;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.this
            }
        }

        // We intentionally do not provide a DerefMut impl, due to paranoia around soundness.

        #(#conversions)*
    };

    tokens.extend(gen::gen_impl_struct(&inputs).into_token_stream());
    tokens.extend(gen::gen_iunknown_impl(&inputs).into_token_stream());
    tokens.extend(gen::gen_impl_com_object_inner(&inputs).into_token_stream());
    tokens.extend(gen::gen_impl_into_com_object(&inputs).into_token_stream());

    let mut tokens: proc_macro2::TokenStream = tokens.into();
    tokens.extend(proc_macro2::TokenStream::from(
        inputs.original_type.into_token_stream(),
    ));
    tokens
}

fn parse_verify<T: syn::parse::Parse>(t: proc_macro2::TokenStream) -> T {
    match syn::parse2(t.clone()) {
        Ok(x) => x,
        Err(e) => {
            panic!("Failed to parse token stream into expected AST: {e:?}\nTokens:\n{t}");
        }
    }
}

struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,
    span: proc_macro2::Span,
}

impl ImplementType {
    fn to_ident(&self) -> proc_macro2::TokenStream {
        let type_name = syn::parse_str::<proc_macro2::TokenStream>(&self.type_name)
            .expect("Invalid token stream");
        let generics = self.generics.iter().map(|g| g.to_ident());
        quote! { #type_name<#(#generics,)*> }
    }
    fn to_vtbl_ident(&self) -> proc_macro2::TokenStream {
        let ident = self.to_ident();
        quote! {
            <#ident as ::windows_core::Interface>::Vtable
        }
    }
}

#[derive(Default)]
struct ImplementAttributes {
    pub implement: Vec<ImplementType>,
    pub trust_level: usize,
}

impl syn::parse::Parse for ImplementAttributes {
    fn parse(cursor: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let mut input = Self::default();

        while !cursor.is_empty() {
            input.parse_implement(cursor)?;
        }

        Ok(input)
    }
}

impl ImplementAttributes {
    fn parse_implement(&mut self, cursor: syn::parse::ParseStream<'_>) -> syn::parse::Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(&tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<syn::Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(
        &mut self,
        tree: &UseTree2,
        namespace: &mut String,
    ) -> syn::parse::Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(&input.tree, namespace)?;
            }
            UseTree2::Name(_) => {
                self.implement.push(tree.to_element_type(namespace)?);
            }
            UseTree2::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(tree, namespace)?;
                }
            }
            UseTree2::TrustLevel(input) => self.trust_level = *input,
        }

        Ok(())
    }
}

enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
    TrustLevel(usize),
}

impl UseTree2 {
    fn to_element_type(&self, namespace: &mut String) -> syn::parse::Result<ImplementType> {
        match self {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(namespace)
            }
            UseTree2::Name(input) => {
                let mut type_name = input.ident.to_string();

                if !namespace.is_empty() {
                    type_name = format!("{namespace}::{type_name}");
                }

                let mut generics = vec![];

                for g in &input.generics {
                    generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(ImplementType {
                    type_name,
                    span: input.ident.span(),
                    generics,
                })
            }
            UseTree2::Group(input) => Err(syn::parse::Error::new(
                input.brace_token.span.join(),
                "Syntax not supported",
            )),
            _ => unimplemented!(),
        }
    }
}

struct UsePath2 {
    pub ident: syn::Ident,
    pub tree: Box<UseTree2>,
}

struct UseName2 {
    pub ident: syn::Ident,
    pub generics: Vec<UseTree2>,
}

struct UseGroup2 {
    pub brace_token: syn::token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, syn::Token![,]>,
}

impl syn::parse::Parse for UseTree2 {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(syn::Ident::parse_any)?;
            if input.peek(syn::Token![::]) {
                input.parse::<syn::Token![::]>()?;
                Ok(UseTree2::Path(UsePath2 {
                    ident,
                    tree: Box::new(input.parse()?),
                }))
            } else if input.peek(syn::Token![=]) {
                if ident != "TrustLevel" {
                    return Err(syn::parse::Error::new(
                        ident.span(),
                        "Unrecognized key-value pair",
                    ));
                }
                input.parse::<syn::Token![=]>()?;
                let span = input.span();
                let value = input.call(syn::Ident::parse_any)?;
                match value.to_string().as_str() {
                    "Partial" => Ok(UseTree2::TrustLevel(1)),
                    "Full" => Ok(UseTree2::TrustLevel(2)),
                    _ => Err(syn::parse::Error::new(
                        span,
                        "`TrustLevel` must be `Partial` or `Full`",
                    )),
                }
            } else {
                let generics = if input.peek(syn::Token![<]) {
                    input.parse::<syn::Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<syn::Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<syn::Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            let brace_token = syn::braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse, syn::Token![,])?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}

fn convert_implements_to_interface_chains(implements: Vec<ImplementType>) -> Vec<InterfaceChain> {
    let mut chains = Vec::with_capacity(implements.len());

    for (i, implement) in implements.into_iter().enumerate() {
        // Create an identifier for this interface chain.
        // We only use this for naming fields; it is never visible to the developer.
        // This helps with debugging.

        let mut ident_string = format!("interface{i}");

        let suffix = get_interface_ident_suffix(&implement.type_name);
        if !suffix.is_empty() {
            ident_string.push('_');
            ident_string.push_str(&suffix);
        }
        let field_ident = syn::Ident::new(&ident_string, implement.span);

        chains.push(InterfaceChain {
            implement,
            field_ident,
        });
    }

    chains
}

fn get_interface_ident_suffix(type_name: &str) -> String {
    let mut suffix = String::new();
    for c in type_name.chars() {
        let c = c.to_ascii_lowercase();

        if suffix.len() >= 20 {
            break;
        }

        if c.is_ascii_alphanumeric() {
            suffix.push(c);
        }
    }

    suffix
}
