use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub id: i64,
    pub email: String,
    pub username: String,
}
