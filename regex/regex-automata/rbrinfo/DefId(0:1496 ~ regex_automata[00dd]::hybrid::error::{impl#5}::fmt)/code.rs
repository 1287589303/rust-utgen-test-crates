fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            StartError::Cache { .. } => write!(
                f,
                "error computing start state because of cache inefficiency"
            ),
            StartError::Quit { byte } => write!(
                f,
                "error computing start state because the look-behind byte \
                 {:?} triggered a quit state",
                crate::util::escape::DebugByte(byte),
            ),
            StartError::UnsupportedAnchored { mode: Anchored::Yes } => {
                write!(
                    f,
                    "error computing start state because \
                     anchored searches are not supported or enabled"
                )
            }
            StartError::UnsupportedAnchored { mode: Anchored::No } => {
                write!(
                    f,
                    "error computing start state because \
                     unanchored searches are not supported or enabled"
                )
            }
            StartError::UnsupportedAnchored {
                mode: Anchored::Pattern(pid),
            } => {
                write!(
                    f,
                    "error computing start state because \
                     anchored searches for a specific pattern ({}) \
                     are not supported or enabled",
                    pid.as_usize(),
                )
            }
        }
    }