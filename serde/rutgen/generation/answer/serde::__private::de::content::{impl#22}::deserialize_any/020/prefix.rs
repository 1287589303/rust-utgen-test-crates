// Answer 0

#[test]
fn test_deserialize_any_u16_valid() {
    let content = Content::U16(0);
    let deserializer = ContentRefDeserializer::new(&content);
    let dummy_visitor = DummyVisitor;

    let _ = deserializer.deserialize_any(dummy_visitor);
}

#[test]
fn test_deserialize_any_u16_max() {
    let content = Content::U16(65535);
    let deserializer = ContentRefDeserializer::new(&content);
    let dummy_visitor = DummyVisitor;

    let _ = deserializer.deserialize_any(dummy_visitor);
}

#[test]
fn test_deserialize_any_u16_mid() {
    let content = Content::U16(32768);
    let deserializer = ContentRefDeserializer::new(&content);
    let dummy_visitor = DummyVisitor;

    let _ = deserializer.deserialize_any(dummy_visitor);
}

struct DummyVisitor;

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = ();

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_char<E>(self, _: char) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_some<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Visitor<'de>,
    {
        Ok(())
    }

    fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Visitor<'de>,
    {
        Ok(())
    }

    fn visit_seq<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Visitor<'de>,
    {
        Ok(())
    }

    fn visit_map<V>(self, _: V) -> Result<Self::Value, E>
    where
        V: Visitor<'de>,
    {
        Ok(())
    }
}

