use chrono::{DateTime, Local};

pub fn format_relative_time(expires_at: &DateTime<Local>) -> String {
    let now = chrono::Utc::now();
    let minutes = expires_at.signed_duration_since(now).num_minutes();

    match minutes {
        m if m <= 0 => "less than a minute".to_string(),
        1 => "1 minute".to_string(),
        m if m < 60 => format!("{} minutes", m),
        m if m < 120 => "1 hour".to_string(),
        m if m < 1440 => format!("{} hours", m / 60),
        m if m < 2880 => "1 day".to_string(),
        m => format!("{} days", m / 1440),
    }
}