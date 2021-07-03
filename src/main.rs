//#![allow(dead_code)]

#[macro_use]
extern crate log;
//extern crate pretty_env_logger;
use exchange::my_binance::{Exchange, MyBinance};
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use strategy::core_satellite_investment::CoreSatellite;
use strategy::rsi::RsiTradingStrategy;
use utils::constants::{BNB, BNB_BUSD, BUSD, ONE_MINUTE_KLINE};
use utils::utility::{load_env, StrategyType, TradingStyle};

mod exchange;
mod strategy;
mod utils;

fn main() {
    load_env();

    use_rsi_trading_strategy();

    /*
     * Core satellite strategy
     */
    //use_engulfing_trading_strategy();
}

fn use_rsi_trading_strategy() {
    let core_satellite_investment = initialize_core_satellite_investment();

    let account = MyBinance::get_account();

    let my_binance: MyBinance = Exchange::new(
        account.unwrap(),
        RefCell::new(Vec::new()),          // opens
        RefCell::new(Vec::new()),          // closes
        RefCell::new(Vec::new()),          // highs
        RefCell::new(Vec::new()),          // lows
        BNB_BUSD,                          // pair
        ONE_MINUTE_KLINE,                  // kline
        BNB,                               // left asset name
        1.0,                               // left asset percentage
        BUSD,                              // right asset name
        0.2,                               // right asset percentage
        StrategyType::Rsi(true),           // strategy type
        TradingStyle::CoreSatellite,       // trading style
        core_satellite_investment,         // Core satellite instance
        initialize_rsi_trading_strategy(), // Rsi Trading Strategy instance
    );

    //let result = my_binance.buy_asset_with_stable();
    // warn!("IS the asset bought?? {}", result);

    // let result = my_binance.sell_asset();
    // warn!("IS the asset sold?? {}", result);

    let mut web_socket = my_binance.kline_websocket();

    my_binance.open_websocket_with_pair(&mut web_socket);
    my_binance.close_websocket(&mut web_socket);
}

fn use_engulfing_trading_strategy() {
    let core_satellite_investment = initialize_core_satellite_investment();
    let account_1 = MyBinance::get_account();

    let my_binance_1: MyBinance = Exchange::new(
        account_1.unwrap(),
        RefCell::new(Vec::new()),          // opens
        RefCell::new(Vec::new()),          // closes
        RefCell::new(Vec::new()),          // highs
        RefCell::new(Vec::new()),          // lows
        BNB_BUSD,                          // pair
        ONE_MINUTE_KLINE,                  // kline
        BNB,                               // left asset name
        1.0,                               // left asset percentage
        BUSD,                              // right asset name
        0.2,                               // right asset percentage
        StrategyType::Engulfing(true),     // strategy type
        TradingStyle::CoreSatellite,       // Trading style
        core_satellite_investment,         // Core Satellite Instance
        initialize_rsi_trading_strategy(), // Rsi Trading Strategy Instance
    );

    let in_position_for_rsi: &mut bool = &mut false;

    let mut web_socket = my_binance_1.kline_websocket();

    my_binance_1.open_websocket_with_pair(&mut web_socket);
    my_binance_1.close_websocket(&mut web_socket);
}

fn initialize_core_satellite_investment() -> RefCell<CoreSatellite> {
    RefCell::new(CoreSatellite::new(
        0.8,                                 // core_trade_amount - initial_asset_amount_to_be_purchased
        0.2,                                 // trade_amount -  this amount is used for trading
        0.0,                                 // core_quantity  - how much asset is purchased
        RefCell::new(AtomicBool::new(true)), // core_to_trade - this is used for first time trading
        0.0,                                 // portfolio -  I don't know
        RefCell::new(Vec::new()),            // investment - investment logs
        RefCell::new(Vec::new()), // real_time_portfolio_value - I guess this is also for logs
        1.0,                      // money_end -  final amount left
    ))
}

fn initialize_rsi_trading_strategy() -> RefCell<RsiTradingStrategy> {
    RefCell::new(RsiTradingStrategy::new(RefCell::new(AtomicBool::new(
        false,
    ))))
}
