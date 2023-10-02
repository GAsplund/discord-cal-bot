use sqlx;

pub fn add_auth(calendar: String, secret: String, expiry: i64) {

}

pub fn get_notify_channels_for_event(event: String) -> Vec<u64> {
    vec![]
}

pub fn cal_from_resource(resource_id: String) -> String {
    "primary".to_string()
}
