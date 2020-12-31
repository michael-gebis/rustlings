// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

fn calculate_apple_price(n:u32) -> u32 {
    const PRICE:u32 = 2;
    const BULK_QUANT:u32 = 40;
    const BULK_PRICE:u32 = 1;

    if n > BULK_QUANT {
        return n*BULK_PRICE;
    }
    n*PRICE
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
