use ::std::path::{Path, PathBuf};
use ::std::str::{self, FromStr};
use ::std::fs::File;
use ::std::io::Read;
use ::std::fmt::Debug;

static BAT_PATH: &'static str = "/sys/class/power_supply/BAT0/";
pub struct Power;

fn read_bat_prop<T: FromStr> (name: &str, buf: &mut [u8]) -> T where
        T: FromStr,
        <T as FromStr>::Err: Debug {
    let mut pb = PathBuf::from(BAT_PATH);
    pb.push(name);
    let errmsg = format!("cannot read BAT0 property `{}`", name);
    let mut f = File::open(pb.as_path()).expect(&errmsg);
    let flen = f.read(buf).expect(&errmsg);
    let ret = unsafe {
        T::from_str(
                str::from_utf8_unchecked(&buf[..flen]).trim()
        ).expect(&errmsg)
    };
    for b in buf.iter_mut().take(flen) { *b = 0; }
    ret
}

impl super::Status for Power {
    fn head(&self) -> String {
        let mut buf = [0u8; 32];
        let status: String = read_bat_prop("status", &mut buf[0..1]);
        let mut ret = String::from("PW [") + &status[0..1];
        (match status.as_str() {
            "D" => {
                let charge: usize = read_bat_prop("charge_now", &mut buf);
                let consumption: usize = read_bat_prop("current_now", &mut buf);
                let secs = charge as f64 / consumption as f64 * 3600f64;
                ret + &(String::from("; ")
                    + &(secs as u16 / 3600).to_string()
                    + ":" + &(format!("{:02}", (secs / 60f64) as u16 % 60))
                )
            },
            _ => ret
        } + "]")
    }

    fn body(&self) -> String {
        let mut buf = [0u8; 16];
        let charge_max: usize = read_bat_prop("charge_full", &mut buf);
        let charge_cur: usize = read_bat_prop("charge_now", &mut buf);
        (charge_cur * 100 / charge_max).to_string() + "%"
    }
}
