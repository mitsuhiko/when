use anyhow::bail;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use clap::Parser;
use console::style;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use crate::location::{find_zone, ZoneKind, ZoneRef};
use crate::parser::Expr;

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
#[clap(version = clap::crate_version!())]
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

    /// the input expression to evaluate.
    ///
    /// If this is not supplied then "now" is assumed to return the time
    /// in the current timezone.
    expr: Option<String>,
}

fn print_date(date: DateTime<Tz>, zone: ZoneRef) {
    let adjusted = date.with_timezone(&zone.tz());
    println!("time: {}", style(adjusted.format("%H:%M:%S")).bold().cyan());
    println!(
        "date: {} ({})",
        style(adjusted.format("%Y-%m-%d")).yellow(),
        style(adjusted.format("%A")),
    );
    if zone.kind() != ZoneKind::Timezone {
        print!("location: {}", style(zone.name()).green());
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
    println!(
        "zone: {} ({})",
        style(zone.tz().name()).underlined(),
        adjusted.format("%z")
    );
}

pub struct JsonLocation<'a>(&'a ZoneRef);

impl<'a> Serialize for JsonLocation<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut m = serializer.serialize_map(None)?;
        m.serialize_entry("name", self.0.name())?;
        if let Some(admin_code) = self.0.admin_code() {
            m.serialize_entry("admin_code", &admin_code)?;
        }
        if let Some(country) = self.0.country() {
            m.serialize_entry("country", &country)?;
        }
        m.end()
    }
}

pub struct JsonEntry<'a>(&'a DateTime<Tz>, &'a ZoneRef);

impl<'a> Serialize for JsonEntry<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut m = serializer.serialize_map(None)?;
        m.serialize_entry("datetime", &self.0)?;
        m.serialize_entry("timezone", self.1.tz().name())?;
        if self.1.kind() != ZoneKind::Timezone {
            m.serialize_entry("location", &JsonLocation(self.1))?;
        }
        m.end()
    }
}

pub fn execute() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.colors.as_deref() {
        None | Some("auto") => {}
        Some("always") => console::set_colors_enabled(true),
        Some("never") => console::set_colors_enabled(false),
        Some(other) => bail!("unknown value for --colors ({})", other),
    };

    let expr = Expr::parse(cli.expr.as_deref().unwrap_or("now"))?;
    let zone_ref = expr.location().unwrap_or("local");
    let from_zone = find_zone(&zone_ref)?;
    let now = Utc::now().with_timezone(&from_zone.tz());
    let from = expr.apply(now);

    let mut to_info = vec![];

    for to_zone_ref in expr.to_locations() {
        let to_zone = find_zone(to_zone_ref)?;
        let to = from.with_timezone(&to_zone.tz());
        to_info.push((to, to_zone));
    }

    if to_info.is_empty() {
        if let Ok(to_zone) = find_zone("local") {
            if to_zone.tz().name() != from_zone.tz().name() {
                to_info.push((from.with_timezone(&to_zone.tz()), to_zone));
            }
        }
    }

    if cli.json {
        let mut entries = vec![JsonEntry(&from, &from_zone)];
        for (date, loc) in to_info.iter() {
            entries.push(JsonEntry(date, loc));
        }
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
    } else if cli.short {
        println!("{} ({})", from.format("%Y-%m-%d %H:%M:%S %z"), from_zone);
        for (date, loc) in to_info.iter() {
            println!("{} ({})", date.format("%Y-%m-%d %H:%M:%S %z"), loc);
        }
    } else {
        print_date(from, from_zone);
        for (to, to_zone) in to_info {
            println!();
            print_date(to, to_zone);
        }
    }

    Ok(())
}
