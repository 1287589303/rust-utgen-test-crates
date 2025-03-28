fn serialize_field<T>(&mut self, value: &T) -> Result<(), M::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<M::Error>::new()));
            self.fields.push(value);
            Ok(())
        }