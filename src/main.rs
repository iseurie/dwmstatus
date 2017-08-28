extern crate time;
extern crate x11;

mod stat;
mod write;

use stat::{Time, Power, Status};
use write::WriteStatus;
use std::time::Duration;

static DELIMITER_STAT: &'static str = " |";

fn main() {
    let stats: [&Status; 2] = [ &Power, &Time ];
    
    let stdout = ::std::io::stdout();
    let mut cli_writer = write::Cli::new(&stdout);
    let mut opt_x11_writer = write::X11::new();
    let writer: &mut WriteStatus;
    if opt_x11_writer.is_some() {
        writer = opt_x11_writer.as_mut().unwrap();
    } else {
        writer = &mut cli_writer;
    }
    loop {
        let status = status_line(&stats);
        writer.write_status(&status);
        ::std::thread::sleep(Duration::from_millis(1000));
    }
}

fn status_line(stats: &[&Status]) -> String {
    let mut ret = String::new();
    let mut iter_stat = stats.iter();
    let first = iter_stat.next().unwrap();
    ret += &(DELIMITER_STAT.to_owned().trim().to_string()
        + &first.head() + ": " + &first.body()
    ).as_str();
    for stat in iter_stat {
        ret += (DELIMITER_STAT.to_owned()
            + &stat.head() + ": " + &stat.body()
        ).as_str();
    }
    ret
}
