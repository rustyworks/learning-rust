struct AverageBookPrice {
    total_book: i32,
    average_price: f64,
}

impl AverageBookPrice {
    fn total_price(&self) -> f64 {
        f64::from(self.total_book) * self.average_price
    }
}

fn calculate_total_price(average_book_prices: Vec<AverageBookPrice>) -> f64 {
    average_book_prices
        .iter()
        .map(|average_book_price| { average_book_price.total_price() })
        .sum::<f64>()
}

fn main() {
    let average_book_prices: Vec<AverageBookPrice> = vec![
        AverageBookPrice {
            total_book: 2,
            average_price: 5.20,
        },
        AverageBookPrice {
            total_book: 3,
            average_price: 4.80,
        },
    ];
    let total_price: f64 = calculate_total_price(average_book_prices);
    println!("Total we need to pay: {:.2}", total_price);
}
