// Answer 0

#[test]
fn test_write_fmt_valid_string() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) }).unwrap(),
        len: 0,
        cap: 10,
        data: ptr::null_mut(),
    };
    let args = format_args!("Hello, {}!", "world");
    let _ = bytes_mut.write_fmt(args);
}

#[test]
fn test_write_fmt_empty_string() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) }).unwrap(),
        len: 4,
        cap: 10,
        data: ptr::null_mut(),
    };
    let args = format_args!(""); 
    let _ = bytes_mut.write_fmt(args);
}

#[test]
fn test_write_fmt_special_characters() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) }).unwrap(),
        len: 0,
        cap: 10,
        data: ptr::null_mut(),
    };
    let args = format_args!("Special characters: ~!@#$%^&*()");
    let _ = bytes_mut.write_fmt(args);
}

#[test]
fn test_write_fmt_with_numbers() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) }).unwrap(),
        len: 0,
        cap: 10,
        data: ptr::null_mut(),
    };
    let args = format_args!("The answer is: {}", 42);
    let _ = bytes_mut.write_fmt(args);
}

#[test]
fn test_write_fmt_with_multiple_format_specifiers() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) }).unwrap(),
        len: 0,
        cap: 10,
        data: ptr::null_mut(),
    };
    let args = format_args!("{} is {} years old.", "Alice", 30);
    let _ = bytes_mut.write_fmt(args);
}

