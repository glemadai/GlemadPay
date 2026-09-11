/// <reference types="node" />
export type Customer = { "product_customer_id": string; "email": string; "name"?: string; };
export type CheckoutRequest = { "order_id": string; "amount": number; "currency": string; "country": string; "customer": Customer; "callback_url"?: string; "success_url"?: string; "cancel_url"?: string; "breakdown"?: Array<{ "label": string; "amountMinor": number; }>; };
export type SetupRequest = { "order_id": string; "country": string; "customer": Customer; "callback_url"?: string; "success_url"?: string; "cancel_url"?: string; };
export type Checkout = { "order_id": string; "provider"?: string; "fallback_provider"?: string | null; "checkout_url": string; "currency"?: string; "status": string; "replayed"?: boolean; "livemode"?: boolean; };
export type ChargeRequest = { "order_id": string; "product_customer_id": string; "amount": number; "currency": string; "country"?: string; };
export type Charge = { "order_id": string; "status": string; "provider"?: string; "provider_reference"?: string | null; "replayed"?: boolean; };
export type Order = { "order_id": string; "status": string; "amount_minor": number; "currency": string; "provider"?: string | null; "checkout_url"?: string | null; "paid_at"?: string | null; "created_at"?: string; "livemode"?: boolean; };
export type RefundRequest = { "refund_id": string; "order_id": string; "amount"?: number; "reason"?: string; };
export type Refund = { "refund_id": string; "status": string; "amount_minor"?: number; "provider"?: string; "replayed"?: boolean; };
export type PaymentMethod = { "product_customer_id"?: string; "has_payment_method": boolean; "active_provider"?: string | null; "country"?: string | null; };
export type Currency = { "country": string; "currency": string; "zero_decimal": boolean; };
export type KycStatus = { "kyc_status": string; "country"?: string | null; "restricted": boolean; "payout_eligible": boolean; "rail_eligible": boolean; };
export type KycSessionRequest = { "product_customer_id": string; "country": string; "expected_details"?: { "first_name"?: string; "last_name"?: string; "date_of_birth"?: string; "gender"?: string; "nationality"?: string; "id_country"?: string; "address"?: string; "identification_number"?: string; }; "contact_details"?: { "email"?: string; "phone"?: string; }; };
export type KycSession = { "url": string; "session_id"?: string; };
export type PayoutOption = { "country": string; "currency": string; "method": "bank" | "mobile_money"; };
export type PayoutOptions = { "country": string; "options": Array<PayoutOption>; };
export type Bank = { "code": string; "name": string; };
export type RecipientRequest = { "product_customer_id": string; "country": string; "currency": string; "method": "bank" | "mobile_money"; "account_number": string; "bank_code": string; };
export type Recipient = { "recipient_id": string; "expires_at": string; "bank_name"?: string; "account_number"?: string; "account_holder": string; };
export type QuoteRequest = { "product_customer_id": string; "recipient_id": string; "amount_minor": number; "available_minor": number; };
export type Quote = { "quote_id": string; "expires_at": string; "currency": string; "amount_minor": number; "fee_minor": number; "total_minor": number; "bank_name"?: string; "account_number"?: string; "account_holder"?: string; };
export type PayoutRequest = { "product_customer_id": string; "payout_id": string; "quote_id": string; "account_ownership_confirmed": true; };
export type Payout = { "payout_id": string; "status": "processing" | "completed" | "failed" | "reversed"; "created_at"?: string; "updated_at"?: string; };
export type KeyRequest = { "mode"?: "live" | "test"; "type"?: "secret" | "publishable"; "label"?: string; };
export type ApiKey = { "id": string; "key_type": "secret" | "publishable"; "prefix": string; "label"?: string; "created_at": string; "expires_at"?: string | null; "revoked_at"?: string | null; };
export type IssuedKey = { "id": string; "key": string; "mode": "live" | "test"; "key_type": "secret" | "publishable"; "prefix"?: string; "label"?: string; "created_at"?: string; "expires_at"?: string | null; };
export type KeyList = { "mode": "live" | "test"; "keys": Array<ApiKey>; };
export type Rotation = { "overlap_seconds": number; };
export type WebhookSecret = { "secret": string; "created_at": string; "previous_expires_at"?: string | null; };
export type WebhookSecretMetadata = { "created_at": string; "previous_expires_at"?: string | null; };
export type Endpoint = { "callback_url": string; };
export type ClientConfig = { "product": string; "mode": "live" | "test"; "api_version": string; };
export type Error = { "error": string; "message"?: string; };
export type Empty = {  };
export type TestOrders = { "status": "paid" | "failed"; };
export type TestPayouts = { "status": "completed" | "failed" | "reversed"; };
export type TestKyc = { "status": "approved" | "declined" | "pending" | "expired"; };
export type AccountRequest = { "glemad_id": string; };
export type Balance = { "currency": string; "available_minor": number; };
export type Account = { "account_id": string; "account_type": "personal" | "business"; "status": "active" | "restricted" | "closed"; "balances": Array<Balance>; "created_at": string; };
export type TransferRequest = { "transfer_id": string; "destination": string; "amount": number; "currency": string; "reference": string; };
export type Transfer = { "transfer_id": string; "destination": string; "amount": number; "currency": string; "reference": string; "status": "completed" | "reversed"; "created_at": string; "replayed"?: boolean; };
export interface RequestOptions { signal?: AbortSignal; idempotencyKey?: string; }
export interface WebhookEvent { event_id: string; timestamp: number; [key: string]: unknown; }
export declare function verifyWebhook(raw: Buffer, headers: Record<string,string | string[] | undefined>, secret: string, now?: number): boolean;
export declare class GlemadPayError extends Error { status: number; code: string; data: unknown; requestId: string | null; }
export declare class GlemadPay {
constructor(options: {apiKey: string; baseUrl: string; timeoutMs?: number; fetch?: typeof fetch});
payments: {
checkout(body: CheckoutRequest, options?: RequestOptions): Promise<Checkout>;
setup(body: SetupRequest, options?: RequestOptions): Promise<Checkout>;
charge(body: ChargeRequest, options?: RequestOptions): Promise<Charge>;
refund(body: RefundRequest, options?: RequestOptions): Promise<Refund>;
retrieve(orderId: string, options?: RequestOptions): Promise<Order>;
};
customers: {
paymentMethod(productCustomerId: string, options?: RequestOptions): Promise<PaymentMethod>;
removePaymentMethod(productCustomerId: string, options?: RequestOptions): Promise<PaymentMethod>;
};
currencies: {
retrieve(query: { "country": string; }, options?: RequestOptions): Promise<Currency>;
};
kyc: {
status(query: { "product_customer_id": string; "country"?: string; }, options?: RequestOptions): Promise<KycStatus>;
createSession(body: KycSessionRequest, options?: RequestOptions): Promise<KycSession>;
};
accounts: {
create(body: AccountRequest, options?: RequestOptions): Promise<Account>;
retrieve(accountId: string, options?: RequestOptions): Promise<Account>;
};
transfers: {
create(body: TransferRequest, options?: RequestOptions): Promise<Transfer>;
retrieve(transferId: string, options?: RequestOptions): Promise<Transfer>;
};
payouts: {
options(query: { "product_customer_id": string; }, options?: RequestOptions): Promise<PayoutOptions>;
banks(query: { "product_customer_id": string; "currency": string; }, options?: RequestOptions): Promise<Bank[]>;
resolveRecipient(body: RecipientRequest, options?: RequestOptions): Promise<Recipient>;
quote(body: QuoteRequest, options?: RequestOptions): Promise<Quote>;
create(body: PayoutRequest, options?: RequestOptions): Promise<Payout>;
retrieve(payoutId: string, options?: RequestOptions): Promise<Payout>;
};
keys: {
list(options?: RequestOptions): Promise<KeyList>;
create(body: KeyRequest, options?: RequestOptions): Promise<IssuedKey>;
rotate(id: string, body: Rotation, options?: RequestOptions): Promise<IssuedKey>;
revoke(id: string, options?: RequestOptions): Promise<ApiKey>;
};
webhooks: {
secretMetadata(options?: RequestOptions): Promise<WebhookSecretMetadata>;
createSecret(options?: RequestOptions): Promise<WebhookSecret>;
rotateSecret(body: Rotation, options?: RequestOptions): Promise<WebhookSecret>;
endpoint(options?: RequestOptions): Promise<Endpoint>;
setEndpoint(body: Endpoint, options?: RequestOptions): Promise<Endpoint>;
verify: typeof verifyWebhook; constructEvent(raw: Buffer, headers: Record<string,string | string[] | undefined>, secret: string): WebhookEvent;
};
client: {
config(options?: RequestOptions): Promise<ClientConfig>;
};
testing: {
completeOrder(orderId: string, body: TestOrders, options?: RequestOptions): Promise<Order>;
completePayout(payoutId: string, body: TestPayouts, options?: RequestOptions): Promise<Payout>;
decideKyc(customerId: string, body: TestKyc, options?: RequestOptions): Promise<KycStatus>;
};
}
