use crate::api_utils::api_error::ApiError;
use chrono::{FixedOffset, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use sea_orm::entity::prelude::*;
use validator::ValidationError;
// 1. Definir la expresión regular para caracteres especiales permitidos.
// Acepta alfanuméricos, guion bajo y arroba.
lazy_static! {
    static ref RE_SPECIAL_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9_@]+$").unwrap();
}

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
    if page == 0 {
        0
    } else {
        (page.saturating_sub(1)) as u64
    }
}

/// Name: `to_page_limit`
/// Description: Convert client `page` to offset for paginators (assuming fixed page size of 10).
/// Parameters: `page` - page number from client (u64).
/// Outputs: offset for paginators (u64).
pub fn to_page_limit(page: u64) -> u64 {
    if page == 0 { 10 } else { page }
}

pub fn get_current_timestamp_now() -> DateTimeWithTimeZone {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}
