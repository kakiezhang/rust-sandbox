use chrono;

static DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

fn main() {
    let date = chrono::Utc::now().format(DATE_FORMAT).to_string();
    println!("{:?}", date);

    let date = chrono::Local::now().format(DATE_FORMAT).to_string();
    println!("{:?}", date);
}
