High-Frequency Trading (HFT) is a type of algorithmic trading that uses powerful computers and complex mathematical models to execute a large number of orders at extremely high speeds—often in fractions of a second.
Key Features of HFT:
Speed: Trades happen in microseconds (millionths of a second) using advanced hardware and direct market access.
Algorithms: Decisions to buy/sell are made
automatically using mathematical models-no human intervention.
Volume: Executes thousands or millions of trades a day to exploit very small price differences.

Holding Time: HFT firms hold assets for very short durations-sometimes just milliseconds or seconds.
Market Access: Uses co-location (putting servers near exchanges) for faster data access.
Why HFT Exists:
9,885
To profit from small price differences across markets (arbitrage).
74
To provide liquidity (buying when others sell and vice versa).
4,269
To react faster than other traders to news or
market movements.

Why HFT Exists:
To profit from small price differences across
markets (arbitrage).
To provide liquidity (buying when others sell and vice versa).
9,885
74
To react faster than other traders to news or
market movements.
Example Strategies:
4,269
Market Making: Placing buy and sell orders constantly, profiting from the bid-ask spread.

```
graph LR
    A[Incoming Orders] --> B[ConcurrentHashMap Matching]
    B --> C[Trade Output]
    C --> D[MPSC Channel]
    D --> E[DB Writer Thread]
    E --> F[Batched SQL INSERTs]
```
🎯 Ultimate Goal:
Build a high-speed, event-driven system that:

Receives buy/sell orders

Matches compatible orders

Executes trades instantly

Updates wallets/balances

Stores the data

Sends results back in real time

✅ Step-by-Step Breakdown
Here’s what we need to create first, and why:

1. order_book.rs
📂 Path: engine/order_book.rs

✅ Why?
This module maintains the in-memory list of active buy/sell orders for each trading pair. It allows:

Fast access to open orders

Sorted buy/sell queues

Efficient insert/delete/search

Example:

order_book.insert(order);    // Add new order  
order_book.match_order();    // Try matching


2. matcher.rs
📂 Path: engine/matcher.rs

✅ Why?
Contains the matching logic:

Compares new orders with existing ones in order_book

Finds matches by price/quantity

Returns a list of Trades to execute

This is your algorithmic engine.



3. trader.rs
📂 Path: engine/trader.rs

✅ Why?
Executes trades by:

Updating wallet balances

Locking/unlocking funds

Creating Trade records

This is your execution unit, simulating real exchange behavior.



4. order_service.rs
📂 Path: services/order_service.rs

✅ Why?
This layer handles the full order flow:

Validate order data

Insert into order_book

Call matcher

Call trader

Return results

Acts as the controller logic in business layer.


5. order_controller.rs
📂 Path: controllers/order_controller.rs

✅ Why?
This is the public entrypoint (WebSocket):

Accepts WS requests from Python clients

Sends to order_service

This is the API gateway for placing orders.


6. order_repository.rs
📂 Path: repositories/order_repository.rs

✅ Why?
Handles saving:

Orders (new or updated)

Trades (executed results)

Separates DB logic from core logic.


🛠️ TL;DR: What to Create Now

| File                    | Role                  | Why It’s Needed                    |
| ----------------------- | --------------------- | ---------------------------------- |
| ✅ `order_book.rs`       | In-memory order queue | Fast order access, sorted matching |
| ✅ `matcher.rs`          | Matching logic        | Find compatible buy/sell orders    |
| ✅ `trader.rs`           | Trade executor        | Executes trades, updates wallet    |
| ✅ `order_service.rs`    | Business logic        | Orchestrates order → match → trade |
| ✅ `order_controller.rs` | API layer             | Accepts external order requests    |
| ✅ `order_repository.rs` | Persistence           | Saves to DB                        |


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
