fn doc_comment<'a>(input: Cursor<'a>, trees: &mut TokenStreamBuilder) -> PResult<'a, ()> {
    #[cfg(span_locations)]
    let lo = input.off;
    let (rest, (comment, inner)) = doc_comment_contents(input)?;
    let fallback_span = Span {
        #[cfg(span_locations)]
        lo,
        #[cfg(span_locations)]
        hi: rest.off,
    };
    let span = crate::Span::_new_fallback(fallback_span);

    let mut scan_for_bare_cr = comment;
    while let Some(cr) = scan_for_bare_cr.find('\r') {
        let rest = &scan_for_bare_cr[cr + 1..];
        if !rest.starts_with('\n') {
            return Err(Reject);
        }
        scan_for_bare_cr = rest;
    }

    let mut pound = Punct::new('#', Spacing::Alone);
    pound.set_span(span);
    trees.push_token_from_parser(TokenTree::Punct(pound));

    if inner {
        let mut bang = Punct::new('!', Spacing::Alone);
        bang.set_span(span);
        trees.push_token_from_parser(TokenTree::Punct(bang));
    }

    let doc_ident = crate::Ident::_new_fallback(Ident::new_unchecked("doc", fallback_span));
    let mut equal = Punct::new('=', Spacing::Alone);
    equal.set_span(span);
    let mut literal = crate::Literal::_new_fallback(Literal::string(comment));
    literal.set_span(span);
    let mut bracketed = TokenStreamBuilder::with_capacity(3);
    bracketed.push_token_from_parser(TokenTree::Ident(doc_ident));
    bracketed.push_token_from_parser(TokenTree::Punct(equal));
    bracketed.push_token_from_parser(TokenTree::Literal(literal));
    let group = Group::new(Delimiter::Bracket, bracketed.build());
    let mut group = crate::Group::_new_fallback(group);
    group.set_span(span);
    trees.push_token_from_parser(TokenTree::Group(group));

    Ok((rest, ()))
}