
// A function that takes a 12h time string and returns a 24h time string
fn convert_12h_to_24h(time: &str) -> String {
    // Split the time string by ":" and get the hour, minute and second parts
    let parts: Vec<&str> = time.split(":").collect();
    let hour = parts[0];
    let minute = parts[1];
    let second = parts[2];

    // Check if the time is AM or PM and adjust the hour accordingly
    let is_pm = second.ends_with("PM");
    let mut hour_24 = hour.parse::<u32>().unwrap(); // Parse the hour as a u32
    if is_pm && hour_24 != 12 {
        // If PM and not 12, add 12 to the hour
        hour_24 += 12;
    } else if !is_pm && hour_24 == 12 {
        // If AM and 12, set the hour to 0
        hour_24 = 0;
    }

    // Format the 24h time string with leading zeros if needed
    format!("{:02}:{:02}:{:02}", hour_24, minute, &second[..2])
}

// A test case
fn main() {
    let time_12h = "12:10:22AM";
    let time_24h = convert_12h_to_24h(time_12h);
    println!("{} -> {}", time_12h, time_24h); // Prints "12:10:22AM -> 00:10:22"
}
