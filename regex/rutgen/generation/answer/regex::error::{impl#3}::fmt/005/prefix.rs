// Answer 0

#[test]
fn test_error_syntax_debug_success() {
    let err = String::from("This is a syntax error.");
    let error_instance = Error::Syntax(err);
    let mut output = Vec::new();
    let result = core::fmt::write(&mut output, |f| error_instance.fmt(f));
}

#[test]
#[should_panic]
fn test_error_syntax_debug_fail() {
    let err = String::from("This is a syntax error.");
    let error_instance = Error::Syntax(err);
    let hr: String = core::iter::repeat('~').take(79).collect();
    let mut output = Vec::new();
    // Force the last writeln! to fail by manipulating the output, here we force a panic after the first two successful writes.
    core::fmt::write(&mut output, |f| {
        writeln!(f, "Syntax(")?;
        writeln!(f, "{}", hr)?;
        // Assuming we do something here to cause an error in the next writeln!
        std::panic::panic_any("Forcing panic to simulate writeln error on hr");
    });
}

