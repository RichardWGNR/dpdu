use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use semver::Version;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct PduVersionData {
    pub mvci_part1_standard_version: PduVersion,

    pub mvci_part2_standard_version: PduVersion,

    pub hw_serial_number: u32,

    pub hw_name: Option<String>,

    pub hw_version: PduVersion,

    pub hw_date: PduDate,

    pub hw_interface: u32,

    pub fw_name: Option<String>,

    pub fw_version: PduVersion,

    pub fw_date: PduDate,

    pub vendor_name: Option<String>,

    pub pdu_api_sw_name: Option<String>,

    pub pdu_api_sw_version: PduVersion,

    pub pdu_api_sw_date: PduDate
}

#[derive(Debug, Clone, Default)]
pub struct PduVersion {
    /// Major.
    pub major: u8,

    /// Minor.
    pub minor: u8,

    /// Patch.
    pub revision: u8
}

impl PduVersion {
    pub fn to_semver(&self) -> Version {
        Version::new(self.major as _, self.minor as _, self.revision as _)
    }
}

impl Display for PduVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.revision)
    }
}

impl From<u32> for PduVersion {
    fn from(value: u32) -> Self {
        // 11.1.4.16 Coding of dates
        let major = (value >> 24) as u8;
        let minor = (value >> 16) as u8;
        let patch = (value >> 8) as u8;

        Self {
            major,
            minor,
            revision: patch
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PduDate {
    /// Year since 1970 (0..255)
    pub year: u8,

    /// Month (1..12)
    pub month: u8,

    /// Day (1..31)
    pub day: u8,

    /// Week (1..52, 0 if not used)
    pub week: u8,
}

impl PduDate {
    pub fn normalize_year(&self) -> u32 {
        (self.year as u32).wrapping_add(1970)
    }

    pub fn to_naive_date(&self) -> Option<NaiveDate> {
        match NaiveDate::from_ymd_opt(self.normalize_year() as _, self.month as _, self.day as _) {
            Some(v) => Some(v),
            None => {
                warn!(y = self.normalize_year(), m = self.month, d = self.day, "Invalid PduDate");
                None
            }
        }
    }
}

impl Display for PduDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self.to_naive_date() {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => format!("{}-{}-{}", self.normalize_year(), self.month, self.day)
        };
        write!(f, "{str}")
    }
}

impl From<u32> for PduDate {
    fn from(value: u32) -> Self {
        // 11.1.4.15 Coding of version numbers
        let year = (value >> 24) as u8;
        let month = (value >> 16) as u8;
        let day = (value >> 8) as u8;
        let week = (value & 0xFF) as u8;

        PduDate {
            year,
            month,
            day,
            week
        }
    }
}