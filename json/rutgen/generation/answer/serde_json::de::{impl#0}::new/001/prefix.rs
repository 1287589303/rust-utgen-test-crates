// Answer 0

#[test]
fn test_new_with_slice_read() {
    struct MockSliceRead<'a> {
        data: &'a [u8],
    }

    impl<'de> read::Read<'de> for MockSliceRead<'_> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let len = buf.len().min(self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data = &self.data[len..];
            Ok(len)
        }
    }

    let input = b"{}";
    let read = MockSliceRead { data: input };
    let deserializer = Deserializer::new(read);
}

#[test]
fn test_new_with_str_read() {
    struct MockStrRead<'a> {
        data: &'a str,
    }

    impl<'de> read::Read<'de> for MockStrRead<'_> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let len = buf.len().min(self.data.len());
            buf[..len].copy_from_slice(&self.data.as_bytes()[..len]);
            self.data = &self.data[len..];
            Ok(len)
        }
    }

    let input = "{}";
    let read = MockStrRead { data: input };
    let deserializer = Deserializer::new(read);
}

#[test]
fn test_new_with_fused() {
    struct MockFusedRead {
        position: usize,
        data: Vec<u8>,
    }

    impl<'de> read::Read<'de> for MockFusedRead {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let len = buf.len().min(self.data.len() - self.position);
            buf[..len].copy_from_slice(&self.data[self.position..self.position + len]);
            self.position += len;
            Ok(len)
        }
    }

    let input = b"{}";
    let read = MockFusedRead {
        position: 0,
        data: input.to_vec(),
    };
    let deserializer = Deserializer::new(read);
}

