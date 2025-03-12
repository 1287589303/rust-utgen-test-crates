// Answer 0

#[test]
#[should_panic(expected = "invalid accelerator index")]
fn test_needles_index_equal_to_len() {
    struct TestAccels {
        accels: [AccelTy; ACCEL_LEN],
    }

    impl AsRef<[AccelTy]> for TestAccels {
        fn as_ref(&self) -> &[AccelTy] {
            &self.accels
        }
    }

    let accels = TestAccels { accels: [1, 0, 0, 0] };
    let accels_instance = Accels { accels };

    let _ = accels_instance.needles(accels_instance.len());
}

#[test]
#[should_panic(expected = "invalid accelerator index")]
fn test_needles_index_greater_than_len() {
    struct TestAccels {
        accels: [AccelTy; ACCEL_LEN],
    }

    impl AsRef<[AccelTy]> for TestAccels {
        fn as_ref(&self) -> &[AccelTy] {
            &self.accels
        }
    }

    let accels = TestAccels { accels: [1, 0, 0, 0] };
    let accels_instance = Accels { accels };

    let _ = accels_instance.needles(accels_instance.len() + 1);
}

