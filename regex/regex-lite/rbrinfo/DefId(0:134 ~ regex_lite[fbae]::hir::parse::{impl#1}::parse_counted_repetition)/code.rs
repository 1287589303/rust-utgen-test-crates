fn parse_counted_repetition(
        &self,
        mut concat: Vec<Hir>,
    ) -> Result<Vec<Hir>, Error> {
        assert_eq!(self.char(), '{', "expected opening brace");
        let sub = match concat.pop() {
            Some(hir) => Box::new(hir),
            None => {
                return Err(Error::new(ERR_COUNTED_REP_SUB_MISSING));
            }
        };
        if !self.bump_and_bump_space() {
            return Err(Error::new(ERR_COUNTED_REP_UNCLOSED));
        }
        let min = self.parse_decimal()?;
        let mut max = Some(min);
        if self.is_done() {
            return Err(Error::new(ERR_COUNTED_REP_MIN_UNCLOSED));
        }
        if self.char() == ',' {
            if !self.bump_and_bump_space() {
                return Err(Error::new(ERR_COUNTED_REP_COMMA_UNCLOSED));
            }
            if self.char() != '}' {
                max = Some(self.parse_decimal()?);
            } else {
                max = None;
            }
            if self.is_done() {
                return Err(Error::new(ERR_COUNTED_REP_MIN_MAX_UNCLOSED));
            }
        }
        if self.char() != '}' {
            return Err(Error::new(ERR_COUNTED_REP_INVALID));
        }

        let mut greedy = true;
        if self.bump_and_bump_space() && self.char() == '?' {
            greedy = false;
            self.bump();
        }
        if self.flags().swap_greed {
            greedy = !greedy;
        }

        if max.map_or(false, |max| min > max) {
            return Err(Error::new(ERR_COUNTED_REP_INVALID_RANGE));
        }
        concat.push(Hir::repetition(hir::Repetition {
            min,
            max,
            greedy,
            sub,
        }));
        Ok(concat)
    }