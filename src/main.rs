//#![allow(dead_code)]

use exchange::my_binance::{Exchange, MyBinance};
use std::cell::RefCell;
use std::env;
use strategy::rsi::RsiTradingStrategy;
use utils::constants::{ONE_MINUTE_KLINE, SOL_USDT};
use utils::utility::load_env;

mod exchange;
mod strategy;
mod utils;

fn main() {
    load_env();

    let account = MyBinance::get_account();
    let my_binance: MyBinance = Exchange::new(
        account.unwrap(),
        RefCell::new(Vec::new()),
        SOL_USDT,
        ONE_MINUTE_KLINE,
    );

    let in_position: &mut bool = &mut false;
    let mut web_socket = my_binance.kline_websocket(&my_binance, in_position);

    my_binance.open_websocket(&mut web_socket);
    my_binance.close_websocket(&mut web_socket);
}
