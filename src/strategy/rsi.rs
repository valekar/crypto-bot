use crate::exchange::my_binance::MyBinance;
use crate::utils::utility::display_contents;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

pub const RSI_PERIOD: u8 = 3;
pub const RSI_OVERBOUGHT: f64 = 70.0;
pub const RSI_OVERSOLD: f64 = 30.0;

pub enum StrategyType {
    RSI(bool),
    ENGULFING,
}
#[derive(PartialEq)]
pub enum TransactionType {
    SELL,
    BUY,
    HOLD,
}

pub struct RsiTradingStrategy {}

impl RsiTradingStrategy {
    pub fn call_rsi_logic(closes: Vec<f64>) -> TransactionType {
        info!("No of closes {}", closes.len());
        if closes.len() > RSI_PERIOD.into() {
            let result = Self::rsi(&closes);
            display_contents(&result);
            let last_rsi = result.last();
            match last_rsi {
                Some(res) => {
                    info!("the current RSI is {}", res);
                    return RsiTradingStrategy::enter_into_position(res);
                }
                None => {
                    info!("no RSI result");
                }
            }
        }

        TransactionType::HOLD
    }

    fn enter_into_position(last_rsi: &f64) -> TransactionType {
        if *last_rsi > RSI_OVERBOUGHT {
            return TransactionType::BUY;
        }
        if *last_rsi < RSI_OVERSOLD {
            return TransactionType::SELL;
        }

        TransactionType::HOLD
    }

    fn rsi(close_prices: &Vec<f64>) -> Vec<f64> {
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
