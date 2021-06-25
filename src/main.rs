//#![allow(dead_code)]
use binance_market::Binance;
use std::cell::RefCell;
use std::error::Error;

mod binance_market;
mod strategy;
mod utils;

fn main() {
    //let closes = RefCell::new(Vec::new());
    let binance_market = Binance::new("solusdt", "@kline_1m");
    let mut web_socket = Binance::create_kline_and_handle_websocket_event(&binance_market);
    binance_market.open_websocket(&mut web_socket);
    binance_market.close_websocket(&mut web_socket);
    //println!("Hello, world!");
}

enum Pair {
    solusdt(String),
}
