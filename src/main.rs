use chrono::Datelike;
use chrono::Timelike;
use std::error::Error;
// use reqwest::blocking as reqw;
use clap::Arg;
use serde::Deserialize;

// const API_URL: &str = "http://worldtimeapi.org/api/timezone/{area}/{location}";
const API_URL: &str = "http://worldtimeapi.org/api/timezone";
const DEFAULT_LOCATION: &str = "Europe";
const DEFAULT_AREA: &str = "Rome";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("dateupdate_rs")
        .version("0.1")
        .author("Lorenzo Botti")
        .arg(Arg::with_name("help").short("h").long("help"))
        .arg(Arg::with_name("command").short("c").long("command"))
        .arg(
            Arg::with_name("location")
                .short("l")
                .long("location")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("area")
                .short("a")
                .long("area")
                .takes_value(true),
        )
        .get_matches();

    let sudo = matches.occurrences_of("sudo") >= 1;
    let command = matches.occurrences_of("command") >= 1 || sudo;
    let help = matches.occurrences_of("help") >= 1;

    let location = matches.value_of("location").unwrap_or(DEFAULT_LOCATION);
    let area = matches.value_of("area").unwrap_or(DEFAULT_AREA);

    if help {
        eprintln!("Look man, I'll help at some point");
        std::process::exit(0);
    }

    let res = ApiResult::from_api(location, area)?;

    let formatted = res.format()?;
    let formatted_date = formatted.as_str();

    if sudo {
        println!("sudo date --set \"{}\"", formatted_date);
    } else if command {
        println!("date --set \"{}\"", formatted_date);
    } else {
        println!("{}", formatted_date);
    }

    Ok(())
}

#[derive(Deserialize, Debug, Clone)]
struct ApiResult {
    abbreviation: String,
    datetime: String,
    utc_datetime: String,
    dst: bool,

    day_of_year: u16,
    day_of_week: u16,
    week_number: u16,

    timezone: String,
    unixtime: u64,
}

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::ParseError;

impl ApiResult {
    fn time(&self) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_rfc3339(&self.datetime)
    }

    fn format(&self) -> Result<String, Box<dyn Error>> {
        let time = self.time()?;
        Ok(format!(
            "{:04}{:02}{:02} {:02}:{:02}",
            time.year(),
            time.month(),
            time.day(),
            time.time().hour(),
            time.time().minute(),
        ))
    }

    fn from_api(location: &str, area: &str) -> Result<Self, Box<dyn Error>> {
        let res = reqwest::blocking::get(format!(
            "{url}/{area}/{location}",
            url = API_URL,
            area = area,
            location = location
        ))?;
        serde_json::from_reader(res).map_err(Into::into)
    }
}

// abbreviation     CEST
// day_of_year      207
// dst_from         2021-03-28T01:00:00+00:00
// raw_offset       3600
// dst_offset       3600
// utc_datetime     2021-07-26T12:59:59.739761+00:00
// week_number      30
// client_ip        37.162.183.219
// dst              true
// dst_until        2021-10-31T01:00:00+00:00
// utc_offset       +02:00
// datetime         2021-07-26T14:59:59.739761+02:00
// day_of_week      1
// timezone         Europe/Rome
// unixtime         1.627304399e+09
