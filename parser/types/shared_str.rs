
use crate::types::common::Crc;


pub type SharedStr = Crc<str>;

pub trait SharedStrExt {
    fn to_share(self) -> SharedStr;
}

impl SharedStrExt for String {
    fn to_share(self) -> SharedStr {
        Crc::from(self.into_boxed_str())
    }
}
