
use dotenv::dotenv;

pub fn display_contents(elements: &Vec<f64>) {
    println!("Contents of array ::");
    for element in elements {
        print!(" {}", element)
    }
    println!(" ")
}

pub fn load_env() {
    println!("Loading .env variables!!");
    dotenv().ok();
}

mod paper {
    pub fn buy(
        investment: &'static mut Vec<f64>,
        allocated_money: f64,
        price: f64,
        portfolio: &'static mut f64,
        money_end: &'static mut f64,
    ) -> (&'static mut f64, &'static mut f64, &'static mut Vec<f64>) {
        let quantity = allocated_money / price;

        *money_end -= quantity * price;
        *portfolio += quantity;

        if investment.is_empty() {
            investment.push(allocated_money);
        } else {
            let last_invested = investment.last().unwrap() + allocated_money;
            investment.push(last_invested);
        }

        (portfolio, money_end, investment)
    }

    pub fn sell(
        investment: &'static mut Vec<f64>,
        allocated_money: f64,
        price: f64,
        portfolio: &'static mut f64,
        money_end: &'static mut f64,
    ) -> (&'static mut f64, &'static mut f64, &'static mut Vec<f64>) {
        let quantity = allocated_money / price;

        *money_end += quantity * price;
        *portfolio -= quantity;

        let last_invested = investment.last().unwrap() - allocated_money;
        investment.push(last_invested);

        (portfolio, money_end, investment)
    }
}
