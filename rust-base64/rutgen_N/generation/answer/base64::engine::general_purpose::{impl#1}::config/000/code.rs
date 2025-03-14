// Answer 0

#[derive(Debug)]
struct TestStruct {
    config: String,
}

trait Configurable {
    type Config;
    fn config(&self) -> &Self::Config;
}

impl Configurable for TestStruct {
    type Config = String;

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

#[test]
fn test_config() {
    let test_instance = TestStruct {
        config: String::from("test_config_value"),
    };
    
    assert_eq!(test_instance.config(), "test_config_value");
}

