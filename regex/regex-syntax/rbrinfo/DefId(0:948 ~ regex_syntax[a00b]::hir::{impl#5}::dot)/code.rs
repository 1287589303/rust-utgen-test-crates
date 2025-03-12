pub fn dot(dot: Dot) -> Hir {
        match dot {
            Dot::AnyChar => Hir::class(Class::Unicode(ClassUnicode::new([
                ClassUnicodeRange::new('\0', '\u{10FFFF}'),
            ]))),
            Dot::AnyByte => Hir::class(Class::Bytes(ClassBytes::new([
                ClassBytesRange::new(b'\0', b'\xFF'),
            ]))),
            Dot::AnyCharExcept(ch) => {
                let mut cls =
                    ClassUnicode::new([ClassUnicodeRange::new(ch, ch)]);
                cls.negate();
                Hir::class(Class::Unicode(cls))
            }
            Dot::AnyCharExceptLF => {
                Hir::class(Class::Unicode(ClassUnicode::new([
                    ClassUnicodeRange::new('\0', '\x09'),
                    ClassUnicodeRange::new('\x0B', '\u{10FFFF}'),
                ])))
            }
            Dot::AnyCharExceptCRLF => {
                Hir::class(Class::Unicode(ClassUnicode::new([
                    ClassUnicodeRange::new('\0', '\x09'),
                    ClassUnicodeRange::new('\x0B', '\x0C'),
                    ClassUnicodeRange::new('\x0E', '\u{10FFFF}'),
                ])))
            }
            Dot::AnyByteExcept(byte) => {
                let mut cls =
                    ClassBytes::new([ClassBytesRange::new(byte, byte)]);
                cls.negate();
                Hir::class(Class::Bytes(cls))
            }
            Dot::AnyByteExceptLF => {
                Hir::class(Class::Bytes(ClassBytes::new([
                    ClassBytesRange::new(b'\0', b'\x09'),
                    ClassBytesRange::new(b'\x0B', b'\xFF'),
                ])))
            }
            Dot::AnyByteExceptCRLF => {
                Hir::class(Class::Bytes(ClassBytes::new([
                    ClassBytesRange::new(b'\0', b'\x09'),
                    ClassBytesRange::new(b'\x0B', b'\x0C'),
                    ClassBytesRange::new(b'\x0E', b'\xFF'),
                ])))
            }
        }
    }