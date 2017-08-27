use ::std::path::{Path, PathBuf};
use ::std::str::{self, FromStr};
use ::std::fs::File;
use ::std::io::Read;

static PATH_BAT: &'static str = "/sys/class/power_supply/BAT0/";

pub struct Power;
impl super::Status for Power {
    fn head(&self) -> String {
        let mut status_path = PathBuf::from(PATH_BAT);
        status_path.push(Path::new("status"));
        let mut f_status = File::open(status_path.as_path())
            .expect("cannot open BAT0 property file");
        let mut buf_status = [0u8];
        f_status.read(&mut buf_status).unwrap_or_default();
        format!("PW [{}]", buf_status[0] as char)
    }

    fn body(&self) -> String {
        const BUF_CHARGE_LEN: u8 = 16;
        let mut buf_charge = [0u8; BUF_CHARGE_LEN as usize];
        let mut buf_read_charge = |prop: &str| -> usize {
            let mut pb = PathBuf::from(PATH_BAT);
            pb.push(prop);
            let errmsg = format!("read BAT0 property '{}'", prop);
            let mut charge_file = File::open(pb.as_path()).expect(&errmsg);
            let digitc = charge_file.read(&mut buf_charge).expect(&errmsg);
            let ret = usize::from_str(
                    str::from_utf8(&buf_charge[..digitc])
                        .expect(&errmsg).trim()
            ).expect(&errmsg);
            buf_charge = [0u8; BUF_CHARGE_LEN as usize];
            ret
        };
        let charge_max = buf_read_charge("charge_full");
        let charge_cur = buf_read_charge("charge_now");
        (charge_cur * 100 / charge_max).to_string() + "%"
    }
}
