pub fn format_time(timestamp: u64) -> String {
    let hours = timestamp / 3600;
    let minutes = (timestamp % 3600) / 60;
    let seconds = timestamp % 60;

    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}
