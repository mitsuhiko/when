use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut out = fs::File::create(out_dir.join("locations.rs")).unwrap();

    writeln!(out, "pub static COUNTRIES: &[(&str, &str)] = &[",).unwrap();
    for line in BufReader::new(fs::File::open("data/countries.txt").unwrap()).lines() {
        let line = line.unwrap();
        let pieces = line.split('\t').collect::<Vec<_>>();
        writeln!(out, "  ({:?}, {:?}),", pieces[0], pieces[1]).unwrap();
    }
    writeln!(out, "];").unwrap();

    writeln!(out, "pub static LOCATIONS: &[Location] = &[",).unwrap();
    for line in BufReader::new(fs::File::open("data/locations.txt").unwrap()).lines() {
        let line = line.unwrap();
        let pieces = line.split('\t').collect::<Vec<_>>();
        writeln!(
            out,
            "  Location {{ name: {:?}, aliases: &{:?}, country: {:?}, admin_code: {:?}, kind: ZoneKind::{}, tz: Tz::{} }},",
            pieces[0],
            if pieces[1].is_empty() { vec![] } else { pieces[1].split(';').collect::<Vec<_>>() },
            pieces[2],
            if pieces[3].is_empty() { None } else { Some(pieces[3]) },
            match pieces[4] {
                "city" => "City",
                "airport" => "Airport",
                "division" => "Division",
                _ => unreachable!(),
            },
            pieces[5].replace(" ", "_").replace("-", "").replace("/", "__"),
        ).unwrap();
    }
    writeln!(out, "];").unwrap();
}
