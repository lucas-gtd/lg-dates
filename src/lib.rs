#![deny(clippy::all)]

use napi_derive::napi;
use chrono::{DateTime, Utc};

#[napi]
pub fn date(year: i32, month: i32, day: i32) -> DateTime<Utc> {
    chrono::NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
}

#[napi]
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

#[napi]
pub fn add(date: DateTime<Utc>, ammount: i64, unit: String) -> DateTime<Utc> {
    match unit.as_str() {
        "years" => date + chrono::Duration::days(ammount * 365),
        "days" => date + chrono::Duration::days(ammount),
        "hours" => date + chrono::Duration::hours(ammount),
        "minutes" => date + chrono::Duration::minutes(ammount),
        "seconds" => date + chrono::Duration::seconds(ammount),
        _ => date,
    }
}

#[napi]
pub fn subtract(date: DateTime<Utc>, ammount: i64, unit: String) -> DateTime<Utc> {
    match unit.as_str() {
        "years" => date - chrono::Duration::days(ammount * 365),
        "days" => date - chrono::Duration::days(ammount),
        "hours" => date - chrono::Duration::hours(ammount),
        "minutes" => date - chrono::Duration::minutes(ammount),
        "seconds" => date - chrono::Duration::seconds(ammount),
        _ => date,
    }
}

#[napi]
pub fn format(date: DateTime<Utc>, format: String) -> String {
    date.format(&format).to_string()
}