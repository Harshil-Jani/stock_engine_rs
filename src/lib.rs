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
        }
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

    #[test]
    fn test_top_n_buy_and_sell_prices() {
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

        assert_eq!(
            order_book.top_n_best_buy_prices(),
            Some(vec![dec!(690), dec!(685)]) // Good for Sellers.
        );
        assert_eq!(
            order_book.top_n_best_sell_prices(),
            Some(vec![dec!(700), dec!(705)]) // Good for Buyers.
        );
    }

    #[test]
    fn test_match_full_order() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(25), dec!(1004), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(40), dec!(1004), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(125), dec!(1003), BuyOrSell::Buy);
        let buy_order_4 = Order::new(dec!(100), dec!(1002), BuyOrSell::Buy);
        let buy_order_5 = Order::new(dec!(150), dec!(1001), BuyOrSell::Buy);

        // Create some sell orders.
        let sell_order_1 = Order::new(dec!(10), dec!(1005), BuyOrSell::Sell);
        let sell_order_2 = Order::new(dec!(50), dec!(1006), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(25), dec!(1007), BuyOrSell::Sell);
        let sell_order_4 = Order::new(dec!(150), dec!(1008), BuyOrSell::Sell);
        let sell_order_5 = Order::new(dec!(120), dec!(1009), BuyOrSell::Sell);

        // Add the orders to the order_book
        order_book.add_order_to_orderbook(buy_order_1);
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(buy_order_4);
        order_book.add_order_to_orderbook(buy_order_5);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);
        order_book.add_order_to_orderbook(sell_order_4);
        order_book.add_order_to_orderbook(sell_order_5);

        assert_eq!(order_book.buy_volume(), Some(dec!(440)));
        assert_eq!(order_book.sell_volume(), Some(dec!(355)));

        // Example 1 : Buy 10 units at limit price of 1005
        let mut incoming_order_1 = Order::new(dec!(10), dec!(1005), BuyOrSell::Buy);
        order_book.match_limit_order(&mut incoming_order_1);
        assert_eq!(incoming_order_1.quantity, dec!(0));
        assert_eq!(order_book.buy_volume(), Some(dec!(440)));
        assert_eq!(order_book.sell_volume(), Some(dec!(345)));

        // Example 2 : Sell 100 units at market price
        let sell_market_price = order_book.market_price(BuyOrSell::Sell).unwrap();
        assert_eq!(sell_market_price, dec!(1004));
        let mut incoming_order_2 = Order::new(dec!(100), sell_market_price, BuyOrSell::Sell);
        order_book.match_market_order(&mut incoming_order_2);
        assert_eq!(incoming_order_2.quantity, dec!(0));
        assert_eq!(order_book.buy_volume(), Some(dec!(340)));
        assert_eq!(order_book.sell_volume(), Some(dec!(345)));

        // Example 3 : Buy 40 units at limit price of 1008
        let mut incoming_order_3 = Order::new(dec!(40), dec!(1008), BuyOrSell::Buy);
        order_book.match_limit_order(&mut incoming_order_3);
        assert_eq!(incoming_order_3.quantity, dec!(0));
        assert_eq!(order_book.buy_volume(), Some(dec!(340)));
        assert_eq!(order_book.sell_volume(), Some(dec!(305)));

        // Example 4 : Sell 20 units at limit price of 1004
        let mut incoming_order_4 = Order::new(dec!(20), dec!(1004), BuyOrSell::Sell);
        order_book.match_limit_order(&mut incoming_order_4);
        assert_eq!(incoming_order_4.quantity, dec!(20));
        assert_eq!(order_book.buy_volume(), Some(dec!(340)));
        assert_eq!(order_book.sell_volume(), Some(dec!(325)));
    }

    #[test]
    fn test_match_partial_order() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(10), dec!(1004), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(25), dec!(1003), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(50), dec!(1002), BuyOrSell::Buy);
        let buy_order_4 = Order::new(dec!(40), dec!(1001), BuyOrSell::Buy);
        let buy_order_5 = Order::new(dec!(120), dec!(1000), BuyOrSell::Buy);
        // Create some sell orders.
        let sell_order_1 = Order::new(dec!(20), dec!(1005), BuyOrSell::Sell);
        let sell_order_2 = Order::new(dec!(50), dec!(1006), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(30), dec!(1007), BuyOrSell::Sell);
        let sell_order_4 = Order::new(dec!(100), dec!(1008), BuyOrSell::Sell);
        let sell_order_5 = Order::new(dec!(210), dec!(1009), BuyOrSell::Sell);
        // Add the orders to the order_book
        order_book.add_order_to_orderbook(buy_order_1);
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(buy_order_4);
        order_book.add_order_to_orderbook(buy_order_5);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);
        order_book.add_order_to_orderbook(sell_order_4);
        order_book.add_order_to_orderbook(sell_order_5);

        assert_eq!(order_book.buy_volume(), Some(dec!(245)));
        assert_eq!(order_book.sell_volume(), Some(dec!(410)));

        // Example 1 : Buy 25 units at 1005
        let mut incoming_order_1 = Order::new(dec!(25), dec!(1005), BuyOrSell::Buy);
        order_book.match_limit_order(&mut incoming_order_1);
        assert_eq!(incoming_order_1.quantity, dec!(5));
        assert_eq!(order_book.buy_volume(), Some(dec!(250)));
        assert_eq!(order_book.sell_volume(), Some(dec!(390)));

        // Example 2 : Sell 1002 units at 1002
        let mut incoming_order_2 = Order::new(dec!(100), dec!(1002), BuyOrSell::Sell);
        order_book.match_limit_order(&mut incoming_order_2);
        assert_eq!(incoming_order_2.quantity, dec!(10));
        assert_eq!(order_book.buy_volume(), Some(dec!(160)));
        assert_eq!(order_book.sell_volume(), Some(dec!(400)));

        // Example 3 : Buy 30 units at Market Price
        let buy_market_price = order_book.market_price(BuyOrSell::Buy).unwrap();
        assert_eq!(buy_market_price, dec!(1002));
        let mut incoming_order_3 = Order::new(dec!(30), buy_market_price, BuyOrSell::Buy);
        order_book.match_market_order(&mut incoming_order_3);
        assert_eq!(incoming_order_3.quantity, dec!(0));
        assert_eq!(order_book.buy_volume(), Some(dec!(160)));
        assert_eq!(order_book.sell_volume(), Some(dec!(370)));
    }
    #[test]
    fn test_match_untradable_orders() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(15), dec!(1005), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(40), dec!(1002), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(90), dec!(1001), BuyOrSell::Buy);
        let buy_order_4 = Order::new(dec!(80), dec!(1000), BuyOrSell::Buy);
        // Create some sell orders.
        let sell_order_1 = Order::new(dec!(75), dec!(1008), BuyOrSell::Sell);
        let sell_order_2 = Order::new(dec!(60), dec!(1009), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(30), dec!(1010), BuyOrSell::Sell);
        let sell_order_4 = Order::new(dec!(210), dec!(1013), BuyOrSell::Sell);
        // Add the orders to the order_book
        order_book.add_order_to_orderbook(buy_order_1);
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(buy_order_4);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);
        order_book.add_order_to_orderbook(sell_order_4);

        assert_eq!(order_book.buy_volume(), Some(dec!(225)));
        assert_eq!(order_book.sell_volume(), Some(dec!(375)));

        // Example 1 : Buy 50 units at limit price of 1007
        let mut incoming_order_1 = Order::new(dec!(50), dec!(1007), BuyOrSell::Buy);
        order_book.match_limit_order(&mut incoming_order_1);
        assert_eq!(incoming_order_1.quantity, dec!(50));
        assert_eq!(order_book.buy_volume(), Some(dec!(275)));
        assert_eq!(order_book.sell_volume(), Some(dec!(375)));
    }
}
