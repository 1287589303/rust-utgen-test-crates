// Answer 0

#[test]
fn test_write_bool_false_to_buffer() {
    let mut buffer = vec![];
    let mut formatter = TestFormatter {};
    let _ = formatter.write_bool(&mut buffer, false);
}

#[test]
fn test_write_bool_false_to_writer() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(vec![]);
    let mut formatter = TestFormatter {};
    let _ = formatter.write_bool(&mut cursor, false);
}

#[test]
fn test_write_bool_false_to_string() {
    use std::io::Cursor;

    let mut string_writer = Cursor::new(String::new());
    let mut formatter = TestFormatter {};
    let _ = formatter.write_bool(&mut string_writer, false);
}

struct TestFormatter;

impl Formatter for TestFormatter {
    // Implementations of Formatter methods can be empty for testing.
    fn write_null<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_i8<W>(&mut self, _writer: &mut W, _value: i8) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_i16<W>(&mut self, _writer: &mut W, _value: i16) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_i32<W>(&mut self, _writer: &mut W, _value: i32) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_i64<W>(&mut self, _writer: &mut W, _value: i64) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_i128<W>(&mut self, _writer: &mut W, _value: i128) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_u8<W>(&mut self, _writer: &mut W, _value: u8) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_u16<W>(&mut self, _writer: &mut W, _value: u16) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_u32<W>(&mut self, _writer: &mut W, _value: u32) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_u64<W>(&mut self, _writer: &mut W, _value: u64) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_u128<W>(&mut self, _writer: &mut W, _value: u128) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_f32<W>(&mut self, _writer: &mut W, _value: f32) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_f64<W>(&mut self, _writer: &mut W, _value: f64) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_number_str<W>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_string_fragment<W>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_char_escape<W>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_byte_array<W>(&mut self, _writer: &mut W, _value: &[u8]) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_array_value<W>(&mut self, _writer: &mut W, _first: bool) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_object_key<W>(&mut self, _writer: &mut W, _first: bool) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn begin_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }

    fn write_raw_fragment<W>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()>
    where W: ?Sized + io::Write { Ok(()) }
}

