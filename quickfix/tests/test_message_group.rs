use quickfix::*;

use crate::utils::*;

mod utils;

#[test]
fn test_build_with_simple_group() {
    let msg = build_news("Simple news", &[]).unwrap();
    assert_eq!(
        msg.as_string().unwrap(),
        "9=26\u{1}35=B\u{1}33=0\u{1}148=Simple news\u{1}10=189\u{1}"
    );

    let msg = build_news(
        "New great project available",
        &["Release soon", "Idea: making FIX available for everyone"],
    )
    .unwrap();
    assert_eq!(
        msg.as_string().unwrap(),
        "9=101\u{1}35=B\u{1}\
         33=2\u{1}\
         58=Release soon\u{1}\
         58=Idea: making FIX available for everyone\u{1}\
         148=New great project available\u{1}10=205\u{1}"
    );
}

#[test]
fn test_build_with_complex_group() {
    let msg = build_list_status(
        "foo",
        &[
            &[(11, "a1"), (14, "b1"), (84, "c1"), (8, "e1"), (6, "d1")],
            &[(84, "c2"), (6, "d2"), (11, "a2"), (14, "b2")],
            &[(11, "a3"), (6, "d3"), (14, "b3"), (84, "c3")],
        ],
    )
    .unwrap();

    assert_eq!(
        msg.as_string().unwrap(),
        "9=81\u{1}66=foo\u{1}73=3\u{1}\
         11=a1\u{1}14=b1\u{1}84=c1\u{1}6=d1\u{1}8=e1\u{1}\
         11=a2\u{1}14=b2\u{1}84=c2\u{1}6=d2\u{1}\
         11=a3\u{1}14=b3\u{1}84=c3\u{1}6=d3\u{1}\
         10=206\u{1}"
    );
}

#[test]
fn test_read_group_clone() {
    let msg = build_news("Great news", &["Some new library", "are available", "soon"]).unwrap();
    assert_eq!(msg.get_field(MSG_HEADLINE).unwrap(), "Great news");

    // Read before
    assert!(msg.clone_group(0, MSG_NO_LINES_OF_TEXT).is_none());

    // Read configured lines
    let group = msg.clone_group(1, MSG_NO_LINES_OF_TEXT).unwrap();
    assert_eq!(group.get_field(MSG_TEXT).unwrap(), "Some new library");

    let group = msg.clone_group(2, MSG_NO_LINES_OF_TEXT).unwrap();
    assert_eq!(group.get_field(MSG_TEXT).unwrap(), "are available");

    let group = msg.clone_group(3, MSG_NO_LINES_OF_TEXT).unwrap();
    assert_eq!(group.get_field(MSG_TEXT).unwrap(), "soon");

    // Read after
    assert!(msg.clone_group(4, MSG_NO_LINES_OF_TEXT).is_none());
}

#[test]
fn test_read_group_ref() {
    let msg = build_news("Great news", &["Some new library", "are available", "soon"]).unwrap();
    assert_eq!(msg.get_field(MSG_HEADLINE).unwrap(), "Great news");

    fn read_text(group: &Group) -> String {
        group.get_field(MSG_TEXT).unwrap()
    }

    // Read before
    assert!(msg.with_group(0, MSG_NO_LINES_OF_TEXT, read_text).is_none());

    // Read configured lines
    assert_eq!(
        msg.with_group(1, MSG_NO_LINES_OF_TEXT, read_text).unwrap(),
        "Some new library"
    );
    assert_eq!(
        msg.with_group(2, MSG_NO_LINES_OF_TEXT, read_text).unwrap(),
        "are available"
    );
    assert_eq!(
        msg.with_group(3, MSG_NO_LINES_OF_TEXT, read_text).unwrap(),
        "soon"
    );

    // Read after
    assert!(msg.with_group(4, MSG_NO_LINES_OF_TEXT, read_text).is_none());
}

#[test]
fn test_modify_group() {
    let mut msg = build_news("Great news", &["Some new library", "are available", "soon"]).unwrap();

    // Check before
    assert_eq!(
        msg.as_string().unwrap(),
        "9=70\u{1}35=B\u{1}33=3\u{1}\
         58=Some new library\u{1}\
         58=are available\u{1}\
         58=soon\u{1}\
         148=Great news\u{1}\
         10=020\u{1}"
    );

    // Update valid group
    assert!(msg
        .with_group_mut(2, MSG_NO_LINES_OF_TEXT, |g| g
            .set_field(MSG_TEXT, "will be available")
            .unwrap())
        .is_some());

    // Update invalid group
    assert!(msg
        .with_group_mut(58, MSG_NO_LINES_OF_TEXT, |g| g
            .set_field(MSG_TEXT, "whatever")
            .unwrap())
        .is_none());

    // Check after
    assert_eq!(
        msg.as_string().unwrap(),
        "9=74\u{1}35=B\u{1}33=3\u{1}\
         58=Some new library\u{1}\
         58=will be available\u{1}\
         58=soon\u{1}148=Great news\u{1}\
         10=127\u{1}"
    );
}
