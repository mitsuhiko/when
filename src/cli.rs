use anyhow::bail;
use argh::FromArgs;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use console::style;

use crate::location::{find_zone, ZoneKind, ZoneRef};
use crate::parser::Expr;

/// A small utility to convert times from the command line.
#[derive(FromArgs)]
struct Cli {
    /// use short output.
    #[argh(switch, short = 's')]
    short: bool,

    /// controls when to use colors. Choices are `auto`, `never`, `always`.
    #[argh(option, long = "colors")]
    colors: Option<String>,

    /// the input expression to evaluate
    #[argh(positional)]
    expr: String,
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

pub fn execute() -> Result<(), anyhow::Error> {
    let cli: Cli = argh::from_env();

    match cli.colors.as_deref() {
        None | Some("auto") => {}
        Some("always") => console::set_colors_enabled(true),
        Some("never") => console::set_colors_enabled(false),
        Some(other) => bail!("unknown value for --colors ({})", other),
    };

    let expr = Expr::parse(&cli.expr)?;
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

    if cli.short {
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
