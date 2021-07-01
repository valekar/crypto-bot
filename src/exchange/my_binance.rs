use crate::utils::constants::FILLED;
use crate::utils::utility::{StrategyType, TradingStyle, TransactionType};
use crate::CoreSatellite;
use crate::RsiTradingStrategy;
use binance::account::*;
use binance::api::*;
use binance::model::KlineEvent;
use binance::websockets::*;
use log::{info, warn};
use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::sync::atomic::AtomicBool;

pub trait Exchange<'b> {
    fn new(
        account: Account,
        opens: RefCell<Vec<f64>>,
        closes: RefCell<Vec<f64>>,
        highs: RefCell<Vec<f64>>,
        lows: RefCell<Vec<f64>>,
        pairs: &'b str,
        klines: &'b str,
        left_asset_name: &'b str,
        left_asset_percent: f64,
        right_asset_name: &'b str,
        right_asset_percent: f64,
        strategy_type: StrategyType,
        trading_style: TradingStyle,
        core_satellite_investment: CoreSatellite,
    ) -> Self;
    fn kline_websocket(
        &'b self,
        in_position_for_rsi: &'b mut bool,
        first_time_trading: &'b mut bool,
    ) -> WebSockets<'b>;
    fn get_account() -> Result<Account, Box<dyn Error>>;
    fn call_trading(&'b self, in_position_for_rsi: &'b mut bool, first_time_trading: &'b mut bool);
    fn get_asset_balance(&'b self, asset_name: &str) -> f64;
    //fn buy_asset(&self) -> bool;
    //fn sell_asset(&self) -> bool;
}

pub struct MyBinance<'a> {
    pair: &'a str,
    kline: &'a str,
    opens: RefCell<Vec<f64>>,
    closes: RefCell<Vec<f64>>,
    highs: RefCell<Vec<f64>>,
    lows: RefCell<Vec<f64>>,
    account: Account,
    left_asset_name: &'a str,
    left_asset_percent: f64,
    right_asset_name: &'a str,
    right_asset_percent: f64,
    strategy_type: StrategyType,
    trading_style: TradingStyle,
    core_satellite_investment: CoreSatellite,
}

impl<'a> MyBinance<'a> {
    pub fn open_websocket_with_pair(&self, web_socket: &mut WebSockets<'a>) {
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let kline: String = format!("{}{}", self.pair.to_lowercase(), self.kline);
        web_socket.connect(&kline).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    error!("Error: {:?}", err);
                }
            }
        }
    }

    fn store_prices(&'a self, kline_event: KlineEvent) {
        if kline_event.kline.is_final_bar == true {
            info!("candle Close at {} ", kline_event.kline.close);
            // closes.push(kline_event.kline.close.parse().unwrap());
            // display_contents(closes);
            self.closes
                .borrow_mut()
                .push(kline_event.kline.close.parse().unwrap());
            self.opens
                .borrow_mut()
                .push(kline_event.kline.open.parse().unwrap());
            self.highs
                .borrow_mut()
                .push(kline_event.kline.high.parse().unwrap());
            self.lows
                .borrow_mut()
                .push(kline_event.kline.low.parse().unwrap());

            self.display_list();
        }
    }

    pub fn display_list(&self) {
        info!("List of closes");
        let mut log_str: String = "".to_owned();
        for close in self.closes.borrow_mut().iter() {
            let log_close: String = format!("{}, ", close);
            log_str.push_str(&log_close);
        }
        info!("{}", log_str);
    }

    pub fn close_websocket(&self, web_socket: &mut WebSockets<'a>) {
        web_socket.disconnect().unwrap();
    }

    pub fn buy_asset_with_right(&self) -> bool {
        error!("BUY! BUY! BUY! ");
        let buy_amount: f64 = self.get_right_asset_amount();
        warn!("Buying {} amount ", buy_amount);
        self.buy_left_asset_with_amount(buy_amount)
        //true
    }

    pub fn buy_left_asset_with_amount(&self, buy_amount: f64) -> bool {
        let result = self
            .account
            .market_buy_using_quote_quantity(self.pair, buy_amount)
            .unwrap();

        &result.status == FILLED
    }

    pub fn get_right_asset_amount(&self) -> f64 {
        let free_amount = self.get_asset_balance(self.right_asset_name);
        warn!(
            " My account balance {}, Buying the asset using {}",
            free_amount, self.right_asset_name
        );
        let calculated_amount = free_amount * self.right_asset_percent;
        let calculated_amount_str = format!("{:.8}", calculated_amount);
        calculated_amount_str.parse::<f64>().unwrap().floor()
    }

    pub fn sell_left_asset(&self) -> bool {
        error!("Sell! Sell! Sell!");

        let sell_amount: f64 = self.get_left_asset_amount();
        warn!("Selling {} amount ", sell_amount);

        self.sell_left_asset_of_amount(sell_amount)
        //true
    }

    pub fn sell_left_asset_of_amount(&self, sell_amount: f64) -> bool {
        let result = self
            .account
            .market_sell(self.pair, sell_amount - 0.001)
            .unwrap();

        &result.status == FILLED
    }

    pub fn get_left_asset_amount(&self) -> f64 {
        let free_amount = self.get_asset_balance(self.left_asset_name);
        warn!(
            " My account balance {}, Selling the free {}",
            free_amount, self.left_asset_name
        );
        let calculated_amount = free_amount * self.left_asset_percent;
        let calculated_amount_str = format!("{:.3}", calculated_amount);
        calculated_amount_str.parse::<f64>().unwrap().floor()
    }
}

impl<'b> Exchange<'b> for MyBinance<'b> {
    fn new(
        account: Account,
        opens: RefCell<Vec<f64>>,
        closes: RefCell<Vec<f64>>,
        highs: RefCell<Vec<f64>>,
        lows: RefCell<Vec<f64>>,
        pairs: &'b str,
        klines: &'b str,
        buy_asset_name: &'b str,
        buy_asset_percent: f64,
        sell_asset_name: &'b str,
        sell_asset_percent: f64,
        strategy_type: StrategyType,
        trading_style: TradingStyle,
        core_satellite_investment: CoreSatellite,
    ) -> Self {
        MyBinance {
            pair: pairs,
            kline: klines,
            opens: opens,
            closes: closes,
            highs: highs,
            lows: lows,
            account: account,
            left_asset_name: buy_asset_name,
            left_asset_percent: buy_asset_percent,
            right_asset_name: sell_asset_name,
            right_asset_percent: sell_asset_percent,
            strategy_type: strategy_type,
            trading_style: trading_style,
            core_satellite_investment: core_satellite_investment,
        }
    }

    fn kline_websocket(
        &'b self,
        in_position_for_rsi: &'b mut bool,
        first_time_trading: &'b mut bool,
    ) -> WebSockets<'b> {
        let web_socket = WebSockets::new(move |event: WebsocketEvent| {
            if let WebsocketEvent::Kline(kline_event) = event {
                //println!("candle Close at {} ", kline_event.kline.close);
                info!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
                self.store_prices(kline_event);
                self.call_trading(in_position_for_rsi, first_time_trading);

                //self.store_prices(kline_event);
            };
            Ok(())
        });
        web_socket
    }

    fn get_account() -> Result<Account, Box<dyn Error>> {
        let api_key = env::var("BINANCE_CLIENT_API").ok();
        let api_secret = env::var("BINANCE_CLIENT_SECRET").ok();
        Ok(Binance::new(api_key, api_secret))
    }

    fn get_asset_balance(&'b self, asset_name: &str) -> f64 {
        self.account
            .get_balance(asset_name)
            .unwrap()
            .free
            .parse::<f64>()
            .unwrap()
    }

    fn call_trading(&'b self, in_position_for_rsi: &mut bool, first_time_trading: &mut bool) {
        //if let StrategyType::RSI = strategy_type {}

        match self.strategy_type {
            StrategyType::Rsi(use_in_position) => {
                warn!("Using IN POSITION RSI Strategy");
                let transaction_type =
                    RsiTradingStrategy::call_rsi_logic(self.closes.borrow_mut().to_vec());

                if use_in_position {
                    // use in position logic

                    match transaction_type {
                        TransactionType::Sell => {
                            let result = self.sell_left_asset();

                            if result {
                                *in_position_for_rsi = false;
                            }
                        }

                        TransactionType::Buy => {
                            if *in_position_for_rsi {
                                warn!("We are already in position, need to do anything!");
                            } else {
                                //warn!("We are buy")
                                let result = self.buy_asset_with_right();

                                if result {
                                    *in_position_for_rsi = true;
                                }
                            }
                        }

                        _ => {}
                    }
                } else {
                    //continous RSI
                }
            }
            StrategyType::Engulfing(use_first_time_trade) => if use_first_time_trade {},
        }
    }
}
