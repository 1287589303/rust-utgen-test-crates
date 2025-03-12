// Answer 0

#[test]
fn test_span_fmt_with_span_locations() {
    #[cfg(span_locations)]
    {
        let span = Span { lo: 0, hi: 0 };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "bytes(0..0)";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: 0, hi: 1 };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "bytes(0..1)";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: std::u32::MAX - 1, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "bytes(4294967294..4294967295)";
        assert_eq!(buffer, expected);
    }
}

#[test]
fn test_span_fmt_without_span_locations() {
    #[cfg(not(span_locations))]
    {
        let span = Span { lo: 0, hi: 0 };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "Span";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: 1, hi: 1 };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "Span";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: std::u32::MAX, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "Span";
        assert_eq!(buffer, expected);
    }
}

#[test]
fn test_span_fmt_boundary_conditions() {
    #[cfg(span_locations)]
    {
        let span = Span { lo: 0, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "bytes(0..4294967295)";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: std::u32::MAX, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "bytes(4294967295..4294967295)";
        assert_eq!(buffer, expected);
    }
    
    #[cfg(not(span_locations))]
    {
        let span = Span { lo: 0, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "Span";
        assert_eq!(buffer, expected);
        
        let span = Span { lo: std::u32::MAX, hi: std::u32::MAX };
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{}", span);
        
        let expected = "Span";
        assert_eq!(buffer, expected);
    }
}

