use std::borrow::Cow;
use std::fmt;

use chrono_tz::Tz;

/// The type of location.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LocationKind {
    City,
    Timezone,
    Airport,
    Division,
}

/// Represents a timezone location.
#[derive(Debug)]
pub struct Location {
    pub(crate) name: &'static str,
    pub(crate) country: &'static str,
    pub(crate) admin_code: Option<&'static str>,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) kind: LocationKind,
    pub(crate) tz: Tz,
}

/// Reference to a timezone.
#[derive(Debug, Clone, Copy)]
pub enum ZoneRef {
    Tz(Tz),
    Location(&'static Location),
}

impl fmt::Display for ZoneRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind() == LocationKind::Timezone {
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
    /// Returns the name of the zone reference.
    ///
    /// For actual timezones that can be the IANA name, for cities
    /// and airports this will be the actual name of the location.
    pub fn name(&self) -> &str {
        match self {
            ZoneRef::Tz(tz) => tz.name(),
            ZoneRef::Location(loc) => loc.name,
        }
    }

    /// True if this zone is the UTC zone.
    ///
    /// Note that this is different than checking if the zone is currently
    /// at UTC+0.
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

    /// Returns the kind of location.
    pub fn kind(&self) -> LocationKind {
        match self {
            ZoneRef::Tz(_) => LocationKind::Timezone,
            ZoneRef::Location(loc) => loc.kind,
        }
    }

    /// If this zone reference points to a country, returns the country name.
    pub fn country(&self) -> Option<&str> {
        match self {
            ZoneRef::Tz(_) => None,
            ZoneRef::Location(loc) => COUNTRIES
                .binary_search_by_key(&loc.country, |x| x.0)
                .ok()
                .map(|pos| COUNTRIES[pos].1),
        }
    }

    /// If the zone has an admin code returns it.
    ///
    /// For the US for instance this can be the name of the US state.
    pub fn admin_code(&self) -> Option<&str> {
        match self {
            ZoneRef::Tz(_) => None,
            ZoneRef::Location(loc) => loc.admin_code,
        }
    }

    /// Returns a `chrono_tz` timezone object.
    pub fn tz(&self) -> Tz {
        match self {
            ZoneRef::Tz(tz) => *tz,
            ZoneRef::Location(loc) => loc.tz,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/locations.rs"));

/// Tries to locate a zone by name
pub fn find_zone(name: &str) -> Option<ZoneRef> {
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
            return Some(ZoneRef::Tz(tz));
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
                return Some(ZoneRef::Location(rv));
            }
        }
    }

    if let Some(loc) = LOCATIONS
        .iter()
        .find(|x| x.name.eq_ignore_ascii_case(&name))
        .map(ZoneRef::Location)
    {
        return Some(loc);
    }

    if name.len() == 3 {
        if let Some(loc) = LOCATIONS
            .iter()
            .find(|x| x.aliases.iter().any(|x| x.eq_ignore_ascii_case(&name)))
            .map(ZoneRef::Location)
        {
            return Some(loc);
        }
    }

    None
}
