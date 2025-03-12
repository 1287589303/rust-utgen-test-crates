pub(crate) fn fmt_state_indicator<A: Automaton>(
    f: &mut core::fmt::Formatter<'_>,
    dfa: A,
    id: StateID,
) -> core::fmt::Result {
    if dfa.is_dead_state(id) {
        write!(f, "D")?;
        if dfa.is_start_state(id) {
            write!(f, ">")?;
        } else {
            write!(f, " ")?;
        }
    } else if dfa.is_quit_state(id) {
        write!(f, "Q ")?;
    } else if dfa.is_start_state(id) {
        if dfa.is_accel_state(id) {
            write!(f, "A>")?;
        } else {
            write!(f, " >")?;
        }
    } else if dfa.is_match_state(id) {
        if dfa.is_accel_state(id) {
            write!(f, "A*")?;
        } else {
            write!(f, " *")?;
        }
    } else if dfa.is_accel_state(id) {
        write!(f, "A ")?;
    } else {
        write!(f, "  ")?;
    }
    Ok(())
}