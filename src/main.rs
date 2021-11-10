use clap::{App, Arg};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
struct Stock {
    #[serde(rename = "c")]
    current: f32,

    #[serde(rename = "d")]
    change: f32,

    #[serde(rename = "dp")]
    change_percent: f32,

    #[serde(rename = "h")]
    high: f32,

    #[serde(rename = "l")]
    low: f32,

    #[serde(rename = "o")]
    open: f32,

    #[serde(rename = "pc")]
    prev_close: f32,

    #[serde(rename = "t")]
    time: i64,
}

impl fmt::Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            Current Price: {}
            Change: {}
            Percent Change: {}%",
            self.current, self.change, self.change_percent
        )
    }
}

fn main() {
    let matches = App::new("Stock Ticker")
        .version("0.1.0")
        .author("Marcus Butler <mtbutler07@gmail.com>")
        .about("Uses Finnhub to grab stock information")
        .arg(
            Arg::with_name("symbol")
                .short("s")
                .long("symbol")
                .index(1)
                .required(true)
                .takes_value(true)
                .help("Stock Symbol To Lookup"),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .env("FINNHUB_API_TOKEN")
                .required(true)
                .help("finnhub.io API Token"),
        )
        .get_matches();

    let symbol: &str = matches.value_of("symbol").unwrap();
    let token: &str = matches.value_of("token").unwrap();
    let url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, token
    );

    let response = reqwest::blocking::get(url).unwrap();

    let data = response.json::<Stock>().unwrap();

    println!("{}", data);
}
