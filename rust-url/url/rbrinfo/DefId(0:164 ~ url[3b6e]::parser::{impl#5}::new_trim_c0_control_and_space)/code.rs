pub fn new_trim_c0_control_and_space(
        original_input: &'i str,
        vfn: Option<&dyn Fn(SyntaxViolation)>,
    ) -> Self {
        let input = original_input.trim_matches(c0_control_or_space);
        if let Some(vfn) = vfn {
            if input.len() < original_input.len() {
                vfn(SyntaxViolation::C0SpaceIgnored)
            }
            if input.chars().any(ascii_tab_or_new_line) {
                vfn(SyntaxViolation::TabOrNewlineIgnored)
            }
        }
        Input {
            chars: input.chars(),
        }
    }