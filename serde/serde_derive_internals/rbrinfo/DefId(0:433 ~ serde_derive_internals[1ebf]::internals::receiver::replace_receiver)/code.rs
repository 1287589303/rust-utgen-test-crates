pub fn replace_receiver(input: &mut DeriveInput) {
    let self_ty = {
        let ident = &input.ident;
        let ty_generics = input.generics.split_for_impl().1;
        parse_quote!(#ident #ty_generics)
    };
    let mut visitor = ReplaceReceiver(&self_ty);
    visitor.visit_generics_mut(&mut input.generics);
    visitor.visit_data_mut(&mut input.data);
}