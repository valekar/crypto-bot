use crate::strategy::engulfing::Engulfing;
use crate::utils::constants::FILLED;
use crate::utils::utility::{display_contents, StrategyType, TradingStyle, TransactionType};
use crate::CoreSatellite;
use crate::RsiTradingStrategy;
use binance::account::*;
use binance::api::*;
use binance::errors::BinanceContentError;
use binance::general::*;
use binance::model::KlineEvent;
use binance::websockets::*;
use log::{info, warn};
use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::sync::atomic::AtomicBool;

pub struct Asset<'b> {
    pub name: &'b str,
    price_percent: f64,
    precision: usize,
    min_qty: f64,
}

impl<'b> Asset<'b> {
    pub fn new(name: &'b str, price_percent: f64, precision: usize, min_qty: f64) -> Self {
        Self {
            name: name,
            price_percent: price_percent,
            precision: precision,
            min_qty: min_qty,
        }
    }
}

pub trait Exchange<'b> {
    fn new(
        account: Account,
        opens: RefCell<Vec<f64>>,
        closes: RefCell<Vec<f64>>,
        highs: RefCell<Vec<f64>>,
        lows: RefCell<Vec<f64>>,
        pairs: &'b str,
        klines: &'b str,
        base_asset: Asset<'b>,
        quote_asset: Asset<'b>,
        strategy_type: StrategyType,
        trading_style: TradingStyle,
        core_satellite_investment: RefCell<CoreSatellite>,
        rsi_trading_strategy: RsiTradingStrategy,
    ) -> Self;
    fn kline_websocket(&'b self) -> WebSockets<'b>;
    fn get_account() -> Result<Account, Box<dyn Error>>;
    fn call_trading(&'b self);
    fn call_trading_at_candle_close(&'b self);
    fn get_asset_free_balance(&'b self, asset_name: &str) -> f64;
    fn get_general() -> Result<General, Box<dyn Error>>;
    fn get_base_asset_precision(&'b self);
    fn set_initial_trading_amounts(&'b self);
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
    base_asset: Asset<'a>,
    quote_asset: Asset<'a>,
    strategy_type: StrategyType,
    trading_style: TradingStyle,
    core_satellite_investment: RefCell<CoreSatellite>,
    rsi_trading_strategy: RsiTradingStrategy,
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

    fn store_prices_at_candle_close(&'a self, kline_event: KlineEvent) {
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

    fn display_list(&self) {
        info!("List of closes");
        display_contents(&self.closes.borrow());

        info!("List of Opens");
        display_contents(&self.opens.borrow());

        info!("List of Highs");
        display_contents(&self.highs.borrow());

        info!("List of Lows");
        display_contents(&self.lows.borrow());
    }

    pub fn close_websocket(&self, web_socket: &mut WebSockets<'a>) {
        web_socket.disconnect().unwrap();
    }

    fn buy_base_asset_with_right(&self, calculated_amount: f64) -> bool {
        error!("BUY! BUY! BUY! ");
        let buy_amount: f64 = self.get_quote_asset_adjusted_amount(calculated_amount);
        warn!("Buying left asset with {} amount of right", buy_amount);
        self.buy_base_asset_with_amount(buy_amount).unwrap_or(false)
        //true
    }

    fn buy_base_asset_with_amount(&self, buy_amount: f64) -> Result<bool, BinanceContentError> {
        if buy_amount <= 0.0 || buy_amount <= self.quote_asset.min_qty {
            error!("The buy amount is {} ", buy_amount);
            error!("Couldn't buy :'( cause amount is zero or less or it doesn't meet minimum amount requirement to buy ");
            Ok(false)
        } else {
            let result = self
                .account
                .market_buy_using_quote_quantity(self.pair, buy_amount)
                .unwrap();
            warn!(
                "Bought asset at {} price",
                self.closes.borrow().last().unwrap_or(&0.0)
            );
            Ok(&result.status == FILLED)
            // Ok(true)
        }
    }

    fn get_quote_asset_adjusted_amount(&self, quote_asset_amount: f64) -> f64 {
        let calculated_amount_str = format!(
            "{:.precision$}",
            quote_asset_amount,
            precision = self.quote_asset.precision as usize
        );

        match calculated_amount_str.parse::<f64>() {
            Ok(result) => result,
            Err(e) => {
                error!(" Parse error {}", e);
                0.0
            }
        }
    }

    fn get_quote_asset_amount(&self) -> f64 {
        let free_amount = self.get_asset_free_balance(self.quote_asset.name);
        info!(
            " My account balance {}, Buying the asset using {}",
            free_amount, self.quote_asset.name
        );
        free_amount * self.quote_asset.price_percent
    }

    fn get_base_asset_adjusted_amount(&self, base_asset_amount: f64) -> f64 {
        let calculated_amount_str = format!(
            "{:.precision$}",
            base_asset_amount,
            precision = self.base_asset.precision as usize
        );
        //let calculated_amount_str = format!("{:.3}", base_asset_amount);
        calculated_amount_str.parse::<f64>().unwrap()
    }
    fn sell_base_asset_of_amount(&self, sell_amount: f64) -> Result<bool, BinanceContentError> {
        if sell_amount <= 0.0 || sell_amount <= self.base_asset.min_qty {
            error!("The sell amount is {} ", sell_amount);
            error!("Couldn't Sell :'( cause amount is zero or less or minimum order quantity for the exchange");

            Ok(false)
        } else {
            let result = self.account.market_sell(self.pair, sell_amount).unwrap();

            warn!(
                "Sold asset at {} price",
                self.closes.borrow().last().unwrap_or(&0.0)
            );
            Ok(&result.status == FILLED)
            //Ok(true)
        }
    }
    fn sell_base_asset(&self, calculated_amount: f64) -> bool {
        error!("Sell! Sell! Sell!");
        let sell_amount: f64 = self.get_base_asset_adjusted_amount(calculated_amount);
        warn!("Selling {} amount ", sell_amount);

        self.sell_base_asset_of_amount(sell_amount).unwrap_or(false)
        //true
    }

    fn get_base_asset_amount(&self) -> f64 {
        let free_amount = self.get_asset_free_balance(self.base_asset.name);
        info!(
            " My account balance {}, Asset name is {}",
            free_amount, self.base_asset.name
        );
        free_amount * self.base_asset.price_percent
    }

    fn is_final_bar(&self, kline_event: KlineEvent) -> bool {
        kline_event.kline.is_final_bar
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
        base_asset: Asset<'b>,
        quote_asset: Asset<'b>,
        strategy_type: StrategyType,
        trading_style: TradingStyle,
        core_satellite_investment: RefCell<CoreSatellite>,
        rsi_trading_strategy: RsiTradingStrategy,
    ) -> Self {
        MyBinance {
            pair: pairs,
            kline: klines,
            opens: opens,
            closes: closes,
            highs: highs,
            lows: lows,
            account: account,
            base_asset: base_asset,
            quote_asset: quote_asset,
            strategy_type: strategy_type,
            trading_style: trading_style,
            core_satellite_investment: core_satellite_investment,
            rsi_trading_strategy: rsi_trading_strategy,
        }
    }

    fn kline_websocket(&'b self) -> WebSockets<'b> {
        let web_socket = WebSockets::new(move |event: WebsocketEvent| {
            if let WebsocketEvent::Kline(kline_event) = event {
                //println!("candle Close at {} ", kline_event.kline.close);
                info!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );

                let kline_event_1 = kline_event.clone();
                let kline_event_2 = kline_event.clone();

                self.store_prices_at_candle_close(kline_event_1);
                self.call_trading();
                if self.is_final_bar(kline_event_2) {
                    self.call_trading_at_candle_close();
                }
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

    fn get_general() -> Result<General, Box<dyn Error>> {
        Ok(Binance::new(None, None))
    }

    fn get_base_asset_precision(&'b self) {
        //let general: General = Exchange::get_general().unwrap();
    }

    fn get_asset_free_balance(&'b self, asset_name: &str) -> f64 {
        self.account
            .get_balance(asset_name)
            .unwrap()
            .free
            .parse::<f64>()
            .unwrap()

        //self.account.
    }

    fn set_initial_trading_amounts(&'b self) {
        self.core_satellite_investment
            .borrow_mut()
            .set_trading_amounts(self.get_asset_free_balance(self.quote_asset.name));
    }

    fn call_trading(&self) {
        if let StrategyType::Rsi(use_in_position) = self.strategy_type {
            warn!("Using IN POSITION RSI Strategy");
            let transaction_type = self
                .rsi_trading_strategy
                .call_rsi_logic(self.closes.borrow_mut().to_vec());

            if use_in_position {
                // use in position logic

                warn!("In use pos");
                match transaction_type {
                    TransactionType::Sell => {
                        let calculated_amount = self.get_base_asset_amount();
                        let result = self.sell_base_asset(calculated_amount);

                        if result {
                            *self.rsi_trading_strategy.in_position.borrow_mut().get_mut() = false;
                        }
                    }

                    TransactionType::Buy => {
                        if *self.rsi_trading_strategy.in_position.borrow_mut().get_mut() {
                            warn!("We are already in position, need to do anything!");
                        } else {
                            //warn!("We are buy")
                            let calculated_amount = self.get_quote_asset_amount();
                            let result = self.buy_base_asset_with_right(calculated_amount);

                            if result {
                                *self.rsi_trading_strategy.in_position.borrow_mut().get_mut() =
                                    true;
                            }
                        }
                    }

                    _ => {}
                }
            } else {
                //continous RSI
            }
        }
    }

    fn call_trading_at_candle_close(&self) {
        // if kline_event.kline.is_final_bar {
        if let StrategyType::Engulfing(use_first_time_trade) = self.strategy_type {
            if use_first_time_trade
                && *self
                    .core_satellite_investment
                    .borrow_mut()
                    .core_to_trade
                    .borrow_mut()
                    .get_mut()
                && self.closes.borrow().len() > 0
            {
                warn!("Using first time trading (Core to Trade)");
                let quote_asset_amount = self.get_quote_asset_amount();
                warn!("Right asset amount {}", quote_asset_amount);
                let core_trade_amount = self.core_satellite_investment.borrow().core_trade_amount;

                if *self
                    .core_satellite_investment
                    .borrow_mut()
                    .core_to_trade
                    .borrow_mut()
                    .get_mut()
                {
                    let result = self.buy_base_asset_with_right(core_trade_amount);
                    if result {
                        *self
                            .core_satellite_investment
                            .borrow_mut()
                            .core_to_trade
                            .borrow_mut()
                            .get_mut() = false;

                        // update core_satellite's values
                        self.core_satellite_investment.borrow_mut().core_quantity =
                            self.get_asset_free_balance(self.base_asset.name);
                        self.core_satellite_investment.borrow_mut().update_for_buy(
                            core_trade_amount,
                            *self.closes.borrow().last().unwrap_or(&0.0),
                        );
                    }
                }
            }

            if self.closes.borrow().len() > 0
                && !*self
                    .core_satellite_investment
                    .borrow()
                    .core_to_trade
                    .borrow_mut()
                    .get_mut()
            {
                let engulfing_value = Engulfing::engulfing(
                    self.closes.borrow().to_vec(),
                    self.opens.borrow().to_vec(),
                    self.highs.borrow().to_vec(),
                    self.lows.borrow().to_vec(),
                );

                let engulfing_list = &engulfing_value.unwrap();
                display_contents(&engulfing_list);

                let last_engulfing = engulfing_list.last().unwrap_or(&0);

                let amount = (self.core_satellite_investment.borrow().trade_amount
                    * (*last_engulfing as f64))
                    / 100.0;
                let last_close_price = *self.closes.borrow().last().unwrap();

                let port_value = (self.core_satellite_investment.borrow().portfolio
                    - self.core_satellite_investment.borrow().core_quantity)
                    * last_close_price;

                let mut trade_amount = amount - port_value;

                info!("last close {}", last_close_price);
                info!(
                    "portfolio {}",
                    self.core_satellite_investment.borrow().portfolio
                );
                info!("trade_amount {}", trade_amount);
                info!("amount {}", amount);
                info!("port_value {}", port_value);
                info!(
                    "core_quantity {}",
                    self.core_satellite_investment.borrow().core_quantity
                );

                if *last_engulfing == 0 {
                    trade_amount = 0.0;
                }
                let money_end = self.core_satellite_investment.borrow().money_end;
                let portfolio = self.core_satellite_investment.borrow().portfolio;
                self.core_satellite_investment
                    .borrow_mut()
                    .real_time_portfolio_value
                    .borrow_mut()
                    .push(money_end + portfolio * last_close_price);

                warn!(
                    "The last engulfing value is {} and recommended exposure is {}",
                    last_engulfing, trade_amount
                );
                warn!(
                    "Real time portfolio value is {}",
                    self.core_satellite_investment
                        .borrow()
                        .real_time_portfolio_value
                        .borrow()
                        .last()
                        .unwrap()
                );

                if trade_amount > 0.0 {
                    self.buy_base_asset_with_right(trade_amount);
                    self.core_satellite_investment
                        .borrow_mut()
                        .update_for_buy(trade_amount, *self.closes.borrow().last().unwrap_or(&0.0));
                } else if trade_amount < 0.0 {
                    let base_asset_amount = self.get_base_asset_amount();
                    let calculate_base_amount =
                        base_asset_amount * (-trade_amount / last_close_price);
                    //let calculate_base_amount = -trade_amount / last_close_price;
                    self.sell_base_asset(calculate_base_amount);

                    self.core_satellite_investment.borrow_mut().update_for_sell(
                        trade_amount,
                        *self.closes.borrow().last().unwrap_or(&0.0),
                    );
                } else {
                    // don't do anything!
                }
            }
        }
        //}
    }
}
