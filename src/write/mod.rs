mod x11;
mod cli;

pub use self::x11::*;
pub use self::cli::*;

pub trait WriteStatus {
    fn write_status(&mut self, status: &str);
}
