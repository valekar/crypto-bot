use binance::model::KlineEvent;
use binance::websockets::*;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
//use std::sync::atomic::
use crate::utils::utility::display_contents;
trait Market {
    fn open_websocket(&self);
}

pub struct Binance<'a> {
    // close_prices: RefCell<Vec<f64>>,
    pair: &'a str,
    kline: &'a str,
}

impl<'a> Binance<'a> {
    pub fn open_websocket(&self, web_socket: &mut WebSockets<'a>) {
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let kline: String = format!("{}{}", self.pair, self.kline);
        web_socket.connect(&kline).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }

    pub fn create_kline_and_handle_websocket_event(binance: &'a Binance) -> WebSockets<'a> {
        let web_socket = WebSockets::new(move |event: WebsocketEvent| {
            if let WebsocketEvent::Kline(kline_event) = event {
                //println!("candle Close at {} ", kline_event.kline.close);
                println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
                binance.process_klines(kline_event);
            };
            Ok(())
        });
        web_socket
    }

    pub fn process_klines(&self, kline_event: KlineEvent) {
        let closes: &mut Vec<f64> = &mut Vec::new();
        if kline_event.kline.is_final_bar == true {
            println!("candle Close at {} ", kline_event.kline.close);
            // self.close_prices
            //     .borrow_mut()
            //     .push(kline_event.kline.close.parse().unwrap());
            // self.display_contents();

            closes.push(kline_event.kline.close.parse().unwrap());
            display_contents(closes);
        }
    }

    // pub fn display_contents(&self) {
    //     for ii in self.close_prices.borrow_mut().iter_mut() {
    //         println!("{}", ii);
    //     }
    // }

    pub fn close_websocket(&self, web_socket: &mut WebSockets<'a>) {
        web_socket.disconnect().unwrap();
    }

    pub fn new(pair: &'a str, kline: &'a str) -> Self {
        Binance {
            // close_prices: close_prices,
            pair: pair,
            kline: kline,
        }
    }
}

// pub fn create_kline_and_handle_websocket_event<'a>(binance: &'a Binance) -> WebSockets<'a> {
//     let web_socket = WebSockets::new(move |event: WebsocketEvent| {
//         if let WebsocketEvent::Kline(kline_event) = event {
//             //println!("candle Close at {} ", kline_event.kline.close);
//             println!(
//                 "Symbol: {}, high: {}, low: {}",
//                 kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
//             );

//             binance.process_kline(kline_event);
//         };
//         Ok(())
//     });
//     web_socket
// }
