use rand::rngs::OsRng; 
use ed25519_dalek::SigningKey;
use bs58;
use hex;
use crate::responses::login_responses::LoginResponse;
use crate::responses::responses::SafeUser;
use crate::{
    repositories::auth_repository::AuthenticationRepository,
    utils::jwt::generate_token,
};
use crate::controllers::auth_controller::{RegisterRequest, LoginRequest};
use anyhow::{Result, anyhow};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_user(
    db: &MySqlPool,
    req: RegisterRequest,
) -> Result<SafeUser> {
    let hashed_password = hash_password(&req.password)?;
    let repo = AuthenticationRepository { db: db.clone() };
    let user = repo.create_user(&req.email, &req.username, &hashed_password).await?;
    Ok(user)
}

#[allow(dead_code)]
fn generate_wallet_address(asset: &str) -> String {
    match asset.to_lowercase().as_str() {
        "ethereum" => generate_eth_wallet_address(),
        "bitcoin" => generate_btc_wallet_address(),
        "solana" => generate_solana_wallet_address(),
        "litecoin" => generate_btc_wallet_address(),         // BTC-style
        "bitcoin_cash" => generate_btc_wallet_address(),     // BTC-style
        _ => generate_eth_wallet_address(),
    }
}

#[allow(dead_code)]
fn generate_eth_wallet_address() -> String {
    use rand::RngCore;
    use sha2::{Sha256, Digest};

    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(&random_bytes);
    let result = hasher.finalize();

    let eth_address = &result[12..]; // Last 20 bytes (Ethereum)
    format!("0x{}", hex::encode(eth_address))
}

#[allow(dead_code)]
fn generate_btc_wallet_address() -> String {
    use rand::RngCore;
    use sha2::{Sha256, Digest};
    use bs58;

    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 20]; // Public key hash
    rng.fill_bytes(&mut random_bytes);

    let mut data = vec![0x00]; // Version byte
    data.extend_from_slice(&random_bytes);

    let checksum = {
        let first = Sha256::digest(&data);
        let second = Sha256::digest(&first);
        second[..4].to_vec()
    };

    data.extend_from_slice(&checksum);
    bs58::encode(data).into_string()
}

#[allow(dead_code)]
pub fn generate_solana_wallet_address() -> String {
    let signing_key = SigningKey::generate(&mut OsRng);
    let public_key = signing_key.verifying_key();
    let public_bytes = public_key.to_bytes(); // 32 bytes
    bs58::encode(public_bytes).into_string()
}

pub async fn login_user(
    db: &MySqlPool,
    req: LoginRequest,
) -> Result<LoginResponse> {
    let repo = AuthenticationRepository { db: db.clone() };
    let user = repo.login_user(&req.email).await?;

    let is_valid = verify_password(&req.password, &user.password)?;
    if !is_valid {
        return Err(anyhow!("Invalid credentials"));
    }

    let is_admin = user.is_admin;

    let token = generate_token(
        user.id,
        user.email.clone(),
        is_admin,
    )?;

    Ok(LoginResponse {
        token,
        id: user.id,
        email: user.email.clone(),
        username: user.username.clone(),
    })
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e.to_string()))? 
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow!(e.to_string()))?;

    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}