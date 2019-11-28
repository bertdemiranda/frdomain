#![allow(dead_code)]

use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::collections::HashMap;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

enum TaxType {
    Tax,
    Fee,
    Commision,
}

enum TransactionType {
    InterestComputation, 
    Dividend,
}

type Amount = BigDecimal;
#[derive(Default)]
struct Balance {
    amount: Amount,
}

// --------------------------------------

type TaxRates = HashMap<TaxType, Amount>;
struct TaxCalculationTable {
    table: TaxRates,
} 
impl TaxCalculationTable {
    fn get_tax_rates(&self) -> &TaxRates {
        &self.table
    }
}

// --------------------------------------

trait CalculateTax {
    fn tax(&self, tax_on: Amount) -> Amount;
}

struct TaxCalculation {
    table: TaxCalculationTable,
}

impl TaxCalculation {
    pub fn calculate(&self, tax_on: Amount) -> Amount {
        self.table.get_tax_rates()
                    .iter()
                    .map (|(_,r)| TaxCalculation::do_compute(&tax_on, r))
                    .sum ()
    }

    fn do_compute(tax_on: &Amount, rate: &Amount) -> Amount {
        tax_on * rate
    }
}

impl CalculateTax for TaxCalculation {
    fn tax(&self, tax_on: Amount) -> Amount {
        self.calculate(tax_on)
    }
}

struct SingaporeTaxCalculation {
    tax_calculation: TaxCalculation,
}

impl SingaporeTaxCalculation {
    fn calculate_gst(tax: Amount, gst_rate: Amount) -> Amount {
        tax * gst_rate
    }
}

impl CalculateTax for SingaporeTaxCalculation {
    fn tax(&self, tax_on: Amount) -> Amount {
        let gst_rate = bigdec("0.2");
        self.tax_calculation.calculate(SingaporeTaxCalculation::calculate_gst(tax_on, gst_rate))
    }
}

// ---------------------------------------

struct InterestCalculation {
    calculate_tax: dyn CalculateTax,
}

impl InterestCalculation {
    fn interest(b: Balance) -> Option<Amount> {
        Some(b.amount * bigdec("0.05"))
    }

    pub fn calculate(&self, balance: Balance) -> Option<Amount> {
        InterestCalculation::interest(balance).map(|i| i.clone()-self.calculate_tax.tax(i))
    }
}

// ---------------------------------------

fn main() {
    println!("Hello, taxes!");
}
