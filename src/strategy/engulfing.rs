use ta_lib_wrapper::{TA_Integer, TA_Real, TA_RetCode, TA_CDLENGULFING};

pub struct Engulfing {}

impl Engulfing {
    pub fn engulfing(
        closes: Vec<TA_Real>,
        opens: Vec<TA_Real>,
        highs: Vec<TA_Real>,
        lows: Vec<TA_Real>,
    ) -> Option<Vec<i32>> {
        let mut out_begin: TA_Integer = 0;
        let mut out_size: TA_Integer = 0;
        let mut out: Vec<i32> = Vec::with_capacity(closes.len());

        unsafe {
            let ret_code = TA_CDLENGULFING(
                0,
                closes.len() as i32 - 1,
                opens.as_ptr(),
                highs.as_ptr(),
                lows.as_ptr(),
                closes.as_ptr(),
                &mut out_begin,
                &mut out_size,
                out.as_mut_ptr(),
            );

            match ret_code {
                TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
                _ => panic!("Could not compute indicator, err: {:?}", ret_code),
            }
        }
        Some(out)
    }
}
