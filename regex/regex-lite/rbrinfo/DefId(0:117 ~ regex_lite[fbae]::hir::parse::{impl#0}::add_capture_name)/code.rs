fn add_capture_name(&self, name: &str) -> Result<(), Error> {
        let mut names = self.capture_names.borrow_mut();
        match names.binary_search_by(|n| name.cmp(n)) {
            Ok(_) => Err(Error::new(ERR_DUPLICATE_CAPTURE_NAME)),
            Err(i) => {
                names.insert(i, name.to_string());
                Ok(())
            }
        }
    }