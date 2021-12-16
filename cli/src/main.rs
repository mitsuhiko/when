use std::fmt;

use anyhow::bail;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use clap::Parser;
use console::style;

use libwhen::{get_time_of_day, InputExpr, LocationKind, TimeAtLocation};

/// A small utility to convert times from the command line.
///
/// When takes a time and date expression and helps converting it into
/// different timezones.  If no arguments are supplied the current time
/// in the current location is returned.
///
/// The basic syntax for the expression is "time_spec [in location_spec]".
/// Translations between locations is done by using the "->" operator.
///
/// For instance "2pm in vie -> yyz" takes 14:00 in vienna time and
/// translates it to toronto (airport).  It then prints out both
/// timestamps on stdout with additional information.
///
/// For more examples see https://github.com/mitsuhiko/when
#[derive(Parser)]
#[clap(version = clap::crate_version!(), max_term_width = 100)]
struct Cli {
    /// use short output.
    ///
    /// When short output is enabled one line per timezone is returned.
    #[clap(short = 's', long = "short")]
    short: bool,

    /// controls when to use colors. Choices are `auto`, `never`, `always`.
    #[clap(long = "colors")]
    colors: Option<String>,

    /// output in JSON format.
    #[clap(long = "json")]
    json: bool,

    /// returns a list of all known IANA/Olson timezones.
    #[clap(long = "list-timezones")]
    list_timezones: bool,

    /// the input expression to evaluate.
    ///
    /// If this is not supplied then "now" is assumed to return the time
    /// in the current timezone.
    expr: Option<String>,
}

pub struct ZoneOffset(DateTime<Tz>);

impl fmt::Display for ZoneOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abbrev = self.0.format("%Z").to_string();
        if abbrev.chars().all(|x| x.is_ascii_alphabetic()) {
            write!(f, "{}; {}", abbrev, self.0.format("%z"))?
        } else {
            write!(f, "{}", self.0.format("%z"))?
        }
        Ok(())
    }
}

fn print_date(tod: &TimeAtLocation, now: DateTime<Utc>) {
    let date = tod.datetime();
    let zone = tod.zone();
    let adjusted = date.with_timezone(&zone.tz());
    println!(
        "time: {} ({}; {})",
        style(adjusted.format("%H:%M:%S")).bold().cyan(),
        tod.relative_to_human(now),
        get_time_of_day(adjusted),
    );
    println!(
        "date: {} ({})",
        style(adjusted.format("%Y-%m-%d")).yellow(),
        style(adjusted.format("%A")),
    );
    println!(
        "zone: {} ({})",
        style(zone.tz().name()).underlined(),
        ZoneOffset(adjusted),
    );
    if zone.kind() != LocationKind::Timezone {
        print!("location: {}", style(zone.name()).bold());
        print!(" (");
        let mut with_code = false;
        if let Some(code) = zone.admin_code() {
            print!("{}", code);
            with_code = true;
        }
        if let Some(country) = zone.country() {
            if with_code {
                print!("; ");
            }
            print!("{}", country);
        }
        print!(")");
        println!();
    }
}

fn list_timezones() -> Result<(), anyhow::Error> {
    let now = Utc::now();
    let mut zone_list = Vec::new();
    for zone in chrono_tz::TZ_VARIANTS {
        let there = now.with_timezone(&zone);
        zone_list.push((zone, there));
    }
    zone_list.sort_by_key(|x| x.0.name());

    for (zone, there) in zone_list {
        println!("{} ({})", zone.name(), ZoneOffset(there));
    }

    Ok(())
}

pub fn execute() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.colors.as_deref() {
        None | Some("auto") => {}
        Some("always") => console::set_colors_enabled(true),
        Some("never") => console::set_colors_enabled(false),
        Some(other) => bail!("unknown value for --colors ({})", other),
    };

    if cli.list_timezones {
        return list_timezones();
    }

    let expr = InputExpr::parse(cli.expr.as_deref().unwrap_or("now"))?;
    let timestamps = expr.process()?;

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&timestamps).unwrap());
    } else if cli.short {
        for t in timestamps.iter() {
            println!(
                "{} ({})",
                t.datetime().format("%Y-%m-%d %H:%M:%S %z"),
                t.zone()
            );
        }
    } else {
        let now = Utc::now();
        for (idx, t) in timestamps.iter().enumerate() {
            if idx > 0 {
                println!();
            }
            print_date(t, now);
        }
    }

    Ok(())
}

fn main() {
    match execute() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
}
