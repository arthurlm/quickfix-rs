use std::cmp::Ordering;

use crate::field::{BEGINSTRING, BODYLENGTH, CHECKSUM, MSGTYPE, SIGNATURE, SIGNATURELENGTH};

#[derive(Clone, Debug)]
pub enum CompareMode {
    Header,
    Trailer,
    Normal,
    Group,
}

#[derive(Clone, Debug)]
pub struct MessageOrder {
    pub compare_mode: CompareMode,
    pub largest: i32,
    pub group_order: Option<Vec<i32>>,
}

impl MessageOrder {
    pub fn header() -> Self {
        Self {
            compare_mode: CompareMode::Header,
            largest: 0,
            group_order: None,
        }
    }

    pub fn trailer() -> Self {
        Self {
            compare_mode: CompareMode::Trailer,
            largest: 0,
            group_order: None,
        }
    }
    pub fn normal() -> Self {
        Self {
            compare_mode: CompareMode::Normal,
            largest: 0,
            group_order: None,
        }
    }

    pub fn group(order: Vec<i32>) -> Self {
        Self {
            compare_mode: CompareMode::Group,
            largest: *order.iter().max().unwrap_or(&0i32),
            group_order: Some(order),
        }
    }

    pub fn compare(&self, x: i32, y: i32) -> std::cmp::Ordering {
        match self.compare_mode {
            CompareMode::Group => {
                self.compare_group(x, y, self.group_order.as_ref().unwrap(), self.largest)
            }
            CompareMode::Header => self.compare_header(x, y),
            CompareMode::Trailer => self.compare_trailer(x, y),
            CompareMode::Normal => x.cmp(&y),
        }
    }

    fn compare_group(&self, x: i32, y: i32, order: &[i32], largest: i32) -> std::cmp::Ordering {
        if x <= largest && y <= largest {
            let ix = order.get(x as usize).copied().unwrap_or(0);
            let iy = order.get(y as usize).copied().unwrap_or(0);

            if ix == 0 && iy == 0 {
                x.cmp(&y)
            } else if ix == 0 {
                Ordering::Greater
            } else if iy == 0 {
                return Ordering::Less;
            } else {
                ix.cmp(&iy)
            }
        } else if x <= largest {
            Ordering::Less
        } else if y <= largest {
            Ordering::Greater
        } else {
            x.cmp(&y)
        }
    }
    fn compare_header(&self, x: i32, y: i32) -> std::cmp::Ordering {
        let ordered_x = self.get_header_position(x);
        let ordered_y = self.get_header_position(y);
        match (ordered_x, ordered_y) {
            (Some(px), Some(py)) => px.cmp(&py),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => x.cmp(&y),
        }
    }

    fn compare_trailer(&self, x: i32, y: i32) -> std::cmp::Ordering {
        if x == CHECKSUM {
            return Ordering::Greater;
        } else if y == CHECKSUM {
            return Ordering::Less;
        }

        let ordered_x = self.get_trailer_position(x);
        let ordered_y = self.get_trailer_position(y);
        match (ordered_x, ordered_y) {
            (Some(px), Some(py)) => px.cmp(&py),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => x.cmp(&y),
        }
    }
    fn get_header_position(&self, field_id: i32) -> Option<i32> {
        match field_id {
            BEGINSTRING => Some(1),
            BODYLENGTH => Some(2),
            MSGTYPE => Some(3),
            _ => None,
        }
    }

    fn get_trailer_position(&self, field_id: i32) -> Option<i32> {
        match field_id {
            SIGNATURELENGTH => Some(1),
            SIGNATURE => Some(2),
            _ => None,
        }
    }
}
