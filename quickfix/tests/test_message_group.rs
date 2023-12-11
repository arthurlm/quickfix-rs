use msg_const::*;

mod msg_const;

#[test]
fn test_build_with_group() {
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
