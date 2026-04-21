use crate::api_utils::api_error::ApiError;
use chrono::Offset;
use chrono::{FixedOffset, Utc};
use chrono_tz::Tz;
use flexi_logger::{DeferredNow, Record, style};
use regex::Regex;
use sea_orm::entity::prelude::*;
use std::sync::LazyLock;
use validator::ValidationError;

/// Application timezone loaded from `APP_TIMEZONE` env var (defaults to `America/Mexico_City`).
static APP_TZ: LazyLock<Tz> = LazyLock::new(|| {
    let tz_str =
        std::env::var("APP_TIMEZONE").unwrap_or_else(|_| "America/Mexico_City".to_string());
    tz_str
        .parse::<Tz>()
        .unwrap_or_else(|_| panic!("Invalid APP_TIMEZONE value: {tz_str}"))
});
// 1. Definir la expresión regular para caracteres especiales permitidos.
// Acepta alfanuméricos, guion bajo y arroba.
static RE_SPECIAL_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_@]+$").unwrap());

// 2. Función de validación personalizada para usar en el atributo.
pub fn validate_special_chars(value: &str) -> Result<(), ValidationError> {
    if !RE_SPECIAL_CHARS.is_match(value) {
        return Err(ValidationError::new(
            "Invalid characters: only letters, numbers, underscores, and @ are allowed",
        ));
    }
    Ok(())
}

// ---------------------- Helpers ----------------------
/// Name: `parse_date_str`
/// Description: Parse a `YYYY-MM-DD` string into a `sea_orm::Date`.
/// Parameters: `date_str` - string slice with date in `YYYY-MM-DD` format.
/// Outputs: `Ok(Date)` on success or `Err(ApiError::ValidationError)` on parse failure.
pub fn parse_date_str(date_str: &str) -> Result<Date, ApiError> {
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date: Date = d;
            Ok(date)
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

/// Name: `to_page_index`
/// Description: Convert client 1-based `page` to 0-based page index used by paginators.
/// Parameters: `page` - page number from client (u64).
/// Outputs: 0-based `usize` page index.
pub fn to_page_index(page: u64) -> u64 {
    if page == 0 { 0 } else { page.saturating_sub(1) }
}

/// Name: `to_page_limit`
/// Description: Convert client `limit` to page size for paginators, capped at 100.
/// Parameters: `page` - requested page size from client (u64).
/// Outputs: page size for paginators (u64), between 1 and 100.
pub fn to_page_limit(page: u64) -> u64 {
    if page == 0 { 10 } else { page }
}
/// Name: `get_current_timestamp_now`
/// Description: Get the current timestamp in UTC with a fixed offset of 0.
/// Outputs: `DateTimeWithTimeZone` representing the current timestamp.
pub fn get_current_timestamp_now() -> DateTimeWithTimeZone {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

///Name: get_current_timestamp_at_zone
///Description: Get the current timestamp in the configured APP_TIMEZONE.
///Outputs: `DateTimeWithTimeZone` representing the current timestamp in the configured timezone.
pub fn get_current_timestamp_at_zone(date_time: DateTimeWithTimeZone) -> DateTimeWithTimeZone {
    // Convert the incoming fixed-offset datetime to UTC, then to APP_TZ (handles DST),
    // and finally to a fixed-offset `DateTimeWithTimeZone` representing the same instant
    // in the local offset.
    let dt_utc = date_time.with_timezone(&Utc);
    let tz_dt = dt_utc.with_timezone(&*APP_TZ);
    let fixed_offset = tz_dt.offset().fix();

    tz_dt.with_timezone(&fixed_offset)
}

pub fn parse_date_str_to_date_time_with_timezone(
    date_str: &str,
) -> Result<DateTimeWithTimeZone, ApiError> {
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date_time = d.and_hms_opt(0, 0, 0).unwrap();
            Ok(DateTimeWithTimeZone::from_naive_utc_and_offset(
                date_time,
                FixedOffset::east_opt(0).unwrap(),
            ))
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

pub fn parse_date_time_str_to_date_time_with_timezone(
    date_time_str: &str,
) -> Result<DateTimeWithTimeZone, ApiError> {
    let naive = chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%dT%H:%M:%S");
    match naive {
        Ok(d) => Ok(DateTimeWithTimeZone::from_naive_utc_and_offset(
            d,
            FixedOffset::east_opt(0).unwrap(),
        )),
        Err(e) => Err(ApiError::ValidationError(format!(
            "Invalid date-time: {}",
            e
        ))),
    }
}

pub fn parse_date_time_str_to_date_time_with_timezone_opt(
    date_time_str: &str,
) -> Result<Option<DateTimeWithTimeZone>, ApiError> {
    if date_time_str.is_empty() {
        return Ok(None);
    }
    let naive = chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%dT%H:%M:%S");
    match naive {
        Ok(d) => Ok(Some(DateTimeWithTimeZone::from_naive_utc_and_offset(
            d,
            FixedOffset::east_opt(0).unwrap(),
        ))),
        Err(e) => Err(ApiError::ValidationError(format!(
            "Invalid date-time: {}",
            e
        ))),
    }
}

pub fn parse_date_str_to_date_time_with_timezone_local(
    date_str: &str,
) -> Result<DateTimeWithTimeZone, ApiError> {
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date_time = d.and_hms_opt(0, 0, 0).unwrap();
            Ok(get_current_timestamp_at_zone(
                DateTimeWithTimeZone::from_naive_utc_and_offset(
                    date_time,
                    FixedOffset::east_opt(0).unwrap(),
                ),
            ))
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

/// Parse a `YYYY-MM-DD` date range interpreting them as local dates in the configured
/// APP_TIMEZONE, converting to UTC for database queries.
/// - `date_init` → local 00:00:00 → UTC
/// - `date_end`  → local 23:59:59 → UTC
/// Handles DST automatically via `chrono_tz`.
pub fn parse_local_date_range_to_utc(
    date_init: &str,
    date_end: &str,
) -> Result<(Option<DateTimeWithTimeZone>, Option<DateTimeWithTimeZone>), ApiError> {
    let start = parse_local_date_to_utc(date_init, 0, 0, 0)?;
    let end = parse_local_date_to_utc(date_end, 23, 59, 59)?;

    if let (Some(s), Some(e)) = (start, end) {
        if s > e {
            return Err(ApiError::ValidationError(
                "Start date cannot be after end date".to_string(),
            ));
        }
    }
    Ok((start, end))
}

/// Parse a single `YYYY-MM-DD` string as local time in the configured APP_TIMEZONE
/// with the given hour/min/sec, then convert to UTC. Returns `None` for empty strings.
fn parse_local_date_to_utc(
    date_str: &str,
    hour: u32,
    min: u32,
    sec: u32,
) -> Result<Option<DateTimeWithTimeZone>, ApiError> {
    if date_str.is_empty() {
        return Ok(None);
    }
    let naive_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| ApiError::ValidationError(format!("Invalid date: {}", e)))?;
    let naive_dt = naive_date
        .and_hms_opt(hour, min, sec)
        .ok_or_else(|| ApiError::ValidationError("Invalid time".to_string()))?;
    // Interpret as local time in APP_TIMEZONE
    let local_dt = naive_dt
        .and_local_timezone(*APP_TZ)
        .single()
        .ok_or_else(|| ApiError::ValidationError("Ambiguous or invalid local time".to_string()))?;
    // Convert to UTC with fixed offset +00:00
    let utc_dt = local_dt.with_timezone(&Utc);
    Ok(Some(
        utc_dt.with_timezone(&FixedOffset::east_opt(0).unwrap()),
    ))
}

pub fn parse_date_str_to_date_time_with_timezone_opt(
    date_str: &str,
) -> Result<Option<DateTimeWithTimeZone>, ApiError> {
    if date_str.is_empty() {
        return Ok(None);
    }
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date_time = d.and_hms_opt(0, 0, 0).unwrap();
            Ok(Some(DateTimeWithTimeZone::from_naive_utc_and_offset(
                date_time,
                FixedOffset::east_opt(0).unwrap(),
            )))
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

pub fn validate_date_range(start_date: &str, end_date: &str) -> Result<(), ApiError> {
    let start = parse_date_str(start_date)?;
    let end = parse_date_str(end_date)?;

    if start > end {
        return Err(ApiError::ValidationError(
            "Start date cannot be after end date".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_date_time_range(
    start_date_time: &str,
    end_date_time: &str,
) -> Result<(), ApiError> {
    let start = parse_date_time_str_to_date_time_with_timezone(start_date_time)?;
    let end = parse_date_time_str_to_date_time_with_timezone(end_date_time)?;

    if start > end {
        return Err(ApiError::ValidationError(
            "Start date-time cannot be after end date-time".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_date_time_range_date(
    start_date_time: &str,
    end_date_time: &str,
) -> Result<(Option<DateTimeWithTimeZone>, Option<DateTimeWithTimeZone>), ApiError> {
    parse_local_date_range_to_utc(start_date_time, end_date_time)
}

pub fn validate_date_time_range_opt(
    start_date_time: &str,
    end_date_time: &str,
) -> Result<(), ApiError> {
    let start = parse_date_time_str_to_date_time_with_timezone_opt(start_date_time)?;
    let end = parse_date_time_str_to_date_time_with_timezone_opt(end_date_time)?;

    if let (Some(start), Some(end)) = (start, end)
        && start > end
    {
        return Err(ApiError::ValidationError(
            "Start date-time cannot be after end date-time".to_string(),
        ));
    }
    Ok(())
}

/// Custom log format: [YYYY-MM-DD HH:MM:SS.mmm TZ] LEVEL [module] message
pub fn custom_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}] {} [{}] {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f %:z"),
        record.level(),
        record.module_path().unwrap_or("<unknown>"),
        record.args()
    )
}

/// Colored format for stdout
pub fn custom_format_colored(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    let level = record.level();
    write!(
        w,
        "[{}] {} [{}] {}",
        style(level).paint(now.format("%Y-%m-%d %H:%M:%S%.3f %:z").to_string()),
        style(level).paint(level.to_string()),
        record.module_path().unwrap_or("<unknown>"),
        record.args()
    )
}
