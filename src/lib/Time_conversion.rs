fn main() {
    let time_12h = "12:10:22AM";
    let time_24h = to_24h(time_12h);
    println!("{} -> {}", time_12h, time_24h);
}

fn to_24h(time: &str) -> String {
    let v: Vec<&str> = time.split(":").collect();
    let (h, m, s) = (v[0], v[1], v[2]);
    let is_pm = s.ends_with("PM");
    let mut hour_24 = h.parse::<u32>().unwrap();

    if is_pm && hour_24 != 12 {
        hour_24 += 12;
    } else if !is_pm && hour_24 == 12 {
        hour_24 = 0;
    }
    format!("{:02}:{:02}:{:02}", hour_24, m, &s[..2])
}
