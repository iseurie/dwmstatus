use ::std::io::{self, Write};

pub struct Cli<'l> {
    writer: io::StdoutLock<'l>
}

impl<'l> Cli<'l> {
    pub fn new(stdout: &'l io::Stdout) -> Self {
        Self {
            writer: stdout.lock()
        }
    }
}

impl<'l> super::WriteStatus for Cli<'l> {
    fn write_status(&mut self, status: &str) {
        self.writer.write(&[b'\r']).unwrap();
        self.writer.write(status.as_bytes()).unwrap();
    }
}
