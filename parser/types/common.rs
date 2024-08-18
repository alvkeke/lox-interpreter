

pub type Result<T> = std::result::Result<T, String>;

pub type Crc<T> = std::rc::Rc<T>;


#[macro_export]
macro_rules! dbg_format {
    ($fmt:expr) => {{
        format!(
            "[{}:{}] {}",
            file!(),
            line!(),
            $fmt
        )
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        format!(
            "[{}:{}] {}",
            file!(),
            line!(),
            format!($fmt, $($arg)*)
        )
    }};
}

#[macro_export]
macro_rules! dbg_println {
    ($fmt:expr) => {{
        println!(
            "[{}:{}] {}",
            file!(),
            line!(),
            $fmt
        )
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        println!(
            "[{}:{}] {}",
            file!(),
            line!(),
            println!($fmt, $($arg)*)
        )
    }};
}
