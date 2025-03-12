// Answer 0

#[test]
#[should_panic]
fn test_move_next_stride_greater_than_bucket_mask() {
    let mut probe_seq = ProbeSeq { pos: 0, stride: 10 }; // striding greater than bucket_mask
    let bucket_mask = 5; // bucket_mask is less than stride
    probe_seq.move_next(bucket_mask);
}

#[test]
#[should_panic]
fn test_move_next_stride_equal_to_bucket_mask() {
    let mut probe_seq = ProbeSeq { pos: 0, stride: 5 }; // striding equal to bucket_mask
    let bucket_mask = 5; // bucket_mask is equal to stride
    probe_seq.move_next(bucket_mask);
}

#[test]
#[should_panic]
fn test_move_next_large_stride() {
    let mut probe_seq = ProbeSeq { pos: 0, stride: 100 }; // striding much greater than bucket_mask
    let bucket_mask = 50; // bucket_mask is less than stride
    probe_seq.move_next(bucket_mask);
}

