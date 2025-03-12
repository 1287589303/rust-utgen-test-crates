// Answer 0

#[test]
fn test_from_repr_endlf() {
    let repr: u32 = 0b00_0000_0000_0000_1000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_cases() {
    let repr_start: u32 = 0b00_0000_0000_0000_0001;
    let result_start = Look::from_repr(repr_start);
    
    let repr_end: u32 = 0b00_0000_0000_0000_0010;
    let result_end = Look::from_repr(repr_end);
    
    let repr_startlf: u32 = 0b00_0000_0000_0000_0100;
    let result_startlf = Look::from_repr(repr_startlf);
    
    let repr_startcrlf: u32 = 0b00_0000_0000_0001_0000;
    let result_startcrlf = Look::from_repr(repr_startcrlf);
    
    let repr_endcrlf: u32 = 0b00_0000_0000_0010_0000;
    let result_endcrlf = Look::from_repr(repr_endcrlf);
    
    let repr_wordascii: u32 = 0b00_0000_0000_0100_0000;
    let result_wordascii = Look::from_repr(repr_wordascii);
    
    let repr_wordasciineg: u32 = 0b00_0000_0000_1000_0000;
    let result_wordasciineg = Look::from_repr(repr_wordasciineg);
    
    let repr_wordunicode: u32 = 0b00_0000_0001_0000_0000;
    let result_wordunicode = Look::from_repr(repr_wordunicode);
    
    let repr_wordunicodeng: u32 = 0b00_0000_0010_0000_0000;
    let result_wordunicodeng = Look::from_repr(repr_wordunicodeng);
    
    let repr_wordstartascii: u32 = 0b00_0000_0100_0000_0000;
    let result_wordstartascii = Look::from_repr(repr_wordstartascii);
    
    let repr_wordendascii: u32 = 0b00_0000_1000_0000_0000;
    let result_wordendascii = Look::from_repr(repr_wordendascii);
    
    let repr_wordstartunicode: u32 = 0b00_0001_0000_0000_0000;
    let result_wordstartunicode = Look::from_repr(repr_wordstartunicode);
    
    let repr_wordendunicode: u32 = 0b00_0010_0000_0000_0000;
    let result_wordendunicode = Look::from_repr(repr_wordendunicode);
    
    let repr_wordstarthalfascii: u32 = 0b00_0100_0000_0000_0000;
    let result_wordstarthalfascii = Look::from_repr(repr_wordstarthalfascii);
    
    let repr_wordendhalfascii: u32 = 0b00_1000_0000_0000_0000;
    let result_wordendhalfascii = Look::from_repr(repr_wordendhalfascii);
    
    let repr_wordstarthalfunicode: u32 = 0b01_0000_0000_0000_0000;
    let result_wordstarthalfunicode = Look::from_repr(repr_wordstarthalfunicode);
    
    let repr_wordendhalfunicode: u32 = 0b10_0000_0000_0000_0000;
    let result_wordendhalfunicode = Look::from_repr(repr_wordendhalfunicode);
}

