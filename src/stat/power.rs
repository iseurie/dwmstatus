use ::std::str::{self, FromStr};

pub struct Power;
impl super::Status for Power {
    fn head(&self) -> String {
        let status = include_str!("/sys/class/power_supply/BAT0/status").trim();
        format!("PW [{}]", status.chars().next().unwrap())
    }
    fn body(&self) -> String {
        let charge_max = usize::from_str(&(include_str!("/sys/class/power_supply/BAT0/charge_full").trim())).unwrap();
        let charge_cur = usize::from_str(&(include_str!("/sys/class/power_supply/BAT0/charge_now").trim())).unwrap();
        (charge_cur * 100 / charge_max).to_string() + "%"
    }
}
