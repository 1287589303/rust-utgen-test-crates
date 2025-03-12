// Answer 0

#[test]
fn test_socketaddr_serialize_v4_human_readable() {
    struct V4SocketAddr {
        addr: std::net::SocketAddrV4,
    }

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 8080);
    let socket_addr = V4SocketAddr { addr };

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;

        fn is_human_readable(&self) -> bool {
            true
        }

        fn serialize_newtype_variant<T: Serialize>(&self, _: &str, _: u32, _: &str, _: T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let _ = socket_addr.serialize(serializer);
}

#[test]
fn test_socketaddr_serialize_v4_non_human_readable() {
    struct V4SocketAddr {
        addr: std::net::SocketAddrV4,
    }

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 8080);
    let socket_addr = V4SocketAddr { addr };

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;

        fn is_human_readable(&self) -> bool {
            false
        }

        fn serialize_newtype_variant<T: Serialize>(&self, _: &str, _: u32, _: &str, _: T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let _ = socket_addr.serialize(serializer);
}

#[test]
fn test_socketaddr_serialize_v6_human_readable() {
    struct V6SocketAddr {
        addr: std::net::SocketAddrV6,
    }

    let addr = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    let socket_addr = V6SocketAddr { addr };

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;

        fn is_human_readable(&self) -> bool {
            true
        }

        fn serialize_newtype_variant<T: Serialize>(&self, _: &str, _: u32, _: &str, _: T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let _ = socket_addr.serialize(serializer);
}

#[test]
fn test_socketaddr_serialize_v6_non_human_readable() {
    struct V6SocketAddr {
        addr: std::net::SocketAddrV6,
    }

    let addr = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    let socket_addr = V6SocketAddr { addr };

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;

        fn is_human_readable(&self) -> bool {
            false
        }

        fn serialize_newtype_variant<T: Serialize>(&self, _: &str, _: u32, _: &str, _: T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let _ = socket_addr.serialize(serializer);
}

