//#![allow(dead_code)]

#[macro_use]
extern crate log;
//extern crate pretty_env_logger;
use binance::general::*;
use binance::model::{Filters, Filters::*};
use exchange::my_binance::{Asset, Exchange, MyBinance};
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

    //use_rsi_trading_strategy();

    /*
     * Core satellite strategy
     */
    //
    use_engulfing_trading_strategy();
}

fn use_rsi_trading_strategy() {
    let core_satellite_investment = initialize_core_satellite_investment();

    let account = MyBinance::get_account();
    let (base_asset, quote_asset) = get_assets();

    let my_binance: MyBinance = Exchange::new(
        account.unwrap(),
        RefCell::new(Vec::new()),          // opens
        RefCell::new(Vec::new()),          // closes
        RefCell::new(Vec::new()),          // highs
        RefCell::new(Vec::new()),          // lows
        BNB_BUSD,                          // pair
        ONE_MINUTE_KLINE,                  // kline
        base_asset,                        // base asset details
        quote_asset,                       // quote asset details
        StrategyType::Rsi(true),           // strategy type
        TradingStyle::CoreSatellite,       // trading style
        core_satellite_investment,         // Core satellite instance
        initialize_rsi_trading_strategy(), // Rsi Trading Strategy instance
    );

    //let result = my_binance.buy_asset_with_stable();
    // warn!("IS the asset bought?? {}", result);

    // let result = my_binance.sell_asset();
    // warn!("IS the asset sold?? {}", result);
    my_binance.set_initial_trading_amounts();

    let mut web_socket = my_binance.kline_websocket();

    my_binance.open_websocket_with_pair(&mut web_socket);
    my_binance.close_websocket(&mut web_socket);
}

fn use_engulfing_trading_strategy() {
    info!("Using Core Satellite trading style with Engulfing Trading Strategy");

    let core_satellite_investment = initialize_core_satellite_investment();
    let account_1 = MyBinance::get_account();

    let (base_asset, quote_asset) = get_assets();

    let my_binance_1: MyBinance = Exchange::new(
        account_1.unwrap(),
        RefCell::new(Vec::new()),          // opens
        RefCell::new(Vec::new()),          // closes
        RefCell::new(Vec::new()),          // highs
        RefCell::new(Vec::new()),          // lows
        BNB_BUSD,                          // pair
        ONE_MINUTE_KLINE,                  // kline
        base_asset,                        // base asset details
        quote_asset,                       // quote asset details
        StrategyType::Engulfing(true),     // strategy type
        TradingStyle::CoreSatellite,       // Trading style
        core_satellite_investment,         // Core Satellite Instance
        initialize_rsi_trading_strategy(), // Rsi Trading Strategy Instance
    );

    my_binance_1.set_initial_trading_amounts();

    let mut web_socket = my_binance_1.kline_websocket();
    my_binance_1.open_websocket_with_pair(&mut web_socket);
    my_binance_1.close_websocket(&mut web_socket);
}

fn initialize_core_satellite_investment() -> RefCell<CoreSatellite> {
    RefCell::new(CoreSatellite::new(
        0.0,                                 // core_trade_amount - initial asset amount to be purchased
        0.6, // core_trade_amount_percent - initial asset amount to be purchased with percentage
        0.0, // trade_amount - Trading absolute amount
        0.4, // trade_amount percent -  this amount is used for trading
        0.0, // core_quantity  - how much asset is purchased
        RefCell::new(AtomicBool::new(true)), // core_to_trade - this is used for first time trading
        0.0, // portfolio -  I don't know
        RefCell::new(Vec::new()), // investment - investment logs
        RefCell::new(Vec::new()), // real_time_portfolio_value - I guess this is also for logs
        0.0, // money_end -  final amount base
    ))
}

fn initialize_rsi_trading_strategy() -> RsiTradingStrategy {
    RsiTradingStrategy::new(RefCell::new(AtomicBool::new(false)))
}

fn get_assets<'a>() -> (Asset<'a>, Asset<'a>) {
    let general: General = MyBinance::get_general().unwrap();
    let result = general.get_symbol_info(BNB_BUSD);
    match result {
        Ok(answer) => {
            let (base_min_qty, quote_min_qty) = get_min_quantities(answer.filters.clone());
            println!("Symbol information: {:?}", answer);
            (
                Asset::new(
                    BNB, 1.0, 4,    //answer.base_asset_precision as usize,
                    0.05, //base_min_qty.parse::<f64>().unwrap(),
                ),
                Asset::new(
                    BUSD,
                    1.0,
                    answer.quote_precision as usize,
                    quote_min_qty.parse::<f64>().unwrap(),
                ),
            )
        }
        Err(e) => {
            info!("Error: {}", e);
            (
                Asset::new(BNB, 1.0, 3, 0.05),
                Asset::new(BUSD, 0.3, 8, 10.0),
            )
        }
    }
}

fn get_min_quantities(filters: Vec<Filters>) -> (String, String) {
    let mut base_min_qty = String::new();
    let mut quote_min_qty = String::new();
    for filter in filters.iter() {
        match filter {
            Filters::MinNotional {
                notional: _,
                min_notional,
                apply_to_market: _,
                avg_price_mins: _,
            } => {
                quote_min_qty = min_notional.as_ref().unwrap().to_string();
                info!("Min buy size {}", min_notional.as_ref().unwrap()) // this buy size
            }

            Filters::LotSize {
                min_qty,
                max_qty: _,
                step_size: _,
            } => {
                base_min_qty = min_qty.to_string();
                info!("Min Sell quantity {}", min_qty) // this is sell quantity size
            }

            _ => {}
        }
        // println!("---------{}{}", base_min_qty, quote_min_qty);
    }
    (base_min_qty, quote_min_qty)
}

// minimum  notion -> this the order size , this means quote asset should have atleast this amount
// trade amount / selling base asset - this the LotSize
