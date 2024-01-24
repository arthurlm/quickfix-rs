use quickfix::*;

use crate::utils::*;

mod utils;

#[test]
fn test_build_with_simple_group() {
    let msg = build_news("Simple news", &[]).unwrap();
    assert_eq!(
        msg.to_fix_string().unwrap(),
        "9=26\u{1}35=B\u{1}33=0\u{1}148=Simple news\u{1}10=189\u{1}"
    );

    let msg = build_news(
        "New great project available",
        &["Release soon", "Idea: making FIX available for everyone"],
    )
    .unwrap();
    assert_eq!(
        msg.to_fix_string().unwrap(),
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
        msg.to_fix_string().unwrap(),
        "9=81\u{1}66=foo\u{1}73=3\u{1}\
         11=a1\u{1}14=b1\u{1}84=c1\u{1}6=d1\u{1}8=e1\u{1}\
         11=a2\u{1}14=b2\u{1}84=c2\u{1}6=d2\u{1}\
         11=a3\u{1}14=b3\u{1}84=c3\u{1}6=d3\u{1}\
         10=206\u{1}"
    );
}

#[test]
fn test_build_custom_with_recursive_groups() -> Result<(), QuickFixError> {
    let mut msg = Message::new();

    // Add groups to headers.
    msg.with_header_mut(|h| {
        h.add_group(&{
            let mut group = Group::try_with_orders(40, 50, &[50, 60, 70])?;
            group.set_field(50, "h10")?;
            group.set_field(60, "h11")?;
            group.set_field(70, "h12")?;
            group
        })?;
        h.add_group(&{
            let mut group = Group::try_with_orders(40, 50, &[50, 60, 70])?;
            group.set_field(70, "h22")?;
            group.set_field(60, "h21")?;
            group
        })?;

        Ok::<_, QuickFixError>(())
    })?;

    // Add groups in groups.
    msg.add_group(&{
        let mut group = Group::try_with_orders(100, 101, &[101, 102, 103])?;
        group.set_field(101, "v1")?;
        group.add_group(&{
            let mut sub = Group::try_with_orders(200, 201, &[201, 202])?;
            sub.set_field(201, "x1")?;
            sub
        })?;
        group.add_group(&{
            let mut sub = Group::try_with_orders(200, 201, &[201, 202])?;
            sub.set_field(201, "x2")?;
            sub.set_field(202, "y2")?;
            sub
        })?;
        group.set_field(103, "v1")?;
        group
    })?;
    msg.add_group(&{
        let mut group = Group::try_with_orders(100, 101, &[101, 102, 103])?;
        group.add_group(&{
            let mut sub = Group::try_with_orders(200, 201, &[201, 202])?;
            sub.set_field(201, "x3")?;
            sub
        })?;
        group
    })?;

    // Add groups to trailer.
    msg.with_trailer_mut(|t| {
        t.add_group(&{
            let mut group = Group::try_with_orders(41, 51, &[51, 61])?;
            group.set_field(51, "t10")?;
            group.set_field(61, "t11")?;
            group
        })?;
        t.add_group(&{
            let mut group = Group::try_with_orders(41, 51, &[51, 61])?;
            group.set_field(61, "t21")?;
            group.set_field(51, "t20")?;
            group
        })?;

        Ok::<_, QuickFixError>(())
    })?;

    // Compare FIX text output.
    assert_eq!(
        msg.to_fix_string()?,
        "\
            9=133\u{1}\
            40=2\u{1}\
                50=h10\u{1}60=h11\u{1}70=h12\u{1}\
                60=h21\u{1}70=h22\u{1}\
            \
            100=2\u{1}\
                101=v1\u{1}103=v1\u{1}\
                200=2\u{1}\
                    201=x1\u{1}\
                    201=x2\u{1}202=y2\u{1}\
                200=1\u{1}\
                    201=x3\u{1}\
            \
            41=2\u{1}\
                51=t10\u{1}61=t11\u{1}\
                51=t20\u{1}61=t21\u{1}\
            10=173\u{1}\
            "
    );

    // Check header group getter.
    assert!(msg.clone_header().clone_group(0, 40).is_none());

    let group = msg.clone_header().clone_group(1, 40).unwrap();
    assert_eq!(group.get_field(50).as_deref(), Some("h10"));
    assert_eq!(group.get_field(60).as_deref(), Some("h11"));
    assert_eq!(group.get_field(70).as_deref(), Some("h12"));

    let group = msg.clone_header().clone_group(2, 40).unwrap();
    assert_eq!(group.get_field(50).as_deref(), None);
    assert_eq!(group.get_field(60).as_deref(), Some("h21"));
    assert_eq!(group.get_field(70).as_deref(), Some("h22"));

    assert!(msg.clone_header().clone_group(3, 40).is_none());

    // Check recursive group getters.
    assert!(msg.clone_group(0, 99).is_none());
    assert!(msg.clone_group(1, 99).is_none());

    assert!(msg.clone_group(0, 100).is_none());

    let group = msg.clone_group(1, 100).unwrap();
    assert_eq!(group.get_field(101).as_deref(), Some("v1"));
    assert!(group.clone_group(0, 200).is_none());
    let sub = group.clone_group(1, 200).unwrap();
    assert_eq!(sub.get_field(201).as_deref(), Some("x1"));
    assert_eq!(sub.get_field(202).as_deref(), None);
    let sub = group.clone_group(2, 200).unwrap();
    assert_eq!(sub.get_field(201).as_deref(), Some("x2"));
    assert_eq!(sub.get_field(202).as_deref(), Some("y2"));
    assert_eq!(group.get_field(103).as_deref(), Some("v1"));

    let group = msg.clone_group(2, 100).unwrap();
    let sub = group.clone_group(1, 200).unwrap();
    assert_eq!(sub.get_field(201).as_deref(), Some("x3"));

    assert!(msg.clone_group(3, 100).is_none());

    // Check trailer group getters
    assert!(msg.clone_trailer().clone_group(0, 41).is_none());

    let group = msg.clone_trailer().clone_group(1, 41).unwrap();
    assert_eq!(group.get_field(51).as_deref(), Some("t10"));
    assert_eq!(group.get_field(61).as_deref(), Some("t11"));

    let group = msg.clone_trailer().clone_group(2, 41).unwrap();
    assert_eq!(group.get_field(51).as_deref(), Some("t20"));
    assert_eq!(group.get_field(61).as_deref(), Some("t21"));

    assert!(msg.clone_trailer().clone_group(3, 41).is_none());

    Ok(())
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
        msg.to_fix_string().unwrap(),
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
        msg.to_fix_string().unwrap(),
        "9=74\u{1}35=B\u{1}33=3\u{1}\
         58=Some new library\u{1}\
         58=will be available\u{1}\
         58=soon\u{1}148=Great news\u{1}\
         10=127\u{1}"
    );
}
