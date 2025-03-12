use crate::lib::*;
pub use self::ignored_any::IgnoredAny;
#[cfg(all(not(feature = "std"), no_core_error))]
pub use crate::std_error::Error as StdError;
#[cfg(not(any(feature = "std", no_core_error)))]
pub use core::error::Error as StdError;
#[cfg(feature = "std")]
pub use std::error::Error as StdError;
struct OneOf {
    names: &'static [&'static str],
}
impl Display for OneOf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.names.len() {
            0 => panic!(),
            1 => write!(formatter, "`{}`", self.names[0]),
            2 => write!(formatter, "`{}` or `{}`", self.names[0], self.names[1]),
            _ => {
                tri!(formatter.write_str("one of "));
                for (i, alt) in self.names.iter().enumerate() {
                    if i > 0 {
                        tri!(formatter.write_str(", "));
                    }
                    tri!(write!(formatter, "`{}`", alt));
                }
                Ok(())
            }
        }
    }
}
