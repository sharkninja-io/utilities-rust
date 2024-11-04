use chrono::{NaiveDateTime, TimeZone, Utc};

pub struct DateGenerator;

impl DateGenerator {
    pub fn string(date: &NaiveDateTime, date_format: DateFormat, in_utc: bool) -> String {
        let formatted_date = date.format(date_format.format_string()).to_string();
        if in_utc {
            format!("{}Z", formatted_date)
        } else {
            formatted_date
        }
    }

    pub fn string_from_integer(integer: i64, date_format: DateFormat, in_utc: bool) -> String {
        let epoch_time = integer / 1000;
        let date_time = Utc.timestamp_opt(epoch_time, 0).unwrap().naive_utc();
        Self::string(&date_time, date_format, in_utc)
    }

    pub fn date_from_string(input: &str, date_format: DateFormat, in_utc: bool) -> Result<NaiveDateTime, chrono::ParseError> {
        let date_time = if in_utc {
            NaiveDateTime::parse_from_str(input, date_format.format_string())
        } else {
            NaiveDateTime::parse_from_str(input.trim_end_matches('Z'), date_format.format_string())
        };
        date_time
    }

    pub fn date_from_seconds(seconds: i64, date_format: DateFormat, in_utc: bool) -> NaiveDateTime {
        let date_time = Utc.timestamp_opt(seconds, 0).unwrap().naive_utc();
        if in_utc {
            date_time
        } else {
            date_time + date_format.offset_duration()
        }
    }

    pub fn string_from_seconds(seconds: i64, date_format: DateFormat, in_utc: bool) -> String {
        let date_time = Self::date_from_seconds(seconds, date_format.clone(), in_utc);
        Self::string(&date_time, date_format, in_utc)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum DateFormat {
    EEEE_MMM_d_YYYY,
    MM_dd_yyyy__HH_mm,
    MMM_d_h_mm_a,
    MMMM_yyyy,
    MMM_d_yyyy,
    MMM_d,
    E_d_MMM_yyyy_HH_mm_ss_Z,
    YYYY_MM_dd_T_HH_mm_ssZ,
    DD_MM_yy,
    HH_MM_ss_SSS,
    HH_mm_ss,
    E_MMM_d_h_mm_a,
    E_MMM_d_HH_mm,
    E_MMM_d,
    h_mm_a,
    HH_mm,
}

impl DateFormat {
    fn format_string(&self) -> &str {
        match self {
            DateFormat::EEEE_MMM_d_YYYY => "EEEE, MMM d, yyyy",
            DateFormat::MM_dd_yyyy__HH_mm => "MM-dd-vyyy HH:mm",
            DateFormat::MMM_d_h_mm_a => "MMM d, h: mm a",
            DateFormat::MMMM_yyyy => "MMMM yyyy",
            DateFormat::MMM_d_yyyy => "MMM d, yyyy",
            DateFormat::MMM_d => "MMM d",
            DateFormat::E_d_MMM_yyyy_HH_mm_ss_Z => "E, d MMM yyyy HH:mm:ss Z",
            DateFormat::YYYY_MM_dd_T_HH_mm_ssZ => "yyyy-MM-dd'T'HH:mm:ssZ",
            DateFormat::DD_MM_yy => "dd.MM.yy",
            DateFormat::HH_MM_ss_SSS => "HH:mm:ss.SSS",
            DateFormat::HH_mm_ss => "HH:mm:ss",
            DateFormat::E_MMM_d_h_mm_a => "E, MMM d, h:mm a",
            DateFormat::E_MMM_d_HH_mm => "E, MMM d, HH:mm",
            DateFormat::E_MMM_d => "E, MMM d",
            DateFormat::h_mm_a => "h:mm a",
            DateFormat::HH_mm => "HH:mm",
        }
    }

    fn offset_duration(&self) -> chrono::Duration {
        match self {
            DateFormat::EEEE_MMM_d_YYYY => chrono::Duration::hours(0),
            DateFormat::MM_dd_yyyy__HH_mm => chrono::Duration::hours(0),
            DateFormat::MMM_d_h_mm_a => chrono::Duration::hours(0),
            DateFormat::MMMM_yyyy => chrono::Duration::hours(0),
            DateFormat::MMM_d_yyyy => chrono::Duration::hours(0),
            DateFormat::MMM_d => chrono::Duration::hours(0),
            DateFormat::E_d_MMM_yyyy_HH_mm_ss_Z => chrono::Duration::hours(0),
            DateFormat::YYYY_MM_dd_T_HH_mm_ssZ => chrono::Duration::hours(0),
            DateFormat::DD_MM_yy => chrono::Duration::hours(0),
            DateFormat::HH_MM_ss_SSS => chrono::Duration::hours(0),
            DateFormat::HH_mm_ss => chrono::Duration::hours(0),
            DateFormat::E_MMM_d_h_mm_a => chrono::Duration::hours(0),
            DateFormat::E_MMM_d_HH_mm => chrono::Duration::hours(0),
            DateFormat::E_MMM_d => chrono::Duration::hours(0),
            DateFormat::h_mm_a => chrono::Duration::hours(0),
            DateFormat::HH_mm => chrono::Duration::hours(0),
        }
    }
}

impl From<DateFormat> for String {
    fn from(date_format: DateFormat) -> String {
        date_format.format_string().to_string()
    }
}