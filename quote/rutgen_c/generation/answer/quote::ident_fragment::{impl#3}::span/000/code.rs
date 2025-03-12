// Answer 0

#[test]
fn test_cow_span_none() {
    struct Dummy;

    impl IdentFragment for Dummy {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    impl ToOwned for Dummy {
        type Owned = Dummy;

        fn to_owned(&self) -> Self::Owned {
            Dummy
        }
    }

    let cow: Cow<'_, Dummy> = Cow::Borrowed(&Dummy);
    assert_eq!(cow.span(), None);
}

#[test]
fn test_cow_span_some() {
    struct DummyWithSpan;

    impl IdentFragment for DummyWithSpan {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn span(&self) -> Option<Span> {
            Some(Span::call_site())
        }
    }

    impl ToOwned for DummyWithSpan {
        type Owned = DummyWithSpan;

        fn to_owned(&self) -> Self::Owned {
            DummyWithSpan
        }
    }

    let cow: Cow<'_, DummyWithSpan> = Cow::Borrowed(&DummyWithSpan);
    assert!(cow.span().is_some());
}

