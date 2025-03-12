fn check_remote_generic(cx: &Ctxt, cont: &Container) {
    if let Some(remote) = cont.attrs.remote() {
        let local_has_generic = !cont.generics.params.is_empty();
        let remote_has_generic = !remote.segments.last().unwrap().arguments.is_none();
        if local_has_generic && remote_has_generic {
            cx.error_spanned_by(remote, "remove generic parameters from this path");
        }
    }
}