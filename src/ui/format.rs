pub const NON_TIME_STRING: &str = "--:--";

pub struct UFormat {}

impl UFormat {
    pub fn time_str(value: u64) -> String {
        let seconds = value % 60;

        format!("{}:{:02}", (value - seconds) / 60, seconds)
    }
}