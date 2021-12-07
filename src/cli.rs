use argh::FromArgs;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::location::{find_zone, ZoneKind, ZoneRef};
use crate::parser::Expr;

/// A small utility to convert times from the command line.
#[derive(FromArgs)]
struct Cli {
    /// use short output.
    #[argh(switch, short = 's')]
    short: bool,

    /// the input to convert
    #[argh(positional)]
    input: String,
}

fn print_date(date: DateTime<Tz>, zone: ZoneRef) {
    let adjusted = date.with_timezone(&zone.tz());
    println!("time: {}", adjusted.format("%H:%M:%S"));
    println!("date: {}", adjusted.format("%Y-%m-%d"));
    if zone.kind() != ZoneKind::Timezone {
        print!("location: {}", zone.name());
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
    println!("zone: {} ({})", zone.tz().name(), adjusted.format("%z"));
}

pub fn execute() -> Result<(), anyhow::Error> {
    let cli: Cli = argh::from_env();

    let expr = Expr::parse(&cli.input)?;
    let zone_ref = expr.location().unwrap_or("local");
    let from_zone = find_zone(&zone_ref)?;
    let now = Utc::now().with_timezone(&from_zone.tz());
    let from = expr.apply(now);

    let to = if let Some(to_zone_ref) = expr.to_location() {
        let to_zone = find_zone(to_zone_ref)?;
        let to = from.with_timezone(&to_zone.tz());
        Some((to, to_zone))
    } else {
        None
    };

    if cli.short {
        let date = to.as_ref().map(|x| x.0).unwrap_or(from);
        println!("{}", date.format("%Y-%m-%d %H:%M:%S %z"));
    } else {
        print_date(from, from_zone);
        if let Some((to, to_zone)) = to {
            println!();
            print_date(to, to_zone);
        }
    }

    Ok(())
}
