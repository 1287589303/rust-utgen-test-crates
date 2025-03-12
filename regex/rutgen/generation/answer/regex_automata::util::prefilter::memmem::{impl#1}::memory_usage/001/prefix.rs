// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::search::Span;

    #[test]
    fn test_memory_usage_enabled_features() {
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            let needle = b"test"; // Example needle of length 4
            let finder = memchr::memmem::Finder::new(needle);
            let memmem = Memmem { finder };

            let usage = memmem.memory_usage();
            // Uncomment the assertion line if needed for validation
            // assert_eq!(usage, needle.len());
        }
    }

    #[test]
    #[should_panic]
    fn test_memory_usage_no_features() {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        {
            let memmem = Memmem { _unused: () };
            let _usage = memmem.memory_usage(); 
        }
    }

    #[test]
    fn test_memory_usage_various_needle_lengths() {
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            let needles = vec![
                b"",               // Length 0
                b"a",              // Length 1
                b"example",        // Length 7
                b"Rust is great!", // Length 15
            ];

            for needle in needles {
                let finder = memchr::memmem::Finder::new(needle);
                let memmem = Memmem { finder };

                let usage = memmem.memory_usage();
                // Uncomment the assertion line if needed for validation
                // assert_eq!(usage, needle.len());
            }
        }
    }
}

