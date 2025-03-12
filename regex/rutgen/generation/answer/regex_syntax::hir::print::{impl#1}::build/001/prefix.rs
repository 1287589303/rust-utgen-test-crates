// Answer 0

#[test]
fn test_printer_builder_build() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
}

#[test]
fn test_printer_builder_build_multiple_times() {
    let builder = PrinterBuilder::new();
    let printer1 = builder.build();
    let printer2 = builder.build();
}

#[test]
fn test_printer_builder_build_sequence() {
    let builder = PrinterBuilder::new();
    let printer1 = builder.build();
    let printer2 = builder.build();
    let printer3 = builder.build();
}

