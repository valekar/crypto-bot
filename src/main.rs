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
        RefCell::new(Vec::new()), // opens
        RefCell::new(Vec::new()), // closes
        RefCell::new(Vec::new()), // highs
        RefCell::new(Vec::new()), // lows
        BNB_BUSD,
        ONE_MINUTE_KLINE,
        BNB,
        0.2,
        BUSD,
        1.0,
        StrategyType::RSI(true),
    );

    //let result = my_binance.buy_asset_with_stable();
    // warn!("IS the asset bought?? {}", result);

    // let result = my_binance.sell_asset();
    // warn!("IS the asset sold?? {}", result);

    let in_position_for_rsi: &mut bool = &mut false;

    let mut web_socket = my_binance.kline_websocket(in_position_for_rsi);
    //my_binance.call_trading()(StrategyType::RSI, &mut false);

    my_binance.open_websocket_with_pair(&mut web_socket);
    my_binance.close_websocket(&mut web_socket);
}
