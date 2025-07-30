-- Enable UUID support
SET FOREIGN_KEY_CHECKS = 0;
-- Enable strict mode for data integrity
SET sql_mode = 'STRICT_TRANS_TABLES,NO_ENGINE_SUBSTITUTION';

-- Create complete USERS table with all fields
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    `uuid` BINARY(16) NOT NULL COMMENT 'For external reference',
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
    INDEX idx_users_is_admin (is_admin),
    UNIQUE KEY `idx_uuid` (`uuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


-- Orders (Active and Historical)
CREATE TABLE `orders` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `user_id` BIGINT UNSIGNED NOT NULL,
  `symbol_id` SMALLINT UNSIGNED NOT NULL,
  `client_order_id` VARCHAR(32) NULL COMMENT 'User-provided ID',
  `side` ENUM('BUY','SELL') NOT NULL,
  `type` ENUM('LIMIT','MARKET','STOP_LOSS','TAKE_PROFIT') NOT NULL,
  `price` DECIMAL(24,8) NULL COMMENT 'Null for MARKET orders',
  `quantity` DECIMAL(24,8) NOT NULL,
  `filled` DECIMAL(24,8) NOT NULL DEFAULT 0,
  `status` ENUM('NEW','PARTIALLY_FILLED','FILLED','CANCELED','REJECTED') NOT NULL DEFAULT 'NEW',
  `time_in_force` ENUM('GTC','IOC','FOK') NOT NULL DEFAULT 'GTC',
  `created_at` TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
  `updated_at` TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
  PRIMARY KEY (`id`),
  KEY `idx_user_symbol` (`user_id`, `symbol_id`, `status`),
  KEY `idx_price_time` (`symbol_id`, `side`, `price`, `created_at`) COMMENT 'For orderbook matching'
) ENGINE=InnoDB ROW_FORMAT=COMPRESSED;

-- Trades (Execution History)
CREATE TABLE `trades` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `symbol_id` SMALLINT UNSIGNED NOT NULL,
  `bid_order_id` BIGINT UNSIGNED NOT NULL,
  `ask_order_id` BIGINT UNSIGNED NOT NULL,
  `price` DECIMAL(24,8) NOT NULL,
  `quantity` DECIMAL(24,8) NOT NULL,
  `maker_fee` DECIMAL(24,8) NOT NULL,
  `taker_fee` DECIMAL(24,8) NOT NULL,
  `liquidity` ENUM('MAKER','TAKER') NOT NULL,
  `executed_at` TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
  PRIMARY KEY (`id`),
  KEY `idx_symbol_time` (`symbol_id`, `executed_at`),
  KEY `idx_bid_order` (`bid_order_id`),
  KEY `idx_ask_order` (`ask_order_id`)
) ENGINE=InnoDB ROW_FORMAT=COMPRESSED;


-- Order Book Snapshots 
CREATE TABLE `orderbook_snapshots` (
  `symbol_id` SMALLINT UNSIGNED NOT NULL,
  `sequence` BIGINT UNSIGNED NOT NULL COMMENT 'Monotonic increment',
  `bids` JSON NOT NULL COMMENT '[{price: x, quantity: y}, ...]',
  `asks` JSON NOT NULL,
  `created_at` TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
  PRIMARY KEY (`symbol_id`, `sequence`)
) ENGINE=InnoDB;

-- Account Balances
CREATE TABLE `balances` (
  `user_id` BIGINT UNSIGNED NOT NULL,
  `asset` VARCHAR(6) NOT NULL,
  `total` DECIMAL(24,8) NOT NULL DEFAULT 0,
  `available` DECIMAL(24,8) NOT NULL DEFAULT 0,
  `locked` DECIMAL(24,8) NOT NULL DEFAULT 0 COMMENT 'In open orders',
  `updated_at` TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
  PRIMARY KEY (`user_id`, `asset`),
  KEY `idx_asset` (`asset`)
) ENGINE=InnoDB;