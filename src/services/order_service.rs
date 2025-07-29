use bigdecimal::BigDecimal;
use anyhow::{Result, anyhow};
use sqlx::MySqlPool;
use crate::{
    dto::minimal_order::MinimalOrder, enums::{order_status::{OrderStatus}, order_type::OrderType}, repositories::{
        bank_list_repository::BankListRepository,
        escrow_repository::EscrowRepository,
        order_repository::OrderRepository, wallet_repository::WalletRepository,
    }, services::order_matching_service::OrderMatchingService
};

#[allow(dead_code)]
pub struct OrderService {
    pool: MySqlPool,
    matching_service: OrderMatchingService,
    bank_repo: BankListRepository,
    order_repo: OrderRepository,
    escrow_repo: EscrowRepository,
    wallet_repo: WalletRepository,
}

impl OrderService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            matching_service: OrderMatchingService::new(pool.clone()),
            bank_repo: BankListRepository::new(pool.clone()),
            order_repo: OrderRepository::new(pool.clone()),
            escrow_repo: EscrowRepository::new(pool.clone()),
            wallet_repo : WalletRepository::new(pool.clone()),
            pool,
        }
    }

    pub async fn create_order(
        &self,
        user_id: i64,
        trading_pair: String,
        order_type: OrderType,
        price: BigDecimal,
        amount: BigDecimal,
        bank_id: Option<i64>,
    ) -> Result<()> {
        let status = OrderStatus::OPEN;
        let is_maker = false;
        let filled_amount = BigDecimal::from(0);

        // Clone order_type before it's moved
        let order_type_clone = order_type.clone();

        if order_type_clone == OrderType::SELL {
            self.validate_wallet_balance(user_id, &trading_pair, &amount).await?;
        }

        let order = self
            .save_order(
                user_id,
                is_maker,
                &trading_pair,
                order_type,
                &price,
                &amount,
                &filled_amount,
                status,
                bank_id,
            )
            .await?;

        // Use the clone here
        if order_type_clone == OrderType::SELL {
            self.create_escrow(&order, OrderStatus::OPEN).await?;
        }

        self.matching_service.match_orders(order).await?;

        Ok(())
    }


    // pub async fn create_order(
    //     &self,
    //     user_id: i64,
    //     trading_pair: String,
    //     order_type: OrderType,
    //     price: BigDecimal,
    //     amount: BigDecimal,
    //     bank_id: Option<i64>,
    // ) -> Result<()> {
    //     // Validate bank details if provided
    //     // if let Some(bank_id) = bank_id {
    //     //     self.validate_bank_details(user_id, bank_id).await?;
    //     // }

    //     // Set initial status and maker role
    //     let status = OrderStatus::OPEN;
    //     let is_maker = false;
    //     let filled_amount = BigDecimal::from(0);
        
    //     // For SELL orders, validate wallet balance
    //     if order_type == OrderType::SELL {
    //         self.validate_wallet_balance(user_id, &trading_pair, &amount).await?;
    //     }

    //     // Create and save the order
    //     let order = self.save_order(
    //             user_id,
    //             is_maker,
    //             &trading_pair,
    //             order_type,
    //             &price,
    //             &amount,
    //             &filled_amount,
    //             status,
    //             bank_id,
    //         )
    //         .await?;

    //     if order_type == OrderType::SELL {
    //         self.create_escrow(&order).await?;
    //     }

    //     // self.matching_service.match_orders(order.clone()).await?;

    //     Ok(())
    // }

    #[allow(dead_code)]
    async fn validate_bank_details(&self, user_id: i64, bank_id: i64) -> Result<()> {
        let bank_opt = self.bank_repo
            .find_by_id_and_user_id(bank_id, user_id)
            .await?;

        if bank_opt.is_none() {
            return Err(anyhow!("Bank record not found for user"));
        }

        Ok(())
    }

    async fn validate_wallet_balance(
        &self,
        user_id: i64,
        trading_pair: &str,
        amount: &BigDecimal,
    ) -> Result<()> {
        let base_asset = trading_pair;

        // Fetch wallet for the user and asset
        let wallet_opt = self
            .wallet_repo
            .find_by_user_id_and_asset(user_id, base_asset)
            .await?;

        let wallet = match wallet_opt {
            Some(w) => w,
            None => return Err(anyhow!("Wallet for asset '{}' not found", base_asset)),
        };

        // Compare balance
        if wallet.balance < *amount {
            return Err(anyhow!(
                "Insufficient balance. Available: {}, Required: {}",
                wallet.balance,
                amount
            ));
        }

        Ok(())
    }

    async fn save_order(
        &self,
        user_id: i64,
        is_maker: bool,
        trading_pair: &str,
        order_type: OrderType,
        price: &BigDecimal,
        amount: &BigDecimal,
        filled_amount: &BigDecimal,
        status: OrderStatus,
        bank_id: Option<i64>,
    ) -> Result<MinimalOrder> {
        let order = self.order_repo
            .save_order(
                user_id,
                is_maker,
                trading_pair,
                order_type,
                price,
                amount,
                filled_amount,
                status,
                bank_id,
            )
            .await?;

        Ok(order)
    }

    #[allow(dead_code)]
    async fn create_escrow(&self, order: &MinimalOrder, status:OrderStatus) -> Result<()> {
        self.escrow_repo
            .create_escrow(order.id.clone(), order.amount.clone(), status)
            .await?;
        Ok(())
    }
}
