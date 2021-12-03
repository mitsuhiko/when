use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct LocationRecord<'a> {
    name: &'a str,
    is_important: bool,
    population: i64,
    country: &'a str,
    admin_code: &'a str,
    tz: &'a str,
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut out = fs::File::create(out_dir.join("locations.rs")).unwrap();
    let cities_raw = fs::read_to_string("data/cities15000.txt").unwrap();
    let countries_raw = fs::read_to_string("data/countryInfo.txt").unwrap();
    let mut locations = Vec::new();

    // countries
    let mut countries = HashMap::new();
    for line in countries_raw.lines() {
        if line.starts_with('#') {
            continue;
        }
        let pieces = line.split('\t').collect::<Vec<_>>();
        if pieces.len() < 3 {
            continue;
        }
        countries.insert(pieces[0], pieces[4]);
    }

    // find cities
    for line in cities_raw.lines() {
        if line.starts_with('#') {
            continue;
        }
        let pieces = line.split('\t').collect::<Vec<_>>();
        //let name = pieces[1];
        let name = pieces[2]; // this is the ascii name
        let class = pieces[7];
        let country = pieces[8];
        let admin_code = pieces[10];
        let tz = pieces[17];
        let is_important = class == "PPLC";
        let population = pieces[14].parse::<i64>().unwrap();
        if name.contains('(') {
            continue;
        }
        locations.push(LocationRecord {
            name,
            is_important,
            population,
            country,
            admin_code,
            tz,
        })
    }

    locations.sort_by_cached_key(|x| {
        (
            x.name.to_ascii_lowercase(),
            if x.is_important { 0 } else { 1 },
            if x.country == "US" { 0 } else { 1 },
            x.population,
        )
    });

    writeln!(
        out,
        "pub static COUNTRIES: [(&'static str, &'static str); {}] = [",
        countries.len(),
    )
    .unwrap();
    let mut countries = countries.into_iter().collect::<Vec<_>>();
    countries.sort();
    for (code, country) in countries {
        writeln!(out, "  ({:?}, {:?}),", code, country).unwrap();
    }
    writeln!(out, "];").unwrap();
    writeln!(
        out,
        "pub static LOCATIONS: [Location; {}] = [",
        locations.len(),
    )
    .unwrap();
    for rec in &locations {
        writeln!(
            out,
            "  Location {{ name: {:?}, country: {:?}, admin_code: {:?}, kind: ZoneKind::City, tz: Tz::{} }},",
            rec.name,
            rec.country,
            if rec.admin_code.is_empty() || rec.admin_code.chars().any(|x| x.is_numeric()) { None } else { Some(rec.admin_code) },
            rec.tz.replace(" ", "_").replace("-", "").replace("/", "__"),
        ).unwrap();
    }
    writeln!(out, "];").unwrap();
}
