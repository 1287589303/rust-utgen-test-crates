// Answer 0

#[test]
fn test_shallow_clone_vec_case_1() {
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) };
    let offset: *const u8 = unsafe { buf.add(1) };
    let len: usize = 5;
    let atom: AtomicPtr<()> = AtomicPtr::new((buf as usize + 100) as *mut ());

    let shared: Bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_case_2() {
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(20, 1).unwrap()) };
    let offset: *const u8 = unsafe { buf.add(2) };
    let len: usize = 10;
    let atom: AtomicPtr<()> = AtomicPtr::new((buf as usize + 200) as *mut ());

    let shared: Bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_case_3() {
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(30, 1).unwrap()) };
    let offset: *const u8 = unsafe { buf.add(3) };
    let len: usize = 15;
    let atom: AtomicPtr<()> = AtomicPtr::new(buf);

    let shared: Bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };
}

