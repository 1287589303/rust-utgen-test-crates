// Answer 0

#[test]
fn test_c_fail_with_err() {
    struct MockCompiler {
        add_fail_result: Result<StateID, BuildError>,
    }

    impl Compiler {
        fn add_fail(&self) -> Result<StateID, BuildError> {
            self.add_fail_result.clone()
        }
    }

    let mock_compiler = MockCompiler {
        add_fail_result: Err(BuildError { kind: BuildErrorKind::SomeError }),
    };

    let result = mock_compiler.c_fail();
}

#[test]
fn test_c_fail_with_none() {
    struct MockCompiler {
        add_fail_result: Result<StateID, BuildError>,
    }

    impl Compiler {
        fn add_fail(&self) -> Result<StateID, BuildError> {
            self.add_fail_result.clone()
        }
    }

    let mock_compiler = MockCompiler {
        add_fail_result: Err(BuildError { kind: BuildErrorKind::AnotherError }),
    };

    let result = mock_compiler.c_fail();
}

