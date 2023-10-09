use actix_web::{get, web, Responder, Result};
use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub enum NoAvailabilityReason {
    BankHoliday,
    ClinicianBusy,
}

#[derive(Serialize)]
struct Availability {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    is_available: bool,
    no_availability_reason: Option<NoAvailabilityReason>,
}

#[derive(Serialize)]
struct AvailabilityByDate {
    date: DateTime<Utc>,
    slots: Vec<Availability>,
}

struct WorkSchedule {
    monday: Vec<Availability>,
    tuesday: Vec<Availability>,
    wednesday: Vec<Availability>,
    thursday: Vec<Availability>,
    friday: Vec<Availability>,
    saturday: Vec<Availability>,
    sunday: Vec<Availability>,
}

#[get("/availabilities")]
pub async fn get_availabilities() -> Result<impl Responder> {
    let availability = Availability {
        start: Utc.with_ymd_and_hms(2023, 10, 09, 08, 00, 00).unwrap(),
        end: Utc.with_ymd_and_hms(2023, 10, 09, 08, 00, 00).unwrap(),
        is_available: true,
        no_availability_reason: None,
    };
    let obj = AvailabilityByDate {
        date: Utc.with_ymd_and_hms(2023, 10, 08, 23, 50, 10).unwrap(),
        slots: vec![availability],
    };
    Ok(web::Json(obj))
}
