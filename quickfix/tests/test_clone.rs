use std::fmt::Debug;

use quickfix::*;

fn check<T: Clone + Debug>(obj1: T) {
    // Do some operation before cloning.
    println!("{obj1:?}");

    // Clone and do ops and both obj.
    let obj2 = obj1.clone();
    println!("{obj1:?}");
    println!("{obj2:?}");

    // Drop obj1 and check obj2 still exists.
    drop(obj1);
    println!("{obj2:?}");
}

#[test]
fn test_group() {
    check(Group::try_new(42, 89));
}

#[test]
fn test_header() {
    check(Header::new());
}

#[test]
fn test_message() {
    check(Message::new());
}

#[test]
fn test_trailer() {
    check(Trailer::new());
}
