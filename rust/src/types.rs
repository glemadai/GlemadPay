// Generated from src/apiContract.json.
use serde::{Serialize,Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Customer {
    pub product_customer_id: String,
    pub email: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
    pub country: String,
    pub customer: Customer,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub success_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub breakdown: Option<Vec<CheckoutRequestBreakdownItem>>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetupRequest {
    pub order_id: String,
    pub country: String,
    pub customer: Customer,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub success_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub cancel_url: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Checkout {
    pub order_id: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub fallback_provider: Option<String>,
    pub checkout_url: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub currency: Option<String>,
    pub status: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub replayed: Option<bool>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub livemode: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChargeRequest {
    pub order_id: String,
    pub product_customer_id: String,
    pub amount: i64,
    pub currency: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub country: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Charge {
    pub order_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub provider_reference: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub replayed: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub status: String,
    pub amount_minor: i64,
    pub currency: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub checkout_url: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub livemode: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefundRequest {
    pub refund_id: String,
    pub order_id: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub amount: Option<i64>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Refund {
    pub refund_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub amount_minor: Option<i64>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub replayed: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub product_customer_id: Option<String>,
    pub has_payment_method: bool,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub active_provider: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub country: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Currency {
    pub country: String,
    pub currency: String,
    pub zero_decimal: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KycStatus {
    pub kyc_status: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub country: Option<String>,
    pub restricted: bool,
    pub payout_eligible: bool,
    pub rail_eligible: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KycSessionRequest {
    pub product_customer_id: String,
    pub country: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub expected_details: Option<KycSessionRequestExpectedDetails>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub contact_details: Option<KycSessionRequestContactDetails>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KycSession {
    pub url: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub session_id: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PayoutOption {
    pub country: String,
    pub currency: String,
    pub method: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PayoutOptions {
    pub country: String,
    pub options: Vec<PayoutOption>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bank {
    pub code: String,
    pub name: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecipientRequest {
    pub product_customer_id: String,
    pub country: String,
    pub currency: String,
    pub method: String,
    pub account_number: String,
    pub bank_code: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Recipient {
    pub recipient_id: String,
    pub expires_at: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub account_number: Option<String>,
    pub account_holder: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub product_customer_id: String,
    pub recipient_id: String,
    pub amount_minor: i64,
    pub available_minor: i64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Quote {
    pub quote_id: String,
    pub expires_at: String,
    pub currency: String,
    pub amount_minor: i64,
    pub fee_minor: i64,
    pub total_minor: i64,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub account_number: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub account_holder: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PayoutRequest {
    pub product_customer_id: String,
    pub payout_id: String,
    pub quote_id: String,
    pub account_ownership_confirmed: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Payout {
    pub payout_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyRequest {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub key_type: String,
    pub prefix: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub revoked_at: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssuedKey {
    pub id: String,
    pub key: String,
    pub mode: String,
    pub key_type: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub expires_at: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyList {
    pub mode: String,
    pub keys: Vec<ApiKey>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rotation {
    pub overlap_seconds: i64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookSecret {
    pub secret: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub previous_expires_at: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookSecretMetadata {
    pub created_at: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub previous_expires_at: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Endpoint {
    pub callback_url: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientConfig {
    pub product: String,
    pub mode: String,
    pub api_version: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Empty {
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestOrders {
    pub status: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestPayouts {
    pub status: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestKyc {
    pub status: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckoutRequestBreakdownItem {
    pub label: String,
    #[serde(rename="amountMinor")]
    pub amount_minor: i64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KycSessionRequestExpectedDetails {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub gender: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub nationality: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub id_country: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub identification_number: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KycSessionRequestContactDetails {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,
}
