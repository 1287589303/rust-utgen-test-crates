// Answer 0

#[test]
fn test_fill_tag_single_element() {
    let mut tags: [Tag; 1] = [Tag(0)];
    let new_tag = Tag(100);
    tags.fill_tag(new_tag);
}

#[test]
fn test_fill_tag_multiple_elements() {
    let mut tags: [Tag; 10] = [Tag(0); 10];
    let new_tag = Tag(150);
    tags.fill_tag(new_tag);
}

#[test]
fn test_fill_tag_with_empty() {
    let mut tags: [Tag; 5] = [Tag(0); 5];
    tags.fill_empty();
}

#[test]
fn test_fill_tag_boundary_min() {
    let mut tags: [Tag; 1] = [Tag(0)];
    let new_tag = Tag(0);
    tags.fill_tag(new_tag);
}

#[test]
fn test_fill_tag_boundary_max() {
    let mut tags: [Tag; 100] = [Tag(0); 100];
    let new_tag = Tag(255);
    tags.fill_tag(new_tag);
}

