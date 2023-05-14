use super::orderside::OrderSide;

pub struct Order {
    pub id: u32,           // order id
    pub price: u32,        // asset price
    pub quantity: u32,     // number of assets
    pub side: OrderSide,   // sell or buy
    pub date_created: u32, // unix timestamp
}
