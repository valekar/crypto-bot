use std::cell::RefCell;

pub struct CoreSatellite<'a> {
    initial_amount: Option<f64>,
    core_trade_amount: f64,
    trade_amount: f64,
    core_quantity: f64,
    core_to_trade: &'a mut bool,
    portfolio: f64,
    investment: RefCell<Vec<f64>>,
    real_time_portfolio_value: f64,
    money_end: f64,
}

impl<'a> CoreSatellite<'a> {
    pub fn log_buy(&mut self, allocated_money: f64, price: f64) {
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

    pub fn log_sell(&mut self, allocated_money: f64, price: f64) {
        let quantity = allocated_money / price;
        self.money_end += quantity * price;
        self.portfolio -= quantity;

        let last_invested = self.investment.borrow().last().unwrap() - allocated_money;
        self.investment.borrow_mut().push(last_invested);
    }
}
