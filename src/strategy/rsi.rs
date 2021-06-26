use crate::utils::utility::display_contents;
use std::sync::atomic::AtomicBool;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

pub const RSI_PERIOD: u8 = 14;
pub const RSI_OVERBOUGHT: f64 = 70.0;
pub const RSI_OVERSOLD: f64 = 30.0;

pub struct RsiTradingStrategy {}

impl RsiTradingStrategy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_rsi_logic(closes: Vec<f64>, in_position: &mut bool) {
        if closes.len() > RSI_PERIOD.into() {
            let result = Self::rsi(&closes);
            display_contents(&result);
            let last_rsi = result.last();
            match last_rsi {
                Some(res) => {
                    println!("the current RSI is {}", res);
                    Self::enter_into_position(res, in_position);
                }
                None => {
                    println!("no RSI result");
                }
            }
        }
    }

    fn enter_into_position(last_rsi: &f64, in_position: &mut bool) {
        if *last_rsi > RSI_OVERBOUGHT {
            println!("Sell! Sell! Sell!");
        }
        if *last_rsi < RSI_OVERSOLD {
            if *in_position {
                println!("We have already bought, no need to do anything :) ");
            } else {
                println!("BUY! BUY! BUY! ");
                //order logic
                *in_position = false;
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
