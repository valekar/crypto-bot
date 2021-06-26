//#![allow(dead_code)]
use binance_market::Binance;
use std::cell::RefCell;
use strategy::rsi::RsiTradingStrategy;

mod binance_market;
mod strategy;
mod utils;

fn main() {
    let binance_market = Binance::new(RefCell::new(Vec::new()), "solusdt", "@kline_1m");

    let in_position: &mut bool = &mut false;
    let mut web_socket = Binance::kline_websocket(&binance_market, in_position);
    binance_market.open_websocket(&mut web_socket);
    binance_market.close_websocket(&mut web_socket);
}
