extern crate RollingOHLC;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

struct TickerUpdate {
    symbol: String,
    timestamp: Instant,
    bid_price: f64,
    ask_price: f64,
    bid_qty: f64,
    ask_qty: f64,
}

fn main() {
    let window = Duration::from_secs(5 * 60);
    let mut rolling_ohlc1 = RollingOHLC::RollingOHLC::new(window);
    let mut rolling_ohlc2 = RollingOHLC::RollingOHLC::new(window);

    // Read ticker updates from file for symbol1
    let file1 = File::open("dataset-a.txt").unwrap();
    let reader1 = BufReader::new(file1);
    for line in reader1.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(',').collect();
        let symbol = fields[2].to_string();
        let timestamp = fields[7].parse::<u64>().unwrap();
        let timestamp = timestamp.parse::<Instant>();
        let bid_price = fields[3].parse::<f64>().unwrap();
        let ask_price = fields[5].parse::<f64>().unwrap();
        let bid_qty = fields[4].parse::<f64>().unwrap();
        let ask_qty = fields[6].parse::<f64>().unwrap();
        let ticker_update = TickerUpdate {
            symbol,
            timestamp,
            bid_price,
            ask_price,
            bid_qty,
            ask_qty,
        };
        if ticker_update.symbol == "symbol1" {
            rolling_ohlc1.push(RollingOHLC::Price {
                timestamp: ticker_update.timestamp,
                value: (ticker_update.bid_price + ticker_update.ask_price) / 2.0,
            });
            if let Some(ohlc) = rolling_ohlc1.get() {
                println!(
                    "Symbol 1 OHLC: {:?} - {:?}",
                    ohlc,
                    ticker_update.timestamp
                );
            }
        } else {
            rolling_ohlc2.push(RollingOHLC::Price {
                timestamp: ticker_update.timestamp,
                value: (ticker_update.bid_price + ticker_update.ask_price) / 2.0,
            });
            if let Some(ohlc) = rolling_ohlc2.get() {
                println!(
                    "Symbol 2 OHLC: {:?} - {:?}",
                    ohlc,
                    ticker_update.timestamp
                );
            }
        }
    }
}

