use dotenv::dotenv;
use simplelog::*;
use std::fs::File;
pub fn display_contents(elements: &Vec<f64>) {
    info!("Contents of array ::");
    for element in elements {
        info!(" {}", element)
    }
    info!(" ")
}

pub fn load_env() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
    ])
    .unwrap();

    info!("Loading .env variables!!");
    dotenv().ok();
}

#[allow(dead_code)]
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
