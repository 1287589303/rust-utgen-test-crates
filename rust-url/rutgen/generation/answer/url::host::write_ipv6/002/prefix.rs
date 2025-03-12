// Answer 0

#[test]
fn test_write_ipv6_compress_start_1() {
    struct TestAddr {
        segments: [u16; 8],
    }
    
    impl Ipv6Addr for TestAddr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestAddr { segments: [0, 0, 0x1234, 0x5678, 0x9abc, 0, 0, 0] };
    let mut output = String::new();
    let result = write_ipv6(&addr, &mut output);
}

#[test]
fn test_write_ipv6_compress_start_2() {
    struct TestAddr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestAddr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestAddr { segments: [0xabcd, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3, 0x4] };
    let mut output = String::new();
    let result = write_ipv6(&addr, &mut output);
}

#[test]
fn test_write_ipv6_compress_start_3() {
    struct TestAddr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestAddr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestAddr { segments: [0, 0, 0, 0, 0, 0x1234, 0x5678, 0x9abc] };
    let mut output = String::new();
    let result = write_ipv6(&addr, &mut output);
}

#[test]
fn test_write_ipv6_full_zeros() {
    struct TestAddr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestAddr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestAddr { segments: [0, 0, 0, 0, 0, 0, 0, 0] };
    let mut output = String::new();
    let result = write_ipv6(&addr, &mut output);
}

#[test]
fn test_write_ipv6_no_compress() {
    struct TestAddr {
        segments: [u16; 8],
    }

    impl Ipv6Addr for TestAddr {
        fn segments(&self) -> &[u16; 8] {
            &self.segments
        }
    }

    let addr = TestAddr { segments: [0x1234, 0x5678, 0x9abc, 0xdeef, 0, 0, 0, 0] };
    let mut output = String::new();
    let result = write_ipv6(&addr, &mut output);
}

