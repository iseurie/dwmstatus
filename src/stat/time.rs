pub struct Time;
impl super::Status for Time {
    fn head(&self) -> String {
        String::from("TM")
    }

    fn body(&self) -> String {
        let now = ::time::now();
        let offset = ::time::Duration::seconds(now.tm_utcoff as i64);
        let mut abbv_year = now.tm_year;
        for i in 2..9 {
            abbv_year %= 1 * 10i32.pow(i);
        }
        format!("{:03}{:02}::{:02}/{:02}/{:02}::{:02}:{:02}:{:02}",
                offset.num_hours() - 1, offset.num_minutes() % 60,
                abbv_year, now.tm_mon, now.tm_mday,
                now.tm_hour, now.tm_min, now.tm_sec
        )
    }
}
