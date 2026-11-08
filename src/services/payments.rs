use axum::{
    extract::Form,
    http::StatusCode,
    Json,
    Router,
    routing::post,
};
use serde::Deserialize;

use crate::db::init_pool;

#[derive(Deserialize)]
pub struct RazorpayOrderRequest {
    pub amount: u64, // amount in paise
    pub currency: String,
}

#[derive(Serialize)]
pub struct RazorpayOrderResponse {
    pub id: String,
    pub currency: String,
    pub amount: u64,
    pub receipt: String,
}

#[derive(Deserialize)]
pub struct RazorpayPaymentVerification {
    pub razorpay_order_id: String,
    pub razorpay_payment_id: String,
    pub razorpay_signature: String,
}

pub fn setup_routes() -> Router {
    Router::new()
        .route("/razorpay/create-order", post(create_razorpay_order))
        .route("/razorpay/verify", post(verify_razorpayment))
}

async fn create_razorpay_order(Form(payload): Form<RazorpayOrderRequest>) -> (StatusCode, Json<RazorpayOrderResponse>) {
    let amount = payload.amount;
    let currency = payload.currency;

    // Mock Razorpay order creation for MVP
    // In production, this would call Razorpay API
    let order_id = format!("order_rax_{}", amount);
    let receipt = format!("rabithole_{}", amount);

    let response = RazorpayOrderResponse {
        id: order_id,
        currency,
        amount,
        receipt,
    };

    (StatusCode::OK, Json(response))
}

async fn verify_razorpayment(Form(payload): Form<RazorpayPaymentVerification>) -> StatusCode {
    // Verify Razorpay payment signature
    // In production, this would verify with Razorpay's public key
    // For MVP, accept verification and upgrade user tier

    let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    // Update user tier to paid (user_id 1 for MVP)
    let _ = sqlx::query(
        "UPDATE users SET tier = 'paid' WHERE id = 1",
    )
        .execute(&pool)
        .await;

    StatusCode::OK
}