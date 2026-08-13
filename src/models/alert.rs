use chrono::Date;
use chrono::DateTime;
use chrono::Utc;
use chrono_tz::{Tz, America::Bogota};
pub struct Alert{
    pub id: u32,
    pub alert_type: String,
    pub request_user: String,
    pub request_time: DateTime<Tz>,
    pub request_public_ip: String,
    pub public_host_ip: String,
    pub private_host_ip: String
}