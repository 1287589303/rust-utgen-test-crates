fn string<T: Target>(target: &mut Option<T>) -> &mut String {
    target
        .as_mut()
        .expect("url::form_urlencoded::Serializer finished")
        .as_mut_string()
}