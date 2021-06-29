use crate::strategy::rsi::StrategyType;
use crate::utils::constants::FILLED;
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
        closes: RefCell<Vec<f64>>,
        pairs: &'b str,
        klines: &'b str,
        buy_asset_name: &'b str,
        buy_asset_percent: f64,
        sell_asset_name: &'b str,
        sell_asset_percent: f64,
    ) -> Self;
    fn kline_websocket(&self, binance: &'b MyBinance) -> WebSockets<'b>;
    fn get_account() -> Result<Account, Box<dyn Error>>;
    fn start_trading(&self, strategy_type: StrategyType, in_position: &'b mut bool);
    //fn buy_asset(&self) -> bool;
    //fn sell_asset(&self) -> bool;
}

pub struct MyBinance<'a> {
    pair: &'a str,
    kline: &'a str,
    closes: RefCell<Vec<f64>>,
    account: Account,
    buy_asset_name: &'a str,
    buy_asset_percent: f64,
    sell_asset_name: &'a str,
    sell_asset_percent: f64,
}

impl<'a> MyBinance<'a> {
    pub fn open_websocket_with_pair(&self, web_socket: &mut WebSockets<'a>) {
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let kline: String = format!("{}{}", self.pair.to_lowercase(), self.kline);
        web_socket.connect(&kline).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }

    fn store_final_close_prices(&self, kline_event: KlineEvent) {
        if kline_event.kline.is_final_bar == true {
            info!("candle Close at {} ", kline_event.kline.close);
            // closes.push(kline_event.kline.close.parse().unwrap());
            // display_contents(closes);
            self.closes
                .borrow_mut()
                .push(kline_event.kline.close.parse().unwrap());

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

    pub fn buy_asset_with(&self) -> bool {
        let my_balance = self.account.get_balance(self.sell_asset_name).unwrap();
        warn!("BUY! BUY! BUY! ");
        warn!(
            " My account balance {}, Buying the asset using {}",
            my_balance.free, self.sell_asset_name
        );

        let buy_amount_str = format!(
            "{:.8}",
            (my_balance.free.parse::<f64>().unwrap() * self.buy_asset_percent)
        );
        let buy_amount: f64 = buy_amount_str.parse::<f64>().unwrap().floor();

        warn!("Buying {} amount ", buy_amount);

        let result = self
            .account
            .market_buy_using_quote_quantity(self.pair, buy_amount)
            .unwrap();

        &result.status == FILLED
    }
    pub fn sell_asset(&self) -> bool {
        let my_balance = self.account.get_balance(self.buy_asset_name).unwrap();
        warn!("Sell! Sell! Sell!");
        warn!(
            " My account balance {}, Selling the free {}",
            my_balance.free, self.buy_asset_name
        );

        let sell_amount_str: String = format!(
            "{:.3}",
            (my_balance.free.parse::<f64>().unwrap() * self.sell_asset_percent)
        );

        let sell_amount: f64 = sell_amount_str.parse::<f64>().unwrap();
        warn!("Selling {} amount ", sell_amount);

        let result = self
            .account
            .market_sell(self.pair, sell_amount - 0.001)
            .unwrap();

        &result.status == FILLED
    }
}

impl<'b> Exchange<'b> for MyBinance<'b> {
    fn new(
        account: Account,
        closes: RefCell<Vec<f64>>,
        pairs: &'b str,
        klines: &'b str,
        buy_asset_name: &'b str,
        buy_asset_percent: f64,
        sell_asset_name: &'b str,
        sell_asset_percent: f64,
    ) -> Self {
        MyBinance {
            pair: pairs,
            kline: klines,
            closes: closes,
            account: account,
            buy_asset_name: buy_asset_name,
            buy_asset_percent: buy_asset_percent,
            sell_asset_name: sell_asset_name,
            sell_asset_percent: sell_asset_percent,
        }
    }

    fn kline_websocket(&self, binance: &'b MyBinance) -> WebSockets<'b> {
        let web_socket = WebSockets::new(move |event: WebsocketEvent| {
            if let WebsocketEvent::Kline(kline_event) = event {
                //println!("candle Close at {} ", kline_event.kline.close);
                info!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
                binance.store_final_close_prices(kline_event);
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

    fn start_trading(&self, strategy_type: StrategyType, in_position: &'b mut bool) {
        if let StrategyType::RSI = strategy_type {
            warn!("Using RSI Strategy");
            let rsi_trading_strategy = RsiTradingStrategy::new(Some(self));
            rsi_trading_strategy
                .start_rsi_logic_for_binance(self.closes.borrow_mut().to_vec(), in_position)
        }
    }
}
