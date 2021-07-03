use crate::utils::utility::{display_contents, TransactionType};
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

pub const RSI_PERIOD: u8 = 4;
pub const RSI_OVERBOUGHT: f64 = 70.0;
pub const RSI_OVERSOLD: f64 = 30.0;

pub struct RsiTradingStrategy {
    pub in_position: RefCell<AtomicBool>,
}

impl RsiTradingStrategy {
    pub fn new(in_position: RefCell<AtomicBool>) -> Self {
        Self {
            in_position: in_position,
        }
    }

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

        TransactionType::Hold
    }

    fn enter_into_position(last_rsi: &f64) -> TransactionType {
        if *last_rsi > RSI_OVERBOUGHT {
            return TransactionType::Buy;
        }
        if *last_rsi < RSI_OVERSOLD {
            return TransactionType::Sell;
        }

        TransactionType::Hold
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
