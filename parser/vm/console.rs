
#[derive(Debug)]
pub struct LoxPrinter {
    buffer: String,
    to_console: bool,
}

impl LoxPrinter {

    pub fn new() -> Self {
        LoxPrinter {
            buffer: String::new(),
            to_console: true,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

}

impl LoxPrinter {

    #[allow(dead_code)]
    pub fn auto_to_console(&mut self, to_console: bool) {
        self.to_console = to_console;
    }

    fn try_flush(&mut self) {
        if self.to_console {
            print!("{}", self.buffer);
            self.buffer.clear();
        }
    }

    #[allow(dead_code)]
    pub fn flush_to_console(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        print!("{}", self.buffer);
        self.buffer.clear();
    }

    #[allow(dead_code)]
    pub fn print(&mut self, msg: &str) {
        self.buffer.push_str(msg);
        self.try_flush();
    }

    #[allow(dead_code)]
    pub fn println(&mut self, msg: &str) {
        self.buffer.push_str(msg);
        self.buffer.push_str("\n");
        self.try_flush();
    }

    #[allow(dead_code)]
    pub fn take(&mut self) -> String {
        let ret = self.buffer.clone();
        self.buffer.clear();
        ret
    }

}
