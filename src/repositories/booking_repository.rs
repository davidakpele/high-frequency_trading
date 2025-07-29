use sqlx::{MySqlPool, mysql::MySqlArguments, Arguments};
use anyhow::Result;
use uuid::Uuid;


pub struct BookingRepository {
    pub db: MySqlPool,
}


impl BookingRepository{
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    #[allow(dead_code)]
    pub async fn save_coin_booking(
        &self,
        order_id: String,
        buyer_id: String,
        seller_id: String,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO booking (id, order_id, buyer_id, seller_id) VALUES (?, ?, ?, ?)",
            id,
            order_id,
            buyer_id,
            seller_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    async fn delete_by_user_ids(&self, user_ids: &[String]) -> Result<()> {
        if user_ids.is_empty() {
            return Ok(());
        }

        let mut args = MySqlArguments::default();
        let mut placeholders = vec![];

        for id in user_ids {
            placeholders.push("?");
            let _ = args.add(id.clone());
        }

        let placeholder_str = placeholders.join(", ");
        let query = format!(
            "DELETE FROM booking WHERE buyer_id IN ({0}) OR seller_id IN ({0})",
            placeholder_str
        );

        sqlx::query_with(&query, args)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
