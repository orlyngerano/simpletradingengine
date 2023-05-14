use super::{order::Order, orderside::OrderSide, trade::Trade};

pub struct OrderBook {
    pub sell_orders: Vec<Order>,
    pub buy_orders: Vec<Order>,
}

impl OrderBook {
    pub fn process_order(&mut self, mut order: Order) -> Vec<Trade> {
        let maker_orders = if order.side == OrderSide::SELL {
            &mut self.sell_orders
        } else {
            &mut self.buy_orders
        };

        let mut trades: Vec<Trade> = Vec::new();

        // keep track of all maker orders that has been filled
        let mut empty_order_ids: Vec<usize> = Vec::new();

        for (maker_order_index, maker_order) in maker_orders.into_iter().enumerate() {
            if order.side == OrderSide::SELL && order.price > maker_order.price
                || order.side == OrderSide::BUY && order.price < maker_order.price
            {
                break;
            }

            let trade_quantity = match (order.quantity, maker_order.quantity) {
                // taker order filled
                (t, m) if t < m => {
                    order.quantity = 0;
                    maker_order.quantity -= t;
                    t
                }
                // maker order filled
                (t, m) if t > m => {
                    order.quantity -= m;
                    maker_order.quantity = 0;
                    empty_order_ids.push(maker_order_index);
                    m
                }
                // taker and order filled
                (t, _m) => {
                    order.quantity = 0;
                    maker_order.quantity = 0;
                    empty_order_ids.remove(maker_order_index);
                    t
                }
            };

            trades.push(Trade {
                taker_order_id: order.id,
                maker_order_id: maker_order.id,
                price: order.price,
                quantity: trade_quantity,
            });
        }

        if order.quantity > 0 {
            maker_orders.push(order);
            maker_orders.sort_by_key(|a| (a.price, a.date_created));
        }

        empty_order_ids.iter().for_each(|index| {
            maker_orders.remove(*index);
        });

        trades
    }
}
