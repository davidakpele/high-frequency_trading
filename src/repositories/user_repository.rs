use sqlx::{MySqlPool, Row};
use crate::models::users::User;
use crate::controllers::user_controller::UpdateUserRequest;
use anyhow::{Result, anyhow};
use chrono::NaiveDateTime;

pub struct UserRepository {
    pub db: MySqlPool,
}

impl UserRepository {
    pub async fn find_by_email(&self, email: &str) -> Result<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, 
                email, 
                username, 
                password, 
                is_active != 0 as is_active, 
                is_admin != 0 as is_admin, 
                is_verified != 0 as is_verified, 
                is_staff != 0 as is_staff,
                last_login,
                email_verified_at,
                created_at,
                updated_at
            FROM users 
            WHERE email = ?
            "#
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;

        match row {
            Some(row) => Ok(map_user(row)),
            None => Err(anyhow!("User not found")),
        }
    }

    pub async fn find_by_id(&self, user_id: i32) -> Result<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, 
                email, 
                username, 
                password, 
                is_active != 0 as is_active, 
                is_admin != 0 as is_admin, 
                is_verified != 0 as is_verified, 
                is_staff != 0 as is_staff,
                last_login,
                email_verified_at,
                created_at,
                updated_at
            FROM users 
            WHERE id = ?
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?;

        match row {
            Some(row) => Ok(map_user(row)),
            None => Err(anyhow!("User not found")),
        }
    }

    pub async fn update_user(&self, user_id: i32, payload: UpdateUserRequest) -> Result<User> {
        let current = self.find_by_id(user_id).await?;

        let new_username = payload.username.unwrap_or(current.username);
        let new_email = payload.email.unwrap_or(current.email);

        sqlx::query!(
            "UPDATE users SET username = ?, email = ? WHERE id = ?",
            new_username,
            new_email,
            user_id
        )
        .execute(&self.db)
        .await?;

        self.find_by_id(user_id).await
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        let rows = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.db)
            .await?
            .rows_affected();

        if rows == 0 {
            return Err(anyhow!("User not found or already deleted"));
        }

        Ok(())
    }
}

fn map_user(row: sqlx::mysql::MySqlRow) -> User {
    User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password: row.get("password"),
        is_active: row.get("is_active"),
        is_admin: row.get("is_admin"),
        is_verified: row.get("is_verified"),
        is_staff: row.get("is_staff"),
        last_login: row.get::<Option<NaiveDateTime>, _>("last_login"),
        email_verified_at: row.get::<Option<NaiveDateTime>, _>("email_verified_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
