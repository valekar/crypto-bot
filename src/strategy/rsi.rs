use crate::exchange::my_binance::MyBinance;
use crate::utils::constants::{BNB, BUSD, FILLED};
use crate::utils::utility::display_contents;
use binance::account::*;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

pub const RSI_PERIOD: u8 = 14;
pub const RSI_OVERBOUGHT: f64 = 70.0;
pub const RSI_OVERSOLD: f64 = 30.0;

pub struct RsiTradingStrategy<'a> {
    pair: &'a str,
    account: &'a Account,
    buy_asset: &'a str,
    buy_asset_percent: f64,
    sell_asset: &'a str,
    sell_asset_percent: f64,
}

pub enum StrategyType {
    RSI,
    ENGULFING,
}

impl<'a> RsiTradingStrategy<'a> {
    pub fn new(
        pair: &'a str,
        account: &'a Account,
        buy_asset: &'a str,
        buy_asset_percent: f64,
        sell_asset: &'a str,
        sell_asset_percent: f64,
    ) -> Self {
        Self {
            pair: pair,
            account: account,
            buy_asset: buy_asset,
            buy_asset_percent: buy_asset_percent,
            sell_asset: sell_asset,
            sell_asset_percent: sell_asset_percent,
        }
    }
    pub fn start_rsi_logic_for_binance(&self, closes: Vec<f64>, in_position: &mut bool) {
        if closes.len() > RSI_PERIOD.into() {
            let result = Self::rsi(&closes);
            display_contents(&result);
            let last_rsi = result.last();
            match last_rsi {
                Some(res) => {
                    info!("the current RSI is {}", res);
                    self.enter_into_position(res, in_position);
                }
                None => {
                    info!("no RSI result");
                }
            }
        }
    }

    fn enter_into_position(&self, last_rsi: &f64, in_position: &mut bool) {
        if *last_rsi > RSI_OVERBOUGHT {
            let status = MyBinance::buy_asset(
                self.pair,
                self.buy_asset,
                self.buy_asset_percent,
                self.account,
            );

            if status {
                *in_position = false;
            }
        }
        if *last_rsi < RSI_OVERSOLD {
            if *in_position {
                warn!("We have already bought, no need to do anything :) ");
            } else {
                let status = MyBinance::sell_asset(
                    self.pair,
                    self.sell_asset,
                    self.sell_asset_percent,
                    self.account,
                );

                if status {
                    *in_position = true;
                }
            }
        }
    }

    pub fn rsi(close_prices: &Vec<f64>) -> Vec<f64> {
        let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());
        let mut out_begin: i32 = 0;
        let mut out_size: i32 = 0;
        unsafe {
            let ret_code = TA_RSI(
                0,
                close_prices.len() as i32 - 1,
                close_prices.as_ptr(),
                RSI_PERIOD.into(),
                &mut out_begin,
                &mut out_size,
                out.as_mut_ptr(),
            );
            match ret_code {
                TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
                _ => panic!("Could not compute indicator, err: {:?}", ret_code),
            }
        }
        out
    }
}
