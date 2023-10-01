use std::env;

use chrono::{Timelike, Utc};

pub fn enough_time_between_changes(minutes_between_changes: &i32) -> bool {
    // If minutes_between_changes is 0 change on every workspace change
    // We don't care about setting minute_last_changed here either
    if *minutes_between_changes == 0 as i32 {
        println!("minutes = 0, returning true");
        return true;
    }

    // If minutes_between_changes != 0 determine if a change should be issued
    let current_min = Utc::now().minute() as i32;
    let minute_last_changed = get_minute_last_changed();

    let calculated_minute_last_changed = if minute_last_changed > current_min {
        minute_last_changed - 59
    } else {
        minute_last_changed
    };

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
