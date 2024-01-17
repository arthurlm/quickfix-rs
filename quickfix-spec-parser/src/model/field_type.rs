use std::str::FromStr;

use crate::FixSpecError;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum FieldType {
    // ⬆️ Add in FIX 4.0
    Char,
    Int,
    Float,
    Time,
    Date,
    Length,
    Data,
    // ⬆️ Add in FIX 4.1
    MonthYear,
    DayOfMonth,
    // ⬆️ Add in FIX 4.2
    String,
    Price,
    Amount,
    Quantity,
    Currency,
    MultipleValueString,
    Exchange,
    UtcTimeStamp,
    Boolean,
    LocalMarketDate,
    PriceOffset,
    UtcDate,
    UtcTimeOnly,
    // ⬆️ Add in FIX 4.3
    SequenceNumber,
    NumberInGroup,
    Percentage,
    Country,
    // ⬆️ Add in FIX 4.4
    UtcDateOnly,
    // ⬆️ Add in FIX 5.0
    MultipleCharValue,
    MultipleStringValue,
    TzTimeOnly,
    TzTimestamp, // How can a timestamp include a timezone 🤨
    // ⬆️ Add in FIX 5.0 SP1
    XmlData,
    // ⬆️ Add in FIX 5.0 SP2
    Language,
    TagNumber,
    XidRef,
    Xid,
    LocalMarketTime,
}

impl FieldType {
    pub const fn as_static_str(&self) -> &'static str {
        match self {
            Self::Char => "CHAR",
            Self::Int => "INT",
            Self::Float => "FLOAT",
            Self::Time => "TIME",
            Self::Date => "DATE",
            Self::Length => "LENGTH",
            Self::Data => "DATA",
            Self::MonthYear => "MONTHYEAR",
            Self::DayOfMonth => "DAYOFMONTH",
            Self::String => "STRING",
            Self::Price => "PRICE",
            Self::Amount => "AMT",
            Self::Quantity => "QTY",
            Self::Currency => "CURRENCY",
            Self::MultipleValueString => "MULTIPLEVALUESTRING",
            Self::Exchange => "EXCHANGE",
            Self::UtcTimeStamp => "UTCTIMESTAMP",
            Self::Boolean => "BOOLEAN",
            Self::LocalMarketDate => "LOCALMKTDATE",
            Self::PriceOffset => "PRICEOFFSET",
            Self::UtcDate => "UTCDATE",
            Self::UtcTimeOnly => "UTCTIMEONLY",
            Self::SequenceNumber => "SEQNUM",
            Self::NumberInGroup => "NUMINGROUP",
            Self::Percentage => "PERCENTAGE",
            Self::Country => "COUNTRY",
            Self::UtcDateOnly => "UTCDATEONLY",
            Self::MultipleCharValue => "MULTIPLECHARVALUE",
            Self::MultipleStringValue => "MULTIPLESTRINGVALUE",
            Self::TzTimeOnly => "TZTIMEONLY",
            Self::TzTimestamp => "TZTIMESTAMP",
            Self::XmlData => "XMLDATA",
            Self::Language => "LANGUAGE",
            Self::TagNumber => "TAGNUM",
            Self::XidRef => "XIDREF",
            Self::Xid => "XID",
            Self::LocalMarketTime => "LOCALMKTTIME",
        }
    }
}

impl FromStr for FieldType {
    type Err = FixSpecError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "CHAR" => Ok(Self::Char),
            "INT" => Ok(Self::Int),
            "FLOAT" => Ok(Self::Float),
            "TIME" => Ok(Self::Time),
            "DATE" => Ok(Self::Date),
            "LENGTH" => Ok(Self::Length),
            "DATA" => Ok(Self::Data),
            "MONTHYEAR" => Ok(Self::MonthYear),
            "DAYOFMONTH" => Ok(Self::DayOfMonth),
            "STRING" => Ok(Self::String),
            "PRICE" => Ok(Self::Price),
            "AMT" => Ok(Self::Amount),
            "QTY" => Ok(Self::Quantity),
            "CURRENCY" => Ok(Self::Currency),
            "MULTIPLEVALUESTRING" => Ok(Self::MultipleValueString),
            "EXCHANGE" => Ok(Self::Exchange),
            "UTCTIMESTAMP" => Ok(Self::UtcTimeStamp),
            "BOOLEAN" => Ok(Self::Boolean),
            "LOCALMKTDATE" => Ok(Self::LocalMarketDate),
            "PRICEOFFSET" => Ok(Self::PriceOffset),
            "UTCDATE" => Ok(Self::UtcDate),
            "UTCTIMEONLY" => Ok(Self::UtcTimeOnly),
            "SEQNUM" => Ok(Self::SequenceNumber),
            "NUMINGROUP" => Ok(Self::NumberInGroup),
            "PERCENTAGE" => Ok(Self::Percentage),
            "COUNTRY" => Ok(Self::Country),
            "UTCDATEONLY" => Ok(Self::UtcDateOnly),
            "MULTIPLECHARVALUE" => Ok(Self::MultipleCharValue),
            "MULTIPLESTRINGVALUE" => Ok(Self::MultipleStringValue),
            "TZTIMEONLY" => Ok(Self::TzTimeOnly),
            "TZTIMESTAMP" => Ok(Self::TzTimestamp),
            "XMLDATA" => Ok(Self::XmlData),
            "LANGUAGE" => Ok(Self::Language),
            "TAGNUM" => Ok(Self::TagNumber),
            "XIDREF" => Ok(Self::XidRef),
            "XID" => Ok(Self::Xid),
            "LOCALMKTTIME" => Ok(Self::LocalMarketTime),
            x => Err(FixSpecError::InvalidContent(format!(
                "unknown field type: {x}"
            ))),
        }
    }
}
