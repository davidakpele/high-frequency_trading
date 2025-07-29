-- Enable UUID support
SET FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS market_data;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS wallets;
DROP TABLE IF EXISTS bank_list;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS escrow;
DROP TABLE IF EXISTS booking;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS order_book_snapshots;
DROP TABLE IF EXISTS trade_execution_log;



-- Create complete USERS table with all fields
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_staff BOOLEAN NOT NULL DEFAULT FALSE,
    last_login TIMESTAMP NULL,
    email_verified_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_users_email (email),
    INDEX idx_users_is_active (is_active),
    INDEX idx_users_is_admin (is_admin)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


-- Coin Wallet Table
CREATE TABLE IF NOT EXISTS wallet (
    id CHAR(36) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    crypto_id VARCHAR(50) NOT NULL,
    balance DECIMAL(32,2) NOT NULL,
    wallet_address VARCHAR(255) NOT NULL,
    version INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Bank List Table
CREATE TABLE IF NOT EXISTS bank_list (
    id CHAR(36) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    account_number VARCHAR(50) NOT NULL,
    bank_code VARCHAR(50) NOT NULL,
    bank_name VARCHAR(100),
    account_name VARCHAR(100),
    branch_code VARCHAR(50),
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY (account_number, bank_code),
    INDEX idx_user_id (user_id),
    INDEX idx_account_number (account_number)
) ENGINE=InnoDB;

-- Orders Table
CREATE TABLE IF NOT EXISTS orders (
    id CHAR(36) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    is_maker BOOLEAN DEFAULT FALSE,
    trading_pair VARCHAR(20) NOT NULL, -- e.g., "BTC/USD"
    order_type ENUM('BUY', 'SELL') NOT NULL,
    price DECIMAL(32,2) NOT NULL,
    amount DECIMAL(32,2) NOT NULL,
    filled_amount DECIMAL(32,2) DEFAULT 0,
    status ENUM('OPEN', 'PARTIALLY_FILLED', 'FILLED', 'PENDING', 'CANCELED') NOT NULL DEFAULT 'OPEN',
    bank_id CHAR(36),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Escrow Table
CREATE TABLE IF NOT EXISTS escrow (
    id CHAR(36) PRIMARY KEY,
    order_id CHAR(36) NOT NULL,
    amount DECIMAL(32,2) NOT NULL,
    status ENUM('OPEN', 'PARTIALLY_FILLED', 'FILLED','PENDING', 'CANCELED') NOT NULL DEFAULT 'OPEN',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_order_id (order_id),
    INDEX idx_status (status),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
) ENGINE=InnoDB;

-- Coin Booking Table
CREATE TABLE IF NOT EXISTS booking (
    id CHAR(36) PRIMARY KEY,
    order_id CHAR(36) NOT NULL,
    buyer_id CHAR(36) NOT NULL,
    seller_id CHAR(36) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_order_id (order_id),
    INDEX idx_buyer_id (buyer_id),
    INDEX idx_seller_id (seller_id)
) ENGINE=InnoDB;

-- Transaction History Table
CREATE TABLE IF NOT EXISTS transactions (
    id CHAR(36) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    transaction_type ENUM('BUY', 'SELL') NOT NULL,
    quantity DECIMAL(32,2) NOT NULL,
    price DECIMAL(32,2) NOT NULL,
    trading_pair VARCHAR(20) NOT NULL,
    fee DECIMAL(32,2) NOT NULL DEFAULT 0,
    order_id CHAR(36),
    related_order_id CHAR(36),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_user_id (user_id),
    INDEX idx_trading_pair (trading_pair),
    INDEX idx_created_at (created_at),
    INDEX idx_order_id (order_id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE SET NULL,
    FOREIGN KEY (related_order_id) REFERENCES orders(id) ON DELETE SET NULL
) ENGINE=InnoDB;

-- Market Data Table (for real-time price tracking)
CREATE TABLE IF NOT EXISTS market_data (
    id CHAR(36) PRIMARY KEY,
    trading_pair VARCHAR(20) NOT NULL,
    bid_price DECIMAL(32,2) NOT NULL,
    ask_price DECIMAL(32,2) NOT NULL,
    last_price DECIMAL(32,2) NOT NULL,
    volume DECIMAL(32,2) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_trading_pair (trading_pair),
    INDEX idx_timestamp (timestamp)
) ENGINE=InnoDB;

-- Order Book Snapshots (for historical analysis)
CREATE TABLE IF NOT EXISTS order_book_snapshots (
    id CHAR(36) PRIMARY KEY,
    trading_pair VARCHAR(20) NOT NULL,
    snapshot_data JSON NOT NULL, -- Stores complete order book state
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_trading_pair (trading_pair),
    INDEX idx_timestamp (timestamp)
) ENGINE=InnoDB;

-- Trade Execution Log (for audit purposes)
CREATE TABLE IF NOT EXISTS trade_execution_log (
    id CHAR(36) PRIMARY KEY,
    buy_order_id CHAR(36),
    sell_order_id CHAR(36),
    executed_price DECIMAL(32,2) NOT NULL,
    executed_quantity DECIMAL(32,2) NOT NULL,
    trading_pair VARCHAR(20) NOT NULL,
    execution_time TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6), -- Microsecond precision
    INDEX idx_buy_order_id (buy_order_id),
    INDEX idx_sell_order_id (sell_order_id),
    INDEX idx_execution_time (execution_time),
    FOREIGN KEY (buy_order_id) REFERENCES orders(id) ON DELETE SET NULL,
    FOREIGN KEY (sell_order_id) REFERENCES orders(id) ON DELETE SET NULL
) ENGINE=InnoDB;

-- User Preferences (for trading settings)
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id CHAR(36) PRIMARY KEY,
    default_trading_pair VARCHAR(20),
    fee_tier VARCHAR(50),
    notification_preferences JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;


CREATE TABLE IF NOT EXISTS coin (
    id VARCHAR(255) PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    image TEXT,
    
    current_price DECIMAL(32,2) NOT NULL,
    market_cap NUMERIC(50, 0),
    market_cap_rank INT,
    fully_diluted_valuation NUMERIC(50, 0),
    total_volume NUMERIC(50, 0),
    high_24h NUMERIC(50, 0),
    low_24h NUMERIC(50, 0),
    
    price_change_24h DECIMAL(32,2),
    price_change_percentage_24h DECIMAL(32,2),
    
    market_cap_change_24h NUMERIC(50, 0),
    market_cap_change_percentage_24h DECIMAL(32,2),
    
    circulating_supply NUMERIC(50, 0),
    total_supply NUMERIC(50, 0),
    max_supply NUMERIC(50, 0),
    
    ath DECIMAL(32,2),
    ath_change_percentage DECIMAL(32,2),
    ath_date DATETIME,
    
    atl DECIMAL(32,2),
    atl_change_percentage DECIMAL(32,2),
    atl_date DATETIME,
    
    roi_times DECIMAL(32,2),
    roi_currency VARCHAR(10),
    roi_percentage DECIMAL(32,2),
    
    last_updated DATETIME,
    
    INDEX idx_symbol (symbol),
    INDEX idx_market_cap_rank (market_cap_rank),
    INDEX idx_last_updated (last_updated)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- Re-enable foreign key checks
SET FOREIGN_KEY_CHECKS = 1;