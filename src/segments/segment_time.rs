extern crate time;

// #[cfg(feature = "chrono")] use chrono::Local;
// #[cfg(feature = "chrono")] use chrono::prelude::*;
#[cfg(feature = "chrono")] use std::fmt::Write;
use {Powerline, Segment, Shell};

pub fn segment_time(p: &mut Powerline) {
    let (bg, fg) = (p.theme.time_bg, p.theme.time_fg);
    if p.shell == Shell::Bare {
        #[cfg(feature = "chrono")]
        {
            let t = time::now();

            let mut formatted = String::with_capacity(2 + 1 + 2 + 1 + 2);
             write!(formatted, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}", t.tm_year+1900, t.tm_mon, t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec, t.tm_nsec/1000000).unwrap();
            // write!(formatted, "{:02}:{:02}:{:02}.{:03}", t.tm_hour, t.tm_min, t.tm_sec, t.tm_nsec/1000000).unwrap();

            // We don't want to dont_escape() here
            p.segments.push(Segment::new(bg, fg, formatted));
        }
        return;
    }
    if p.shell == Shell::Bash {
        #[cfg(feature = "chrono")]
        {
            let t = time::now();

            let mut formatted = String::with_capacity(2 + 1 + 2 + 1 + 2);
            write!(formatted, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}", t.tm_year+1900, t.tm_mon, t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec, t.tm_nsec/1000000).unwrap();

            // We don't want to dont_escape() here
            p.segments.push(Segment::new(bg, fg, formatted));
        }
        return;

    }
    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => unreachable!(),
        Shell::Zsh  => "%@"
    }).dont_escape())
}
