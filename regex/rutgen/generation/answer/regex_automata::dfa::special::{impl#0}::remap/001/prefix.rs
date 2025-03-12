// Answer 0

#[test]
fn test_remap_increment() {
    let special = Special {
        max: StateID(0),
        quit_id: StateID(128),
        min_match: StateID(256),
        max_match: StateID(384),
        min_accel: StateID(512),
        max_accel: StateID(640),
        min_start: StateID(768),
        max_start: StateID(896),
    };

    let result = special.remap(|id| StateID(id.0 + 1));
}

#[test]
fn test_remap_decrement() {
    let special = Special {
        max: StateID(1024),
        quit_id: StateID(896),
        min_match: StateID(768),
        max_match: StateID(640),
        min_accel: StateID(512),
        max_accel: StateID(384),
        min_start: StateID(256),
        max_start: StateID(128),
    };

    let result = special.remap(|id| StateID(id.0 - 1));
}

#[test]
fn test_remap_identity() {
    let special = Special {
        max: StateID(512),
        quit_id: StateID(512),
        min_match: StateID(512),
        max_match: StateID(512),
        min_accel: StateID(512),
        max_accel: StateID(512),
        min_start: StateID(512),
        max_start: StateID(512),
    };

    let result = special.remap(|id| id);
}

#[test]
fn test_remap_varied_steps() {
    let special = Special {
        max: StateID(0),
        quit_id: StateID(128),
        min_match: StateID(256),
        max_match: StateID(384),
        min_accel: StateID(512),
        max_accel: StateID(640),
        min_start: StateID(768),
        max_start: StateID(896),
    };

    let result = special.remap(|id| StateID((id.0 + 128) % 1025));
}

