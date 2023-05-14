use trading_engine::orderbook::{order::Order, orderbook::OrderBook};

fn main() {
    let order_book = OrderBook {
        sell_orders: vec![],
        buy_orders: vec![],
    };

    // accept orders somewhere ex. listen stream-processing platform
    let order = Order {
        id: todo!(),
        price: todo!(),
        quantity: todo!(),
        side: todo!(),
        date_created: todo!(),
    };

    let trades = order_book.process_order(order);
    // send trades somewhere ex. stream-processing platform
}
