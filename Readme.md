# 🏦 High-Frequency Trading (HFT) Engine
- This project is a High-Frequency Trading (HFT) engine designed to simulate and execute algorithmic trades at ultra-fast speeds. It leverages Rust’s performance and safety features to build a robust, concurrent trading system.

## 🚀 What is HFT?
- High-Frequency Trading (HFT) is a form of algorithmic trading that uses advanced hardware, software, and mathematical models to execute a large number of trades in fractions of a second.
## 🔑 Key Features
- ⚡ Speed: Executes trades in microseconds using asynchronous processing and efficient system design.
- 📊 Algorithms: All trading decisions are fully automated with no human intervention, using customizable mathematical models.
- 📈 High Volume: Capable of executing thousands to millions of trades daily.
- ⏱ Short Holding Time: Assets are typically held for milliseconds to a few seconds.
- 📡 Market Access: Designed with low-latency architecture, mimicking co-location strategies for faster access to market data.

## 🎯 Why HFT?
- To profit from arbitrage opportunities — small price differences across markets.
- To provide liquidity — by buying when others sell and vice versa.
- To react instantly to market events and news.

## VERSION 1 
- Authentication (SignIn / SignUp)
- Authorization (Protected routing and Public Routes with Jwt Bearer Token and User Claims Access or Admin and User.)
- Websocket configuration / Connection and User disconnection.
- Message Broadcasting
- Notifications and Error handling.
- Process per-2-per trading, 
    - Accept User Order request (Buy/Sell)
    - Buy order save in order with status "OPEN"
    - Sell Order save into order and also move the user assets to ESCROW pending user is book or match is found update this order status to PENDING and wait till Buyer confirm payment and sell to confimr payment before updating system release asset to buyer.
    - Implemented user wallet and balance for each wallet like Bitcoin, solana and so on. 
    - Implemented Crypto/ wallet Address, for platform to platform (market transfer)


## VERSION 2 IMPROVEMENT (Still on-going)
- Database Operations:
    - Async writes to database in background
    - Batch operations instead of per-trade updates
- Matching Algorithm:
    - Maintain an in-memory order book
    - Implemented price-time priority matching
    - Used efficient data structures (BTreeMap for price levels, VecDeque for orders)

- Result: 
    - Send trades to a background thread via a queue (like a pipe) immediately after matching.
    - Never wait for DB confirmation.
    - 24hr Data Clone?
        - Yes: At midnight, take a snapshot of all active orders and save it.
        - Logs: Keep a separate log of all trades for auditing.

## 📦 Implemented Features

- ✅ Order matching engine using `OrderBook` (bid/ask management)
- ✅ Limit and Market order support
- ✅ Matching logic with quantity and price priority
- ✅ Real-time order placement via WebSocket
- ✅ Broadcasting responses to connected clients
- ✅ Background matching loop (runs every 100ms)
- ✅ Persistence via MySQL (or other SQLx-compatible DB)
- ✅ Modular architecture: services, payloads, handlers, repos

---

## 🛠️ Tech Stack

- **Rust**
- **tokio** – async runtime
- **sqlx** – database access
- **dashmap** – concurrent order book
- **uuid** – unique client identification
- **tokio-tungstenite** – WebSocket handling
- **axum** – web framework (via Tower HTTP)
- **serde / serde_json** – JSON serialization
- **chrono** – timestamps


## 💡 Core Concepts
### 🔁 Matching Engine
- OrderBook holds bids and asks using DashMap<String, VecDeque<Order>>
    - Matching logic:
        - Match when highest bid ≥ lowest ask
        - Match quantity based on min(bid.qty, ask.qty)
        - Generate Trade object
        - or reduce matched orders

## 🧵 Matching Service
- Runs in a loop every 100ms
- Locks the shared OrderBook
- Calls .match_orders()
- Persists matched trades using TradeRepository

## 🌐 WebSocket Integration
- WebSocket server listens on port 9001
- On client connection:
    - Registers client
    - Listens for "create_order" events
- On "create_order":
    - Deserializes into CreateOrderPayload
    - Converts to internal Order model using TryFrom
    - Adds to order book
    - Responds with success or error


## 🛠️ Build & Run

1. Install dependencies
```
cargo build
```
2. Setup database
- Configure your .env with DB connection string
- Run DB migrations (if added)
3. Start the server
```
cargo run
```

## VERSION 3 of this project will be python analyzing data and train our model for trading, at least 55% accurate of trade execution.