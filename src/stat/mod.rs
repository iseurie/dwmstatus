mod time;
mod power;
// TODO: mod wifi;
pub use self::time::*;
pub use self::power::*;

pub trait Status {
    fn head(&self) -> String;
    fn body(&self) -> String;
}
