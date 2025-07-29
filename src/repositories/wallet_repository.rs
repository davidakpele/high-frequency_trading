use crate::models::wallet::Wallet;
use sqlx::{Row, MySqlPool};
use anyhow::Result;
use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct WalletRepository {
    pub pool: MySqlPool,
}

impl WalletRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create_wallet(
        &self,
        user_id: String,
        crypto_id: &str,
        balance: BigDecimal,
        wallet_address: Option<String>,
        version: i32,
    ) -> Result<Wallet> {
        let mut tx = self.pool.begin().await?;

        // Generate UUID for wallet ID
        let wallet_id = Uuid::new_v4().to_string();

        // Step 1: Insert into wallet table with manually generated ID
        sqlx::query(
            r#"
            INSERT INTO wallet (id, user_id, crypto_id, balance, wallet_address, version)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&wallet_id)
        .bind(&user_id)
        .bind(crypto_id)
        .bind(balance)
        .bind(wallet_address.clone())
        .bind(version)
        .execute(&mut *tx)
        .await?;

        // Step 2: Fetch the inserted wallet by wallet_id
        let row = sqlx::query(
            r#"
            SELECT id, user_id, crypto_id, balance, wallet_address, version
            FROM wallet
            WHERE id = ?
            "#
        )
        .bind(&wallet_id)
        .fetch_one(&mut *tx)
        .await?;

        // Step 3: Manually map the row
        let wallet = Wallet {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            crypto_id: row.try_get("crypto_id")?,
            balance: row.try_get("balance")?,
            wallet_address: row.try_get("wallet_address")?,
            version: row.try_get("version")?,
        };

        tx.commit().await?;
        Ok(wallet)
    }

    #[allow(dead_code)]
    pub async fn update_wallet_balance(
        &self,
        user_id: i64,
        crypto_id: &str,
        new_balance: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE wallet
            SET balance = ?
            WHERE user_id = ? AND crypto_id = ?
            "#
        )
        .bind(new_balance)
        .bind(user_id)
        .bind(crypto_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_wallet(
        &self,
        user_id: i64,
        crypto_id: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM wallet
            WHERE user_id = ? AND crypto_id = ?
            "#
        )
        .bind(user_id)
        .bind(crypto_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn find_all_by_user(&self, user_id: i64) -> Result<Vec<Wallet>> {
        let wallets = sqlx::query_as::<_, Wallet>(
            r#"
            SELECT id, user_id, crypto_id, balance, wallet_address, version
            FROM wallet
            WHERE user_id = ?
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(wallets)
    }

    #[allow(dead_code)]
    pub async fn find_by_user_id_and_asset(
        &self,
        user_id: i64,
        crypto_id: &str,
    ) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as::<_, Wallet>(
            r#"
            SELECT id, user_id, crypto_id, balance, wallet_address, version
            FROM wallet
            WHERE user_id = ? AND crypto_id = ?
            "#,
        )
        .bind(user_id)
        .bind(crypto_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(wallet)
    }


    
}
