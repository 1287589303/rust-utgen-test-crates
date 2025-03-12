fn borrowable_lifetimes(
    cx: &Ctxt,
    name: &Name,
    field: &syn::Field,
) -> Result<BTreeSet<syn::Lifetime>, ()> {
    let mut lifetimes = BTreeSet::new();
    collect_lifetimes(&field.ty, &mut lifetimes);
    if lifetimes.is_empty() {
        let msg = format!("field `{}` has no lifetimes to borrow", name);
        cx.error_spanned_by(field, msg);
        Err(())
    } else {
        Ok(lifetimes)
    }
}