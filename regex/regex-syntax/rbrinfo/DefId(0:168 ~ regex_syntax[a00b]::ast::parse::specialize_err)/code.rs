fn specialize_err<T>(
    result: Result<T>,
    from: ast::ErrorKind,
    to: ast::ErrorKind,
) -> Result<T> {
    if let Err(e) = result {
        if e.kind == from {
            Err(ast::Error { kind: to, pattern: e.pattern, span: e.span })
        } else {
            Err(e)
        }
    } else {
        result
    }
}