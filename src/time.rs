use std::env;

use chrono::{Timelike, Utc};

pub fn should_change(minutes_between_changes: &i32) -> bool {
    let current_min = Utc::now().minute() as i32;
    let minute_last_changed = get_minute_last_changed();

    let calculated_minute_last_changed = if minute_last_changed > current_min {
        minute_last_changed - 59
    } else {
        minute_last_changed
    };

    // Handles changes at most every 5 minutes
    if calculated_minute_last_changed + minutes_between_changes > current_min {
        return false;
    }

    set_minute_last_changed(Utc::now().minute() as i32);
    return true;
}

pub fn get_minute_last_changed() -> i32 {
    let last_changed_value = match env::var("SWWW_MIN_LAST_CHANGED") {
        Ok(min) => min,
        Err(_) => String::from("0"),
    };

    last_changed_value.parse::<i32>().unwrap_or(0)
}

pub fn set_minute_last_changed(min: i32) {
    env::set_var("SWWW_MIN_LAST_CHANGED", min.to_string());
}
