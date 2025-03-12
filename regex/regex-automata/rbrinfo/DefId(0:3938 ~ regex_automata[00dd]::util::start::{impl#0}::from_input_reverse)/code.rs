pub fn from_input_reverse(input: &Input<'_>) -> Config {
        let look_behind = input.haystack().get(input.end()).copied();
        Config { look_behind, anchored: input.get_anchored() }
    }