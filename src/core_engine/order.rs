use rust_decimal::Decimal;

#[derive(Clone)]
pub enum BuyOrSell {
    Buy,
    Sell,
}

#[derive(Clone)]
pub struct Order {
    pub quantity: Decimal,
    pub price: Decimal,
    pub order_type: BuyOrSell,
}

impl Order {
    pub fn new(quantity: Decimal, price: Decimal, order_type: BuyOrSell) -> Order {
        Order {
            quantity,
            price,
            order_type,
        }
    }
}
