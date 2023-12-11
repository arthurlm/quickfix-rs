#![allow(dead_code)]

use quickfix::*;

pub const MSG_NO_LINES_OF_TEXT: i32 = 33;
pub const MSG_TYPE: i32 = 35;
pub const MSG_TEXT: i32 = 58;
pub const MSG_HEADLINE: i32 = 148;

/// Create new News message.
///
/// This is a super simple message to build.
///
/// Doc: https://www.onixs.biz/fix-dictionary/4.4/msgType_B_66.html
pub fn build_news(headline: &str, lines: &[&str]) -> Result<Message, QuickFixError> {
    let mut msg = Message::new();
    msg.with_header_mut(|h| h.set_field(MSG_TYPE, "B"))??;

    msg.set_field(MSG_HEADLINE, headline)?;
    msg.set_field(MSG_NO_LINES_OF_TEXT, &lines.len().to_string())?; // Not required but always nice

    for line in lines {
        let mut group = Group::try_new(MSG_NO_LINES_OF_TEXT, MSG_TEXT)?;
        group.set_field(MSG_TEXT, &line)?;
        msg.add_group(&group)?;
    }

    Ok(msg)
}
