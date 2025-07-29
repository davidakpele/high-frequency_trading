use sqlx::{MySqlPool, Row};
use crate::{models::users::User, responses::responses::SafeUser};
use anyhow::{Result, anyhow};
use chrono::NaiveDateTime;

pub struct AuthenticationRepository {
    pub db: MySqlPool,
}

impl AuthenticationRepository {
    pub async fn create_user(&self, email: &str, username: &str, password: &str) -> Result<SafeUser> {
        let mut tx = self.db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO users (email, username, password)
            VALUES (?, ?, ?)
            "#,
            email,
            username,
            password,
        )
        .execute(&mut *tx)
        .await?;

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
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut *tx)
        .await?;

        let user = map_user(row);
        tx.commit().await?;
        Ok(SafeUser::from(user))
    }

    pub async fn login_user(&self, email: &str) -> Result<User> {
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
            None => Err(anyhow!("Invalid credentials")),
        }
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
