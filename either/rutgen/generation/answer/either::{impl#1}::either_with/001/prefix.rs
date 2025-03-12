// Answer 0

#[test]
fn test_either_with_right_case() {
    let ctx = 5; // context of type i32
    let value = Right(2.7); // example Right value of type f64

    value.either_with(ctx, 
                      |context, integer| {
                          // Implementation for integer (not invoked in this test)
                      }, 
                      |context, real| {
                          // Handling the Right case
                          let rounded = f64::round(real) as i32;
                          // potentially do something with rounded value
                      });
}

#[test]
fn test_either_with_right_case_string_context() {
    let ctx = String::from("context"); // context of type String
    let value = Right(3.3); // example Right value of type f64

    value.either_with(ctx, 
                      |context, integer| {
                          // Implementation for integer (not invoked in this test)
                      }, 
                      |context, real| {
                          // Handling the Right case
                          let rounded = f64::round(real) as i32;
                          // potentially do something with rounded value
                      });
}

#[test]
fn test_either_with_right_case_struct_context() {
    struct Context {
        data: i32,
    }

    let ctx = Context { data: 10 }; // context of a custom struct
    let value = Right(4.9); // example Right value of type f64

    value.either_with(ctx, 
                      |context, integer| {
                          // Implementation for integer (not invoked in this test)
                      }, 
                      |context, real| {
                          // Handling the Right case
                          let rounded = f64::round(real) as i32;
                          // potentially do something with rounded value
                      });
}

