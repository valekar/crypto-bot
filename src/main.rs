//#![allow(dead_code)]

#[macro_use]
extern crate log;
//extern crate pretty_env_logger;
use exchange::my_binance::{Exchange, MyBinance};
use std::cell::RefCell;
use strategy::rsi::RsiTradingStrategy;
use strategy::rsi::StrategyType;
use utils::constants::{BNB, BNB_BUSD, BUSD, ONE_MINUTE_KLINE};
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
        BNB_BUSD,
        ONE_MINUTE_KLINE,
        BNB,
        0.2,
        BUSD,
        1.0,
    );

    // let result = my_binance.buy_asset_with();
    // warn!("IS the asset bought?? {}", result);

    // let result = my_binance.sell_asset();
    // warn!("IS the asset sold?? {}", result);

    let mut web_socket = my_binance.kline_websocket(&my_binance);
    my_binance.start_trading(StrategyType::RSI, &mut false);

    my_binance.open_websocket_with_pair(&mut web_socket);
    my_binance.close_websocket(&mut web_socket);
}
