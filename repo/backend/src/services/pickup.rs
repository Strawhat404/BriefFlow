use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Duration};
use shared::models::{StoreHours, Reservation};
use shared::dto::PickupSlot;
use rand::Rng;

/// Generate 15-minute pickup slots within store hours for a given date.
///
/// Marks slots as unavailable if they start within `prep_time_minutes` of now,
/// or if there are too many existing reservations for that slot (capacity = 5).
pub fn generate_pickup_slots(
    store_hours: &[StoreHours],
    date: NaiveDate,
    prep_time_minutes: i32,
    existing_reservations: &[Reservation],
) -> Vec<PickupSlot> {
    // Migration seeds day_of_week as 0=Sunday, 1=Monday, ..., 6=Saturday.
    // chrono's weekday().num_days_from_sunday() returns 0=Sun, 1=Mon, ..., 6=Sat.
    let day_of_week = date.weekday().num_days_from_sunday() as u8;

    let hours = store_hours
        .iter()
        .find(|h| h.day_of_week == day_of_week);

    let hours = match hours {
        Some(h) if !h.is_closed => h,
        _ => return Vec::new(),
    };

    let open = match NaiveTime::parse_from_str(&hours.open_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&hours.open_time, "%H:%M"))
    {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let close = match NaiveTime::parse_from_str(&hours.close_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&hours.close_time, "%H:%M"))
    {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let slot_duration = Duration::minutes(15);
    let now = Local::now().naive_local();
    let earliest_available = now + Duration::minutes(prep_time_minutes as i64);
    let max_reservations_per_slot: usize = 5;

    let mut slots = Vec::new();
    let mut slot_start_time = open;

    while slot_start_time + slot_duration <= close {
        let slot_start = NaiveDateTime::new(date, slot_start_time);
        let slot_end = slot_start + slot_duration;

        // Count overlapping reservations
        let reservation_count = existing_reservations
            .iter()
            .filter(|r| {
                r.status != "Expired" && r.status != "Canceled"
                    && r.pickup_slot_start < slot_end
                    && r.pickup_slot_end > slot_start
            })
            .count();

        let available = slot_start >= earliest_available
            && reservation_count < max_reservations_per_slot;

        slots.push(PickupSlot {
            start: slot_start.format("%Y-%m-%dT%H:%M:%S").to_string(),
            end: slot_end.format("%Y-%m-%dT%H:%M:%S").to_string(),
            available,
        });

        slot_start_time += slot_duration;
    }

    slots
}

/// Check whether a given slot start time is still available (far enough in the future).
pub fn is_slot_available(slot_start: NaiveDateTime, prep_time_minutes: i32) -> bool {
    let now = Local::now().naive_local();
    let earliest = now + Duration::minutes(prep_time_minutes as i64);
    slot_start >= earliest
}

/// Generate a voucher code in the format BF-XXXXXX (alphanumeric uppercase).
pub fn generate_voucher_code() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let code: String = (0..6).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("BF-{}", code)
}
