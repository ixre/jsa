struct S {}

fn borrow_obj() -> S {
    let s = S {};
    s
    //*s
}

#[test]
fn test_borrow() {
    borrow_obj();
}
