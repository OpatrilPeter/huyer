// hides console window on Windows
#![cfg_attr(feature = "pure-gui", windows_subsystem = "windows")]

use anyhow::{bail, Error as AnyError};
use clap::Parser;
use scan_fmt::scan_fmt;
use std::{thread::sleep, time::Duration};
use time::OffsetDateTime;

macro_rules! log {
    ($($tokens:tt)+) => {
        #[cfg(not(feature = "pure-gui"))]
        {
            println!($($tokens)+);
        }
    }
}

mod notify;

fn parse_timespec(input: &str, current_time: OffsetDateTime) -> Result<Duration, AnyError> {
    if input.contains(':') {
        let (hours, minutes) = scan_fmt!(input, "{}:{}", u8, u8)?;

        let desired_time = current_time
            .replace_hour(hours)?
            .replace_minute(minutes)?
            .replace_second(0)?
            .replace_millisecond(0)?;

        if desired_time <= current_time {
            bail!("Desired time {} is in the past", timespec_fmt(desired_time));
        }

        return Duration::try_from(desired_time - current_time).map_err(Into::into);
    } else {
        if input.ends_with("s") {
            let x = input[..input.len() - 1].parse::<u64>()?;
            return Ok(Duration::from_secs(x));
        } else if input.ends_with("m") {
            let x = input[..input.len() - 1].parse::<u64>()?;
            return Ok(Duration::from_secs(x * 60));
        } else if input.ends_with("h") {
            let x = input[..input.len() - 1].parse::<u64>()?;
            return Ok(Duration::from_secs(x * 60 * 60));
        } else {
            if input.parse::<u64>().is_ok() {
                bail!("Missing suffix");
            } else if input.is_empty() {
                bail!("No time specified");
            }
            else {
                bail!("Unknown format");
            }
        }
    }
}

fn parse_current_timespec(input: &str) -> Result<Duration, AnyError> {
    parse_timespec(input, OffsetDateTime::now_local()?)
}

/// Pretty printing the time format.
/// In our domain we only care about current day time specification of limited precision
fn timespec_fmt(timespec: OffsetDateTime) -> String {
    let (h, m, _) = timespec.time().as_hms();
    format!("{h:02}:{m:02}")
}

#[derive(Clone, Parser)]
#[command(author, version, about)]
struct Args {
    /// Definition of when the alarm should be triggered,
    /// either as time interval or a clock time.
    ///
    /// In interval mode, units (h,m,s) must be present as a suffix.
    /// In clock mode, hh:mm format must be used.
    #[arg(value_parser = parse_current_timespec)]
    timespec: Option<Duration>,
    /// Message to display once alarm is triggered.
    #[arg(short, long, default_value = "Time is up.")]
    message: String,
}

fn main() {
    let mut args = Args::parse();

    let Some(timespec) = args.timespec.or_else(|| {
        notify::ask(&mut args);
        args.timespec
    }) else {
        log!("No alarm set.");
        return;
    };

    let target_time = OffsetDateTime::now_local().unwrap() + timespec;

    log!("Sleeping until {} ...", timespec_fmt(target_time));
    sleep(timespec);
    notify::alarm(target_time, &args.message);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_timespec() {
        let current_time = time::macros::datetime!(2022-01-01 10:00:00+0000);

        assert_eq!(
            parse_timespec("123s", current_time).ok(),
            Some(Duration::from_secs(123))
        );
        assert_eq!(
            parse_timespec("123m", current_time).ok(),
            Some(Duration::from_secs(123 * 60))
        );
        assert_eq!(
            parse_timespec("123h", current_time).ok(),
            Some(Duration::from_secs(123 * 60 * 60))
        );

        assert!(parse_timespec("123", current_time).is_err());
        assert!(parse_timespec("s", current_time).is_err());
        assert!(parse_timespec("-12s", current_time).is_err());
        assert!(parse_timespec("", current_time).is_err());
        assert!(parse_timespec("asfaf", current_time).is_err());

        assert_eq!(
            parse_timespec("10:10", current_time).ok(),
            Some(Duration::from_secs(10 * 60))
        );
        assert_eq!(
            parse_timespec("10:05", current_time).ok(),
            Some(Duration::from_secs(5 * 60))
        );
        assert!(parse_timespec("9:02", current_time).is_err());
    }
}
