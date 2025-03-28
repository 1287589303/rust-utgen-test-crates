fn serialize_u128(self, value: u128) -> Result<Value> {
        #[cfg(feature = "arbitrary_precision")]
        {
            Ok(Value::Number(value.into()))
        }

        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
            }
        }
    }