use std::collections::HashMap;

use super::orderbook::OrderBook;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Market {
    IndianMarket(IndianExchange),
    USMarket(USExchange),
    CryptoMarket(CryptoExchange),
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum IndianExchange {
    NSE,
    BSE,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum USExchange {
    NASDAQ,
    NYSE,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum CryptoExchange {
    WazirX,
    CoinDCX,
    Binance,
    Coinbase,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Sector {
    Technology,
    Finance,
    Banking,
    Healthcare,
    Energy,
    ConsumerDiscretionary,
    ConsumerStaples,
    Industrials,
    Materials,
    RealEstate,
    CommunicationServices,
    Utilities,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Company {
    name: String,
    symbol: String,
    sector: Sector,
    market: Market,
}

impl Company {
    pub fn new(name: String, symbol: String, sector: Sector, market: Market) -> Company {
        Company {
            name,
            symbol,
            sector,
            market,
        }
    }
}

pub struct MatchingEngine {
    pub orderbooks: HashMap<Company, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn list_new_company(&mut self, company: Company) {
        let orderbook = OrderBook::new();
        self.orderbooks.insert(company, orderbook);
    }

    pub fn get_company_orderbook(&mut self, company: &Company) -> Option<&mut OrderBook> {
        self.orderbooks.get_mut(company)
    }
}
