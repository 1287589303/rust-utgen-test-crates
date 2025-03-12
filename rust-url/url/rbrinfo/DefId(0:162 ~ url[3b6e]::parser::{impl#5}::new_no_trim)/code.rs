pub fn new_no_trim(input: &'i str) -> Self {
        Input {
            chars: input.chars(),
        }
    }