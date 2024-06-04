fn main() {
    let year = 2024;

    let month_days = [
        31, if is_leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
    ];

    for i in 1..=12 {
        let day_of_week: i8 = day_of_week(year, i, 1) as i8 - 1;

        print_month(if day_of_week == -1 { 6 } else { day_of_week as u8 }, month_days[(i - 1) as usize]);

        println!("\n")
    }
}

fn is_leap_year(year: u16) -> bool {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

fn day_of_week(year: u16, month: u16, day: u16) -> u8 {
    let q = day;
    let m = if month < 3 { month + 12 } else { month };
    let year = if month < 3 { year - 1 } else { year };

    let k = year % 100;
    let j = year / 100;

    let h = (q + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;

    h as u8
}

fn print_month(start_day: u8, days: u8) {
    if start_day > 6 {
        panic!("Start day of the must must be 0-6 (Sun - Sat).");
    }

    println!("Sun\tMon\tTue\tWed\tThu\tFri\tSat");

    let mut current_day: u8 = start_day;

    print!("{}{}", pad("", ' ', 3), "\t".repeat(current_day as usize));

    for i in 1..=days {
        print!("{}\t", pad(i.to_string().as_str(), ' ', 3));

        if current_day == 6 {
            current_day = 0;
            println!();
            continue;
        }

        current_day += 1
    }
}

fn pad(value: &str, pad_char: char, max_len: u8) -> String {
    let value_len = value.len() as u8;

    if value_len > max_len {
        panic!("String value length must be less than or equal to the assigned maximum length.");
    }

    if value_len == max_len {
        return String::from(value);
    }

    return String::from(format!("{}{}", String::from(pad_char.to_string().repeat((max_len - value_len) as usize)), value));
}