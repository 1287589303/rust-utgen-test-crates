pub fn from_input_forward(input: &Input<'_>) -> Config {
        let look_behind = input
            .start()
            .checked_sub(1)
            .and_then(|i| input.haystack().get(i).copied());
        Config { look_behind, anchored: input.get_anchored() }
    }