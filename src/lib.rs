pub mod core_engine;

#[cfg(test)]
mod test {
    use self::core_engine::engine::{Company, IndianExchange, Market, MatchingEngine, Sector};

    use super::*;
    use core_engine::{
        order::{BuyOrSell, Order},
        orderbook::OrderBook,
    };
    use rust_decimal_macros::dec;

    #[test]
    fn test_add_order_to_orderbook() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(35), dec!(690), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(20), dec!(685), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(15), dec!(690), BuyOrSell::Buy);

        // Create some sell orders.
        let sell_order_1 = Order::new(dec!(10), dec!(700), BuyOrSell::Sell);
        let sell_order_2 = Order::new(dec!(25), dec!(705), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(30), dec!(700), BuyOrSell::Sell);

        // Add the orders to the order_book
        order_book.add_order_to_orderbook(buy_order_1);
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);
        assert_eq!(order_book.buy_orders.len(), 2);
        assert_eq!(order_book.sell_orders.len(), 2);
        assert_eq!(order_book.buy_orders.get(&dec!(690)).unwrap().len(), 2);
        assert_eq!(order_book.buy_orders.get(&dec!(685)).unwrap().len(), 1);
        assert_eq!(order_book.sell_orders.get(&dec!(700)).unwrap().len(), 2);
        assert_eq!(order_book.sell_orders.get(&dec!(705)).unwrap().len(), 1);
    }

    #[test]
    fn test_prices_and_volumes() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(35), dec!(690), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(20), dec!(685), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(15), dec!(690), BuyOrSell::Buy);

        // Create some sell orders.
        let sell_order_1 = Order::new(dec!(10), dec!(700), BuyOrSell::Sell);
        let sell_order_2 = Order::new(dec!(25), dec!(705), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(30), dec!(700), BuyOrSell::Sell);

        // Add the orders to the order_book
        order_book.add_order_to_orderbook(buy_order_1);
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);

        assert_eq!(order_book.best_buy_price().unwrap(), dec!(690));
        assert_eq!(order_book.best_sell_price().unwrap(), dec!(700));
        // Total Buying Order Quantity = 35+20+15
        assert_eq!(order_book.buy_volume().unwrap(), dec!(70));
        // Total Selling Order Quantity = 10+25+30
        assert_eq!(order_book.sell_volume().unwrap(), dec!(65));
    }

    #[test]
    fn test_company_listing() {
        let mut engine = MatchingEngine::new();
        let company = Company::new(
            "Nactore".to_string(),
            "NACT".to_string(),
            Sector::Technology,
            Market::IndianMarket(IndianExchange::BSE),
        );
        engine.list_new_company(company.clone());
        assert_eq!(engine.orderbooks.len(), 1);
        match engine.get_company_orderbook(&company) {
            Some(order_book) => {
                // Create some buy orders.
                let buy_order_1 = Order::new(dec!(35), dec!(690), BuyOrSell::Buy);
                let buy_order_2 = Order::new(dec!(20), dec!(685), BuyOrSell::Buy);
                let buy_order_3 = Order::new(dec!(15), dec!(690), BuyOrSell::Buy);

                // Create some sell orders.
                let sell_order_1 = Order::new(dec!(10), dec!(700), BuyOrSell::Sell);
                let sell_order_2 = Order::new(dec!(25), dec!(705), BuyOrSell::Sell);
                let sell_order_3 = Order::new(dec!(30), dec!(700), BuyOrSell::Sell);

                // Add the orders to the order_book
                order_book.add_order_to_orderbook(buy_order_1);
                order_book.add_order_to_orderbook(buy_order_2);
                order_book.add_order_to_orderbook(buy_order_3);
                order_book.add_order_to_orderbook(sell_order_1);
                order_book.add_order_to_orderbook(sell_order_2);
                order_book.add_order_to_orderbook(sell_order_3);
            }
            None => panic!("Company not found"),
        };
        assert_eq!(
            engine
                .get_company_orderbook(&company)
                .unwrap()
                .buy_volume()
                .unwrap(),
            dec!(70)
        );
        assert_eq!(
            engine
                .get_company_orderbook(&company)
                .unwrap()
                .sell_volume()
                .unwrap(),
            dec!(65)
        );
    }
}
