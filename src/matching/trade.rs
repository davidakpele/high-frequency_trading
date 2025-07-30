use crate::models::order::Order;


#[derive(Debug, Clone)]
pub struct Trade {
    pub bid_id: u64,
    pub ask_id: u64,
    pub symbol: String,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
}

impl Trade {
    #[inline]
    pub fn new(bid: Order, ask: Order, quantity: u64) -> Self {
        let price = ask.price.unwrap_or(bid.price.unwrap());
        Self {
            bid_id: bid.id,
            ask_id: ask.id,
            symbol: bid.symbol,
            price,
            quantity,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }
}