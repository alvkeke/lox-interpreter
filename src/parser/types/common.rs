

pub type Result<T> = std::result::Result<T, String>;

pub type Crc<T> = std::rc::Rc<T>;

pub type SharedStr = Crc<str>;

#[inline]
pub fn shared_str_from(s: String) -> SharedStr {
    Crc::from(s.into_boxed_str())
}

