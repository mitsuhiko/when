use std::borrow::Cow;
use std::fmt;

use anyhow::{bail, Error};
use chrono_tz::Tz;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ZoneKind {
    City,
    Timezone,
    Airport,
    Division,
}

#[derive(Debug)]
pub struct Location {
    pub(crate) name: &'static str,
    pub(crate) country: &'static str,
    pub(crate) admin_code: Option<&'static str>,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) kind: ZoneKind,
    pub(crate) tz: Tz,
}

#[derive(Debug, Clone, Copy)]
pub enum ZoneRef {
    Tz(Tz),
    Location(&'static Location),
}

impl fmt::Display for ZoneRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind() == ZoneKind::Timezone {
            write!(f, "{}", self.name())
        } else {
            write!(f, "{}", self.name())?;
            if let Some(code) = self.admin_code() {
                write!(f, ", {}", code)?;
            }
            if let Some(country) = self.country() {
                write!(f, "; ")?;
                write!(f, "{}", country)?;
            }
            Ok(())
        }
    }
}

impl ZoneRef {
    pub fn name(&self) -> &str {
        match self {
            ZoneRef::Tz(tz) => tz.name(),
            ZoneRef::Location(loc) => loc.name,
        }
    }

    pub fn is_utc(&self) -> bool {
        matches!(
            self.tz().name(),
            "Universal"
                | "UTC"
                | "UCT"
                | "Zulu"
                | "Etc/Universal"
                | "Etc/UCT"
                | "Etc/UTC"
                | "Etc/Zulu"
        )
    }

    pub fn kind(&self) -> ZoneKind {
        match self {
            ZoneRef::Tz(_) => ZoneKind::Timezone,
            ZoneRef::Location(loc) => loc.kind,
        }
    }

    pub fn country(&self) -> Option<&str> {
        match self {
            ZoneRef::Tz(_) => None,
            ZoneRef::Location(loc) => COUNTRIES
                .binary_search_by_key(&loc.country, |x| x.0)
                .ok()
                .map(|pos| COUNTRIES[pos].1),
        }
    }

    pub fn admin_code(&self) -> Option<&str> {
        match self {
            ZoneRef::Tz(_) => None,
            ZoneRef::Location(loc) => loc.admin_code,
        }
    }

    pub fn tz(&self) -> Tz {
        match self {
            ZoneRef::Tz(tz) => *tz,
            ZoneRef::Location(loc) => loc.tz,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/locations.rs"));

/// Tries to locate a zone by name
pub fn find_zone(name: &str) -> Result<ZoneRef, Error> {
    let name = if name.eq_ignore_ascii_case("local") {
        match localzone::get_local_zone() {
            Some(zone) => Cow::Owned(zone),
            None => Cow::Borrowed("UTC"),
        }
    } else {
        Cow::Borrowed(name)
    };

    let tz_name = name.replace(" ", "_");
    for tz in chrono_tz::TZ_VARIANTS {
        if tz.name().eq_ignore_ascii_case(&tz_name) {
            return Ok(ZoneRef::Tz(tz));
        }
    }

    for delim in [',', ' '] {
        if let Some((name, code)) = name.rsplit_once(delim) {
            let name = name.trim_end();
            let code = code.trim_start();
            if let Some(rv) = LOCATIONS.iter().find(|x| {
                x.name.eq_ignore_ascii_case(name)
                    && (x.country.eq_ignore_ascii_case(code)
                        || x.admin_code.map_or(false, |x| x.eq_ignore_ascii_case(code)))
            }) {
                return Ok(ZoneRef::Location(rv));
            }
        }
    }

    if let Some(loc) = LOCATIONS
        .iter()
        .find(|x| x.name.eq_ignore_ascii_case(&name))
        .map(ZoneRef::Location)
    {
        return Ok(loc);
    }

    if name.len() == 3 {
        if let Some(loc) = LOCATIONS
            .iter()
            .find(|x| x.aliases.iter().any(|x| x.eq_ignore_ascii_case(&name)))
            .map(ZoneRef::Location)
        {
            return Ok(loc);
        }
    }

    bail!("unknown time zone '{}'", name)
}
