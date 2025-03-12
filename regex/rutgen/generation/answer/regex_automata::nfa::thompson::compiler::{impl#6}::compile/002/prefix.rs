// Answer 0

#[test]
fn test_compile_with_existing_hash() {
    let mut builder = Builder::new();
    let mut utf8_state = Utf8State {
        compiled: Utf8BoundedMap::new(10),
        uncompiled: Vec::new(),
    };
    let mut utf8_compiler = Utf8Compiler::new(&mut builder, &mut utf8_state).unwrap();

    let transition_1 = Transition { start: 1, end: 2, next: StateID(1) };
    let transition_2 = Transition { start: 3, end: 4, next: StateID(2) };
    let node = vec![transition_1.clone(), transition_2.clone()];

    let hash = utf8_state.compiled.hash(&node);
    utf8_state.compiled.set(node.clone(), hash, StateID(3)); // Pre-fill the map

    // Now call the compile function
    let result = utf8_compiler.compile(node.clone());
}

#[test]
fn test_compile_with_valid_node() {
    let mut builder = Builder::new();
    let mut utf8_state = Utf8State {
        compiled: Utf8BoundedMap::new(10),
        uncompiled: Vec::new(),
    };
    let mut utf8_compiler = Utf8Compiler::new(&mut builder, &mut utf8_state).unwrap();

    let transition_1 = Transition { start: 4, end: 5, next: StateID(3) };
    let transition_2 = Transition { start: 6, end: 7, next: StateID(4) };
    let node = vec![transition_1.clone(), transition_2.clone()];

    let hash = utf8_state.compiled.hash(&node);
    utf8_state.compiled.set(node.clone(), hash, StateID(5)); // Simulating existing entry

    // Ensure builder can handle the sparse addition
    let _ = builder.add_sparse(node.clone()).unwrap(); // Pre-fill builder

    // Now call the compile function
    let result = utf8_compiler.compile(node.clone());
}

