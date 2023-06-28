

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_attribute]
pub fn subsystem_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // throw error if input is not an impl
    let implementation = syn::parse_macro_input!(input as syn::ItemImpl);

    // get the name of the struct being implemented
    let struct_name = match *implementation.self_ty {
        syn::Type::Path(ref path) => path.path.segments.last().unwrap().ident.clone(),
        _ => panic!("expected a struct"),
    };

    //get the struct name in caps as an identifier
    let struct_name_caps = syn::Ident::new(
        &format!("__{}", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    // go through all funcs, if none are decorated with `#[new]` then throw an error
    let mut new_func = None;
    let mut other_funcs = Vec::new();
    for item in implementation.items {
        if let syn::ImplItem::Fn(method) = item {
            let mut attrs = method.attrs.iter().clone();
            if attrs.len() > 1 {
                panic!("expected only one attribute per function");
            }
            if attrs.clone().any(|attr| attr.path().is_ident("ignore")) {
                continue;
            }
            if attrs.any(|attr| attr.path().is_ident("new")) {
                if new_func.is_some() {
                    panic!("expected only one function decorated with `#[new]`");
                }
                new_func = Some(method);
            } else {
                other_funcs.push(method);
            }
        }
    };
    if new_func.is_none() {
        panic!("expected a function decorated with `#[new]`");
    };

    // get the new function and rewrite it as private with name `__new`
    let mut new_func = new_func.unwrap();
    new_func.sig.ident = syn::Ident::new("__new", new_func.sig.ident.span());
    new_func.vis = syn::Visibility::Inherited;
    new_func.attrs = Vec::new();

    //put the __new function in an impl block
    let mut impl_block = Vec::new();

    impl_block.push(new_func);

    //for each func in the impl block, make the non static version private and make a public static version
    for item_fn in &mut other_funcs {

        let static_ident = syn::Ident::new(
            &format!("{}", item_fn.sig.ident),
            item_fn.sig.ident.span(),
        );

        //make the non static version private and rename it to __<name>
        item_fn.vis = syn::Visibility::Inherited;
        item_fn.sig.ident = syn::Ident::new(
            &format!("__{}", item_fn.sig.ident),
            item_fn.sig.ident.span(),
        );
        impl_block.push(item_fn.clone());



        // get all input idents 
        let mut input_idents = Vec::new();
        let mut input_types = Vec::new();
        for arg in &item_fn.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    input_idents.push(pat_ident.ident.clone());
                }
                input_types.push(pat_type.ty.clone());
            }
        }
        let non_static_ident = item_fn.sig.ident.clone();
        let non_static_output = item_fn.sig.output.clone();

        let static_fn_item = syn::parse_quote! {
            pub fn #static_ident(#(#input_idents: #input_types),*) #non_static_output {
                let mut this = #struct_name_caps.lock();
                this.#non_static_ident(#(#input_idents),*)
            }
        };

        impl_block.push(static_fn_item);
    };

    let output_stream = quote! {
        impl #struct_name {
            #(#impl_block)*
        }
    };

    // panic!("{}", output_stream.to_string());

    output_stream.into()
}


/// Automatically sets up some boilerplate needed for static subsystems.
/// Expects Subsystem name and UUID(u8) as arguments.
/// Example: subsystem!(TestSubsystem, 1u8)
#[proc_macro]
pub fn subsystem(input: TokenStream) -> TokenStream {
    //get an ident and a literal int from the token stream
    //filter out puncts and commas
    let mut iter = TokenStream2::from(input).into_iter().filter(
        |token| !matches!(token, proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Group(_)),
    );
    let struct_name = syn::parse2::<syn::Ident>(iter.next().expect("could not find first ident").into())
        .expect("could not parse first ident as an ident");
    let literal = syn::parse2::<syn::LitInt>(iter.next().expect("could not find second literal").into())
        .expect("could not parse second literal as an int");

    //get the struct name in caps as an identifier
    let struct_name_caps = syn::Ident::new(
        &format!("__{}", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    let mut output = TokenStream2::new(); 

    // create a static variable for the struct
    let static_variable = quote! {
        static #struct_name_caps: once_cell::sync::Lazy<parking_lot::Mutex<#struct_name>> = once_cell::sync::Lazy::new(|| parking_lot::Mutex::new(#struct_name::__new()));
        static UUID: u8 = #literal;
    };
    output.extend(static_variable);

    //add a static fn to get a &mut self from static variable mutex
    let static_getter = quote!(
        impl #struct_name {
            pub fn get_static() -> parking_lot::MutexGuard<'static, #struct_name> {
                let mut this = #struct_name_caps.lock();
                this
            }
            pub fn uuid() -> u8 {
                UUID as u8
            }
            pub fn name() -> &'static str {
                stringify!(#struct_name)
            }
        }
    );
    output.extend(static_getter);

    output.into()
}