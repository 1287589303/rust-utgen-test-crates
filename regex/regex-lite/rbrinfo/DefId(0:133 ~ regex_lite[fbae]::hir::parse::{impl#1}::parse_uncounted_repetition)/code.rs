fn parse_uncounted_repetition(
        &self,
        mut concat: Vec<Hir>,
    ) -> Result<Vec<Hir>, Error> {
        let sub = match concat.pop() {
            Some(hir) => Box::new(hir),
            None => {
                return Err(Error::new(ERR_UNCOUNTED_REP_SUB_MISSING));
            }
        };
        let (min, max) = match self.char() {
            '?' => (0, Some(1)),
            '*' => (0, None),
            '+' => (1, None),
            unk => unreachable!("unrecognized repetition operator '{}'", unk),
        };
        let mut greedy = true;
        if self.bump() && self.char() == '?' {
            greedy = false;
            self.bump();
        }
        if self.flags().swap_greed {
            greedy = !greedy;
        }
        concat.push(Hir::repetition(hir::Repetition {
            min,
            max,
            greedy,
            sub,
        }));
        Ok(concat)
    }