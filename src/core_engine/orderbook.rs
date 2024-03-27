use super::order::BuyOrSell;
use super::order::Order;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct OrderBook {
    // HashMap : [Key : Price, Value : All the orders at that price]
    pub buy_orders: HashMap<Decimal, Vec<Order>>,
    pub sell_orders: HashMap<Decimal, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),
        }
    }

    pub fn add_order_to_orderbook(&mut self, order: Order) {
        // Check the order type whether it is a buy or sell order
        let order_price = order.price;

        match order.order_type {
            BuyOrSell::Buy => {
                // Check If the price exists in the buy_orders HashMap
                match self.buy_orders.get_mut(&order_price) {
                    Some(orders) => {
                        // If it exists, add the order to the existing price point
                        orders.push(order);
                    }
                    None => {
                        // If it does not exist, create a new price point and add the order
                        self.buy_orders.insert(order_price, vec![order]);
                    }
                }
            }
            BuyOrSell::Sell => {
                // Check If the price exists in the sell_orders HashMap
                match self.sell_orders.get_mut(&order_price) {
                    Some(orders) => {
                        // If it exists, add the order to the existing price point
                        orders.push(order);
                    }
                    None => {
                        // If it does not exist, create a new price point and add the order
                        self.sell_orders.insert(order_price, vec![order]);
                    }
                }
            }
        }
    }

    pub fn best_buy_price(&self) -> Option<Decimal> {
        // Get the maximum price from the buy_orders HashMap
        self.buy_orders.keys().max().cloned()
    }

    pub fn best_sell_price(&self) -> Option<Decimal> {
        // Get the minimum price from the sell_orders HashMap
        self.sell_orders.keys().min().cloned()
    }

    pub fn buy_volume(&self) -> Option<Decimal> {
        // Calculate the total volume of the buy orders
        let buy_volume: Decimal = self
            .buy_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(buy_volume)
    }

    pub fn sell_volume(&self) -> Option<Decimal> {
        // Calculate the total volume of the buy orders
        let sell_volume: Decimal = self
            .sell_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(sell_volume)
    }
}
