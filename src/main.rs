mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

use crate::matching_engine::engine::{MachingEngine, TradingPair};
fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_bob);
    orderbook.add_order(4.4, buy_order_from_alice);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order);
    println!("{:?}", orderbook);

    let mut engine = MachingEngine::new();
    let trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(trading_pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine
        .place_limit_order(trading_pair, 10.000, buy_order)
        .unwrap()
}
