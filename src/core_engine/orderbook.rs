use super::order::BuyOrSell;
use super::order::Order;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::BTreeMap;

pub struct OrderBook {
    // HashMap : [Key : Price, Value : All the orders at that price]
    pub buy_orders: BTreeMap<Decimal, Vec<Order>>,
    pub sell_orders: BTreeMap<Decimal, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
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

    pub fn market_price(&self, order_type: BuyOrSell) -> Option<Decimal> {
        match order_type {
            BuyOrSell::Buy => self.best_sell_price(),
            BuyOrSell::Sell => self.best_buy_price(),
        }
    }

    pub fn top_n_best_buy_prices(&self) -> Option<Vec<Decimal>> {
        let prices = self
            .buy_orders
            .keys()
            .rev()
            .take(5) // At Max 5
            .cloned()
            .collect::<Vec<Decimal>>();
        if prices.is_empty() {
            None
        } else {
            Some(prices)
        }
    }

    pub fn top_n_best_sell_prices(&self) -> Option<Vec<Decimal>> {
        let prices = self
            .sell_orders
            .keys()
            .take(5) // At Max 5
            .cloned()
            .collect::<Vec<Decimal>>();
        if prices.is_empty() {
            None
        } else {
            Some(prices)
        }
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

    pub fn match_market_order(&mut self, incoming_order: &mut Order) {
        match incoming_order.order_type {
            BuyOrSell::Buy => {
                let possible_prices = self.top_n_best_sell_prices();
                match possible_prices {
                    Some(prices) => {
                        for price in prices {
                            if let Some(orders_at_this_price) = self.sell_orders.get_mut(&price) {
                                Self::execute_match(orders_at_this_price, incoming_order);
                            }
                            if incoming_order.quantity == dec!(0) {
                                break;
                            }
                        }
                        if incoming_order.quantity != dec!(0) {
                            // Incoming Order was not fully executed.
                            self.add_order_to_orderbook(incoming_order.clone());
                        }
                    }
                    None => self.add_order_to_orderbook(incoming_order.clone()),
                }
            }
            BuyOrSell::Sell => {
                let possible_prices = self.top_n_best_buy_prices();
                match possible_prices {
                    Some(prices) => {
                        for price in prices {
                            if let Some(orders_at_this_price) = self.buy_orders.get_mut(&price) {
                                Self::execute_match(orders_at_this_price, incoming_order);
                            }
                            if incoming_order.quantity == dec!(0) {
                                break;
                            }
                        }
                        if incoming_order.quantity != dec!(0) {
                            // Incoming Order was not fully executed.
                            self.add_order_to_orderbook(incoming_order.clone());
                        }
                    }
                    None => self.add_order_to_orderbook(incoming_order.clone()),
                }
            }
        }
    }

    pub fn match_limit_order(&mut self, incoming_order: &mut Order) {
        match incoming_order.order_type {
            BuyOrSell::Buy => {
                let possible_prices = self.top_n_best_sell_prices();
                match possible_prices {
                    Some(prices) => {
                        for price in prices {
                            if incoming_order.price >= price {
                                if let Some(orders_at_this_price) = self.sell_orders.get_mut(&price)
                                {
                                    Self::execute_match(orders_at_this_price, incoming_order);
                                }
                                if incoming_order.quantity == dec!(0) {
                                    break;
                                }
                            }
                        }
                        if incoming_order.quantity != dec!(0) {
                            // Incoming Order was not fully executed.
                            self.add_order_to_orderbook(incoming_order.clone());
                        }
                    }
                    None => self.add_order_to_orderbook(incoming_order.clone()),
                }
            }
            BuyOrSell::Sell => {
                let possible_prices = self.top_n_best_buy_prices();
                match possible_prices {
                    Some(prices) => {
                        for price in prices {
                            if incoming_order.price <= price {
                                if let Some(orders_at_this_price) = self.buy_orders.get_mut(&price)
                                {
                                    Self::execute_match(orders_at_this_price, incoming_order);
                                }
                                if incoming_order.quantity == dec!(0) {
                                    break;
                                }
                            }
                        }
                        if incoming_order.quantity != dec!(0) {
                            // Incoming Order was not fully executed.
                            self.add_order_to_orderbook(incoming_order.clone());
                        }
                    }
                    None => self.add_order_to_orderbook(incoming_order.clone()),
                }
            }
        }
    }

    fn execute_match(valid_orders: &mut Vec<Order>, incoming_order: &mut Order) {
        for order in valid_orders.iter_mut() {
            // Partially Matched
            if order.quantity < incoming_order.quantity {
                incoming_order.quantity -= order.quantity;
                order.quantity = dec!(0);
            }
            // Perfectly Matched
            else if order.quantity == incoming_order.quantity {
                order.quantity = dec!(0);
                incoming_order.quantity = dec!(0);
                break;
            }
            // Fully Matched
            else {
                order.quantity -= incoming_order.quantity;
                incoming_order.quantity = dec!(0);
                break;
            }
        }
    }
}
