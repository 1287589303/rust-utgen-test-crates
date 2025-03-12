// Answer 0

#[test]
fn test_printer_builder_default() {
    let builder = PrinterBuilder::default();
    let _printer = builder.build();
}

#[test]
fn test_printer_builder_new() {
    let builder = PrinterBuilder::new();
    let _printer = builder.build();
}

