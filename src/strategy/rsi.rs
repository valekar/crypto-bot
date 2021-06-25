use crate::utils::constants::RSI_PERIOD;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

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
