use quickfix::{FieldMap, Message, QuickFixError};
use quickfix_msg40::{
    field_types::{MsgType, Side},
    *,
};

#[test]
fn test_build_order_status_request() -> Result<(), QuickFixError> {
    // `OrderStatusRequest` is a basic message with no sub components.
    // It is a good start to play with.
    let mut obj =
        OrderStatusRequest::try_new("foo".to_string(), "AAPL US Equity".to_string(), Side::Buy)
            .unwrap();

    // Check FIX message contains valid value.
    assert_eq!(
        obj.as_fix_string(),
        "8=FIX.4.0\u{1}9=35\u{1}35=H\u{1}\
         11=foo\u{1}54=1\u{1}55=AAPL US Equity\u{1}\
         10=213\u{1}"
    );

    // Check headers / trailers getters.
    assert_eq!(obj.header().get_msg_type(), MsgType::OrderStatusRequest);
    assert_eq!(obj.trailer().get_check_sum(), "213");

    // Check headers setters and remover
    obj.header_mut().remove_on_behalf_of_comp_id()?;
    assert_eq!(obj.header().get_on_behalf_of_comp_id(), None);

    obj.header_mut()
        .set_on_behalf_of_comp_id("ZeCorp".to_string())?;
    assert_eq!(
        obj.header().get_on_behalf_of_comp_id().as_deref(),
        Some("ZeCorp")
    );

    obj.header_mut().remove_on_behalf_of_comp_id()?;
    assert_eq!(obj.header().get_on_behalf_of_comp_id().as_deref(), None);

    // Check trailers setters and remover
    obj.trailer_mut().remove_signature()?;
    assert_eq!(obj.trailer().get_signature(), None);

    obj.trailer_mut().set_signature("ME".to_string())?;
    assert_eq!(obj.trailer().get_signature().as_deref(), Some("ME"));

    obj.trailer_mut().remove_signature()?;
    assert_eq!(obj.trailer().get_signature(), None);

    // Check getters.
    assert_eq!(obj.get_cl_ord_id(), "foo");
    assert_eq!(obj.get_order_id(), None);
    assert_eq!(obj.get_side(), Side::Buy);

    // Check required setters.
    obj.set_cl_ord_id("bar".to_string())?;
    assert_eq!(obj.get_cl_ord_id(), "bar");

    // Check setters and remover.
    obj.remove_issuer()?;
    assert_eq!(obj.get_issuer(), None);

    obj.set_issuer("BBG".to_string())?;
    assert_eq!(obj.get_issuer().as_deref(), Some("BBG"));

    obj.remove_issuer()?;
    assert_eq!(obj.get_issuer(), None);

    // Trigger recompute checksum and check it.
    let _ = obj.as_fix_string();
    assert_eq!(obj.trailer().get_check_sum(), "198");

    // Convert struct to and from message.
    let msg: Message = obj.into();
    assert_eq!(
        msg.with_header(|h| h.get_field(field_id::MSG_TYPE))
            .as_deref(),
        Some("H")
    );

    let _obj: OrderStatusRequest = msg.into();

    Ok(())
}

#[test]
fn test_build_list_status() -> Result<(), QuickFixError> {
    // `ListStatus` is the simplest message with required groups that can be found.
    let mut obj = ListStatus::try_new("My list".to_string(), 0, 0)?;

    // Check FIX message contains valid value.
    assert_eq!(
        obj.as_fix_string(),
        "8=FIX.4.0\u{1}9=26\u{1}35=N\u{1}\
         66=My list\u{1}82=0\u{1}83=0\u{1}\
         10=237\u{1}"
    );

    // Add some groups and check again string content
    // NB. We clearly see group content if final string + check sorter works correctly 😎.
    obj.add_no_orders(list_status::NoOrders::try_new(
        "Order:10000".to_string(),
        100,
        50,
        18.5,
    )?)?;
    obj.add_no_orders(list_status::NoOrders::try_new(
        "Order:10001".to_string(),
        89,
        75,
        987.4,
    )?)?;
    obj.add_no_orders(list_status::NoOrders::try_new(
        "Order:10018".to_string(),
        5,
        79,
        5.6,
    )?)?;

    assert_eq!(
        obj.as_fix_string(),
        "8=FIX.4.0\u{1}9=133\u{1}35=N\u{1}\
         66=My list\u{1}73=3\u{1}\
         11=Order:10000\u{1}14=100\u{1}84=50\u{1}6=18.5\u{1}\
         11=Order:10001\u{1}14=89\u{1}84=75\u{1}6=987.4\u{1}\
         11=Order:10018\u{1}14=5\u{1}84=79\u{1}6=5.6\u{1}\
         82=0\u{1}83=0\u{1}\
         10=128\u{1}"
    );

    // Read out of bound groups.
    assert!(obj.clone_group_no_orders(0).is_none());
    assert!(obj.clone_group_no_orders(4).is_none());

    // Read some group back.
    let group = obj.clone_group_no_orders(1).unwrap();
    assert_eq!(group.get_cl_ord_id(), "Order:10000");
    assert_eq!(group.get_cum_qty(), 100);
    assert_eq!(group.get_cxl_qty(), 50);
    assert_eq!(group.get_avg_px(), 18.5);

    let group = obj.clone_group_no_orders(2).unwrap();
    assert_eq!(group.get_cl_ord_id(), "Order:10001");
    assert_eq!(group.get_cum_qty(), 89);
    assert_eq!(group.get_cxl_qty(), 75);
    assert_eq!(group.get_avg_px(), 987.4);

    let group = obj.clone_group_no_orders(3).unwrap();
    assert_eq!(group.get_cl_ord_id(), "Order:10018");
    assert_eq!(group.get_cum_qty(), 5);
    assert_eq!(group.get_cxl_qty(), 79);
    assert_eq!(group.get_avg_px(), 5.6);

    Ok(())
}
