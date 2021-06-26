use crate::utils::utility::display_contents;
use crate::RsiTradingStrategy;
use binance::model::KlineEvent;
use binance::websockets::*;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
trait Market {
    fn open_websocket(&self);
}

pub struct Binance<'a> {
    pair: &'a str,
    kline: &'a str,
    closes: RefCell<Vec<f64>>,
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

    pub fn kline_websocket(binance: &'a Binance, in_position: &'a mut bool) -> WebSockets<'a> {
        let web_socket = WebSockets::new(move |event: WebsocketEvent| {
            if let WebsocketEvent::Kline(kline_event) = event {
                //println!("candle Close at {} ", kline_event.kline.close);
                println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
                binance.store_final_close_prices(kline_event);
                binance.start_trading(in_position);
            };
            Ok(())
        });
        web_socket
    }

    pub fn store_final_close_prices(&self, kline_event: KlineEvent) {
        // if kline_event.kline.is_final_bar == true {
        println!("candle Close at {} ", kline_event.kline.close);
        // closes.push(kline_event.kline.close.parse().unwrap());
        // display_contents(closes);
        self.closes
            .borrow_mut()
            .push(kline_event.kline.close.parse().unwrap());

        self.display_list();
        //}
    }

    pub fn start_trading(&self, in_position: &mut bool) {
        RsiTradingStrategy::start_rsi_logic(self.closes.borrow_mut().to_vec(), in_position)
    }

    pub fn display_list(&self) {
        for ii in self.closes.borrow_mut().iter() {
            println!("{}", ii)
        }
    }

    pub fn close_websocket(&self, web_socket: &mut WebSockets<'a>) {
        web_socket.disconnect().unwrap();
    }

    pub fn is_closes_greater_than(&self, number: usize) -> bool {
        self.closes.borrow_mut().len() > number
    }

    pub fn new(closes: RefCell<Vec<f64>>, pair: &'a str, kline: &'a str) -> Self {
        Binance {
            // close_prices: close_prices,
            pair: pair,
            kline: kline,
            closes: closes,
        }
    }
}
