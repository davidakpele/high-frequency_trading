use crate::models::bank_list::BankList;
use sqlx::{MySqlPool};
use anyhow::Result;

pub struct BankListRepository {
    pub db: MySqlPool,
}

impl BankListRepository {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    #[allow(dead_code)]
    pub async fn find_by_account_number(&self, account_number: &str) -> Result<Option<BankList>> {
        let record = sqlx::query_as::<_, BankList>(
            "SELECT * FROM bank_list WHERE account_number = ?"
        )
        .bind(account_number)
        .fetch_optional(&self.db)
        .await?;

        Ok(record)
    }

    #[allow(dead_code)]
    pub async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<BankList>> {
        let records = sqlx::query_as::<_, BankList>(
            "SELECT * FROM bank_list WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(records)
    }

    #[allow(dead_code)]
    pub async fn find_by_account_number_and_bank_code(
        &self,
        account_number: &str,
        bank_code: &str,
    ) -> Result<Option<BankList>> {
        let record = sqlx::query_as::<_, BankList>(
            "SELECT * FROM bank_list WHERE account_number = ? AND bank_code = ?"
        )
        .bind(account_number)
        .bind(bank_code)
        .fetch_optional(&self.db)
        .await?;

        Ok(record)
    }

    #[allow(dead_code)]
    pub async fn find_by_id_and_user_id(
        &self,
        bank_id: i64,
        user_id: i64,
    ) -> Result<Option<BankList>> {
        let record = sqlx::query_as::<_, BankList>(
            "SELECT * FROM bank_list WHERE id = ? AND user_id = ?"
        )
        .bind(bank_id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?;

        Ok(record)
    }

    #[allow(dead_code)]
    pub async fn delete_user_bank_details_by_ids(&self, ids: &[i64]) -> Result<u64> {
        let query = format!(
            "DELETE FROM bank_list WHERE user_id IN ({})",
            ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query = sqlx::query(&query);
        for id in ids {
            query = query.bind(id);
        }

        let result = query.execute(&self.db).await?;
        Ok(result.rows_affected())
    }
}
