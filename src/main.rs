use std::str::FromStr;

use comfy_table::{CellAlignment, Table};
use currency_rs::{Currency};

#[derive(Debug, Clone, Copy)]
struct TradeSize {
    account_equity: f64,
    price: f64,
    stop_loss: f64,
}

impl TradeSize {
    fn new(account_equity: f64, price: f64, stop_loss: f64) -> TradeSize {
        TradeSize {
            account_equity,
            price,
            stop_loss,
        }
    }

    fn risk_per_unit(&self) -> f64 {
        Currency::new_float(self.price, None).subtract(self.stop_loss).value()
    }

    fn per_unit_risk(&self) -> Currency<'static> {
        Currency::new_float(self.price, None).subtract(self.stop_loss)
    }

    fn risk_equity(&self, risk_percent: f64) -> Currency<'static> {
        let multiplier = risk_percent / 100.0;
        Currency::new_float(self.account_equity, None)
            .multiply(multiplier)
    }

    // number of shares to buy
    fn num_shares(&self, risk_percent: f64) -> u32 {
        (self.risk_equity(risk_percent).value() / self.risk_per_unit()) as u32
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() || args.len() != 4 {
        eprintln!("Usage: ts [ACCOUNT_EQUITY] [PRICE] [STOP_LOSS]");
        std::process::exit(1);
    }

    let account_equity = f64::from_str(&args[1]).expect("Error parsing 'account_equity'");
    let price = f64::from_str(&args[2]).expect("Error parsing 'price'");
    let stop_loss = f64::from_str(&args[3]).expect("Error parsing 'stop_loss'");
    let trade_size = TradeSize::new(account_equity, price, stop_loss);

    input_table(trade_size);
    risk_table(trade_size);
}

fn input_table(ts: TradeSize) {
    let mut input_table = Table::new();
    const INPUT_HEADERS: [&str; 4] = ["Equity", "Price", "Stop-loss", "Per-unit Risk"];
    input_table
        .set_header(INPUT_HEADERS)
        .add_row(vec![
            Currency::new_float(ts.account_equity, None).format().as_str(),
            Currency::new_float(ts.price, None).format().as_str(),
            Currency::new_float(ts.stop_loss, None).format().as_str(),
            ts.per_unit_risk().format().as_str(),
        ])
    ;

    // right justify all columns in input_table
    for (_, column) in input_table.column_iter_mut().enumerate() {
        column.set_cell_alignment(CellAlignment::Right);
    }

    println_padded("Inputs:");
    println!("{input_table}");
}

fn risk_table(ts: TradeSize) {
    let mut risk_table = Table::new();
    const RISK_HEADERS: [&str; 3] = ["% Risk", "Risk Equity", "Shares"];
    const RISK_INCREMENT: f64 = 0.25;
    const MAX_RISK_PERCENT: f64 = 2.0;
    let mut risk_percent: f64 = 1.0;
    while risk_percent <= MAX_RISK_PERCENT {
        risk_table.set_header(RISK_HEADERS);
        risk_table.add_row(vec![
            format!("{pct:.*}", 2, pct = risk_percent),
            ts.risk_equity(risk_percent).format(),
            ts.num_shares(risk_percent).to_string(),
        ]);
        risk_percent += RISK_INCREMENT;
    }

    // right justify all columns in risk_table
    for (_, column) in risk_table.column_iter_mut().enumerate() {
        column.set_cell_alignment(CellAlignment::Right);
    }

    println_padded("Outputs:");
    println!("{risk_table}");
}

fn println_padded(msg: &str) {
    println!();
    println!("{msg}");
}
