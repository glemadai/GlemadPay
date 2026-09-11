// Generated from registered API operations.
impl GlemadPayClient {
    /// Create a hosted checkout.
    pub async fn payments_checkout(&self, body: &types::CheckoutRequest) -> anyhow::Result<types::Checkout> {
        let request = self.http.post(format!("{}/v1/checkout", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Save a payment method with hosted checkout.
    pub async fn payments_setup(&self, body: &types::SetupRequest) -> anyhow::Result<types::Checkout> {
        let request = self.http.post(format!("{}/v1/setup", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Charge a saved payment method.
    pub async fn payments_charge(&self, body: &types::ChargeRequest) -> anyhow::Result<types::Charge> {
        let request = self.http.post(format!("{}/v1/charge", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Refund a paid order.
    pub async fn payments_refund(&self, body: &types::RefundRequest) -> anyhow::Result<types::Refund> {
        let request = self.http.post(format!("{}/v1/refund", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Retrieve an order.
    pub async fn payments_retrieve(&self, order_id: &str) -> anyhow::Result<types::Order> {
        let request = self.http.get(format!("{}/v1/orders/{}", self.base_url, encode_segment(order_id)?));
        self.api_request(request).await
    }
    /// Get the saved payment method summary.
    pub async fn customers_payment_method(&self, product_customer_id: &str) -> anyhow::Result<types::PaymentMethod> {
        let request = self.http.get(format!("{}/v1/customers/{}/payment-method", self.base_url, encode_segment(product_customer_id)?));
        self.api_request(request).await
    }
    /// Remove a saved payment method.
    pub async fn customers_remove_payment_method(&self, product_customer_id: &str) -> anyhow::Result<types::PaymentMethod> {
        let request = self.http.post(format!("{}/v1/customers/{}/payment-method/remove", self.base_url, encode_segment(product_customer_id)?));
        self.api_request(request).await
    }
    /// Resolve the checkout currency.
    pub async fn currencies_retrieve(&self, country: &str) -> anyhow::Result<types::Currency> {
        let request = self.http.get(format!("{}/v1/currency-for-country", self.base_url));
        let request = request.query(&[("country",country)]);
        self.api_request(request).await
    }
    /// Read verification status.
    pub async fn kyc_status(&self, product_customer_id: &str, country: Option<&str>) -> anyhow::Result<types::KycStatus> {
        let request = self.http.get(format!("{}/v1/kyc/status", self.base_url));
        let request = request.query(&[("product_customer_id",product_customer_id)]);
        let request = if let Some(value) = country { request.query(&[("country",value)]) } else { request };
        self.api_request(request).await
    }
    /// Start document, liveness and face verification.
    pub async fn kyc_create_session(&self, body: &types::KycSessionRequest) -> anyhow::Result<types::KycSession> {
        let request = self.http.post(format!("{}/v1/kyc/sessions", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// List available routes for a verified customer.
    pub async fn payouts_options(&self, product_customer_id: &str) -> anyhow::Result<types::PayoutOptions> {
        let request = self.http.get(format!("{}/v1/payouts/options", self.base_url));
        let request = request.query(&[("product_customer_id",product_customer_id)]);
        self.api_request(request).await
    }
    /// List destination banks.
    pub async fn payouts_banks(&self, product_customer_id: &str, currency: &str) -> anyhow::Result<Vec<types::Bank>> {
        let request = self.http.get(format!("{}/v1/payouts/banks", self.base_url));
        let request = request.query(&[("product_customer_id",product_customer_id)]);
        let request = request.query(&[("currency",currency)]);
        self.api_request(request).await
    }
    /// Resolve and store a payout recipient.
    pub async fn payouts_resolve_recipient(&self, body: &types::RecipientRequest) -> anyhow::Result<types::Recipient> {
        let request = self.http.post(format!("{}/v1/payouts/recipients/resolve", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Quote amount, fee and total debit.
    pub async fn payouts_quote(&self, body: &types::QuoteRequest) -> anyhow::Result<types::Quote> {
        let request = self.http.post(format!("{}/v1/payouts/quotes", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Confirm a quoted payout.
    pub async fn payouts_create(&self, body: &types::PayoutRequest) -> anyhow::Result<types::Payout> {
        let request = self.http.post(format!("{}/v1/payouts", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Retrieve payout status.
    pub async fn payouts_retrieve(&self, payout_id: &str) -> anyhow::Result<types::Payout> {
        let request = self.http.get(format!("{}/v1/payouts/{}", self.base_url, encode_segment(payout_id)?));
        self.api_request(request).await
    }
    /// List key metadata.
    pub async fn keys_list(&self) -> anyhow::Result<types::KeyList> {
        let request = self.http.get(format!("{}/v1/api-keys", self.base_url));
        self.api_request(request).await
    }
    /// Create a key and reveal it once.
    pub async fn keys_create(&self, body: &types::KeyRequest) -> anyhow::Result<types::IssuedKey> {
        let request = self.http.post(format!("{}/v1/api-keys", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Rotate a key with explicit overlap.
    pub async fn keys_rotate(&self, id: &str, body: &types::Rotation) -> anyhow::Result<types::IssuedKey> {
        let request = self.http.post(format!("{}/v1/api-keys/{}/rotate", self.base_url, encode_segment(id)?));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Revoke a key immediately.
    pub async fn keys_revoke(&self, id: &str) -> anyhow::Result<types::ApiKey> {
        let request = self.http.post(format!("{}/v1/api-keys/{}/revoke", self.base_url, encode_segment(id)?));
        self.api_request(request).await
    }
    /// Read signing-secret metadata.
    pub async fn webhooks_secret_metadata(&self) -> anyhow::Result<types::WebhookSecretMetadata> {
        let request = self.http.get(format!("{}/v1/webhook-signing-secret", self.base_url));
        self.api_request(request).await
    }
    /// Create a signing secret and reveal it once.
    pub async fn webhooks_create_secret(&self) -> anyhow::Result<types::WebhookSecret> {
        let request = self.http.post(format!("{}/v1/webhook-signing-secret", self.base_url));
        self.api_request(request).await
    }
    /// Rotate the signing secret.
    pub async fn webhooks_rotate_secret(&self, body: &types::Rotation) -> anyhow::Result<types::WebhookSecret> {
        let request = self.http.post(format!("{}/v1/webhook-signing-secret/rotate", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Read the registered callback URL.
    pub async fn webhooks_endpoint(&self) -> anyhow::Result<types::Endpoint> {
        let request = self.http.get(format!("{}/v1/webhook-endpoint", self.base_url));
        self.api_request(request).await
    }
    /// Register the HTTPS callback URL.
    pub async fn webhooks_set_endpoint(&self, body: &types::Endpoint) -> anyhow::Result<types::Endpoint> {
        let request = self.http.post(format!("{}/v1/webhook-endpoint", self.base_url));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Read public client configuration.
    pub async fn client_config(&self) -> anyhow::Result<types::ClientConfig> {
        let request = self.http.get(format!("{}/v1/client-config", self.base_url));
        self.api_request(request).await
    }
    /// Simulate orders outcome (test keys only).
    pub async fn testing_complete_order(&self, order_id: &str, body: &types::TestOrders) -> anyhow::Result<types::Order> {
        let request = self.http.post(format!("{}/v1/testing/orders/{}/complete", self.base_url, encode_segment(order_id)?));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Simulate payouts outcome (test keys only).
    pub async fn testing_complete_payout(&self, payout_id: &str, body: &types::TestPayouts) -> anyhow::Result<types::Payout> {
        let request = self.http.post(format!("{}/v1/testing/payouts/{}/complete", self.base_url, encode_segment(payout_id)?));
        let request = request.json(body);
        self.api_request(request).await
    }
    /// Simulate kyc outcome (test keys only).
    pub async fn testing_decide_kyc(&self, customer_id: &str, body: &types::TestKyc) -> anyhow::Result<types::KycStatus> {
        let request = self.http.post(format!("{}/v1/testing/kyc/{}/decision", self.base_url, encode_segment(customer_id)?));
        let request = request.json(body);
        self.api_request(request).await
    }
}
