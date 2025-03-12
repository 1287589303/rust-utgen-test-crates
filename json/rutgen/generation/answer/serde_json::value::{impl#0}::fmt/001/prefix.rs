// Answer 0

#[test]
fn test_fmt_object_with_err_formatter() {
    struct ErrFormatter;

    impl fmt::Write for ErrFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut err_formatter = ErrFormatter;
    
    let obj = Value::Object(Map {
        map: MapImpl::new(),
    });

    let _ = obj.fmt(&mut err_formatter);
}

#[test]
fn test_fmt_object_with_err_formatter_edge_case() {
    struct ErrFormatter;

    impl fmt::Write for ErrFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut err_formatter = ErrFormatter;
    
    let obj = Value::Object(Map {
        map: MapImpl::new(),
    });

    let _ = obj.fmt(&mut err_formatter);
}

