use std::cell::RefCell;
use std::sync::atomic::AtomicBool;

pub struct CoreSatellite {
    // pub initial_amount: f64,
    pub core_trade_amount_percent: f64,
    pub trade_amount_percent: f64,
    pub core_quantity: f64,
    pub core_to_trade: RefCell<AtomicBool>,
    pub portfolio: f64,
    pub investment: RefCell<Vec<f64>>,
    pub real_time_portfolio_value: RefCell<Vec<f64>>,
    pub money_end: f64,
}

impl<'a> CoreSatellite {
    pub fn new(
        //initial_amount: f64,
        core_trade_amount_percent: f64,
        trade_amount_percent: f64,
        core_quantity: f64,
        core_to_trade: RefCell<AtomicBool>,
        portfolio: f64,
        investment: RefCell<Vec<f64>>,
        real_time_portfolio_value: RefCell<Vec<f64>>,
        money_end: f64,
    ) -> Self {
        CoreSatellite {
            //initial_amount: initial_amount,
            core_trade_amount_percent: core_trade_amount_percent,
            trade_amount_percent: trade_amount_percent,
            core_quantity: core_quantity,
            core_to_trade: core_to_trade,
            portfolio: portfolio,
            investment: investment,
            real_time_portfolio_value: real_time_portfolio_value,
            money_end: money_end,
        }
    }

    pub fn update_for_buy(&mut self, allocated_money: f64, price: f64) {
        let quantity = allocated_money / price;
        self.money_end = self.money_end - quantity * price;
        self.portfolio += quantity;

        if self.investment.borrow().is_empty() {
            self.investment.borrow_mut().push(allocated_money);
        } else {
            let last_invested = self.investment.borrow().last().unwrap() + allocated_money;
            self.investment.borrow_mut().push(last_invested);
        }
    }

    pub fn update_for_sell(&mut self, allocated_money: f64, price: f64) {
        let quantity = allocated_money / price;
        self.money_end += quantity * price;
        self.portfolio -= quantity;

        let last_invested = self.investment.borrow().last().unwrap() - allocated_money;
        self.investment.borrow_mut().push(last_invested);
    }

    pub fn get_core_trade_amount(&self, amount: f64) -> f64 {
        self.core_trade_amount_percent * amount
    }

    pub fn get_trade_amount(&self, amount: f64) -> f64 {
        self.trade_amount_percent * amount
    }
}
