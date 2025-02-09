use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Result, Error};
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct CryptoPrice {
    usd: f64,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Crypto {
    BAN,
    DYM,
    SCRT,
    CHILLGUY,
    SUPRA,
    Unknown
}

struct Info {
    buy_in_price: f64,
    value: u32,
}

impl Info {
    fn new(price: f64, value: u32) -> Self {
        Info {
            buy_in_price: price,
            value,
        }
    }
}

fn get_crypto_name(name: &str) -> Crypto {
    match name {
        "BAN" => Crypto::BAN,
        "DYM" => Crypto::DYM,
        "SCRT" => Crypto::SCRT,
        "CHILLGUY" => Crypto::CHILLGUY,
        "SUPRA" => Crypto::SUPRA,
        _ => Crypto::Unknown
    }
}

fn name_to_string(name: &Crypto) -> &str {
    match name {
        Crypto::BAN => "BAN",
        Crypto::DYM => "DYM",
        Crypto::SCRT => "SCRT",
        Crypto::SUPRA => "SUPRA",
        Crypto::CHILLGUY => "CHILLGUY",
        _ => ""
    }
}

fn read_input() -> Result<HashMap<Crypto, Info>> {
    let input = File::open("portfolio.txt")?;
    let reader = BufReader::new(input);
    let mut portfolio: HashMap<Crypto, Info> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let buffer: Vec<&str> = content
                    .trim()
                    .split_whitespace()
                    .collect();

                if buffer.len() == 3 {
                    let part: (String, f64, u32) = (
                        buffer[0].to_string(),
                        buffer[1].parse().unwrap(),
                        buffer[2].parse().unwrap(),
                    );

                    let info = Info::new(part.1, part.2);
                    let crypto_name = get_crypto_name(&part.0);

                    portfolio.insert(crypto_name, info);
                }
            },
            Err(error) => eprintln!("Error: {}", error),
        }
    }
    Ok(portfolio)
}

fn get_crypto_id(name: &Crypto) -> &str {
    match name {
        Crypto::BAN => "comedian",
        Crypto::SUPRA => "supra",
        Crypto::DYM => "dymension",
        Crypto::SCRT => "secret",
        Crypto::CHILLGUY => "chill-guy",
        Crypto::Unknown => "none",
    }
}

fn get_live_price(crypto_id: &str) -> std::io::Result<f64> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        crypto_id
    );

    let response = get(&url)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        .json::<HashMap<String, CryptoPrice>>()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    if let Some(price) = response.get(crypto_id) {
        Ok(price.usd)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Crypto ID not found"))
    }
}

fn table() {
    let portfolio = read_input();
    let mut total_value = 0;
    let mut total_profit_in_usd = 0.0;

    println!("|---------------------------------------------------------------------------------|");
    println!("|                                  PORTFOLIO                                      |");
    println!("|---------------------------------------------------------------------------------|");
    println!("|  SYMBOL  | Capital(USD) | Entry Price | Current Price | Profit(%) | Profit(USD) |");
    println!("|---------------------------------------------------------------------------------|");
    for (symbol, info) in &portfolio.unwrap() {
        let name = name_to_string(symbol);
        let price_result = get_live_price(get_crypto_id(&symbol));

        let current_price = match price_result {
            Ok(price) => price,
            Err(_) => info.buy_in_price,
        };

        let profit_in_percentage = (current_price - info.buy_in_price) / info.buy_in_price * 100.0;
        let profit_in_usd = profit_in_percentage * (info.value as f64) / 100.0;
        total_value += info.value;
        total_profit_in_usd += profit_in_usd;
        println!("| {:<8} | {:<12} | {:<11} | {:<13} | {:<9.2} | {:<11.2} |", name, info.value,  info.buy_in_price, current_price, profit_in_percentage, profit_in_usd);
    }
    let total_profit_in_percentage = total_profit_in_usd / (total_value as f64) * 100.0;
    println!("|---------------------------------------------------------------------------------|");
    println!("| Total invested(USD): {:<8} | Profit(%): {:>10.2} | Profit(USD): {:>10.2} |", total_value, total_profit_in_percentage, total_profit_in_usd);
    println!("|---------------------------------------------------------------------------------|");
}

fn main() {
    table();
}
