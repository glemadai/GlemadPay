# Glemad Pay SDK

Use the Glemad Pay SDK from your server to create checkout sessions, save payment methods, issue refunds, verify customers, send payouts, and verify payment webhooks.

## Install from GitHub

Pin a release tag in production:

```bash
npm install https://github.com/glemadai/GlemadPay/archive/refs/tags/v1.1.0.tar.gz
```

For Rust:

```toml
[dependencies]
glemad-pay = { git = "https://github.com/glemadai/GlemadPay", tag = "v1.1.0" }
```

## Create a client

```js
const { GlemadPay } = require("@glemad/pay");

const pay = new GlemadPay({
  apiKey: process.env.GLEMAD_PAY_SECRET_KEY,
  baseUrl: "https://pay-api.glemad.com",
});
```

Keep secret keys on your server. Do not commit them to Git or include them in a website or mobile application.

## Create checkout

```js
const checkout = await pay.payments.checkout({
  order_id: "order_1001",
  amount: 250000,
  currency: "NGN",
  country: "NG",
  customer: {
    product_customer_id: "customer_42",
    email: "ada@example.com",
    name: "Ada Okafor",
  },
  callback_url: "https://example.com/webhooks/glemad-pay",
  success_url: "https://example.com/orders/order_1001",
  cancel_url: "https://example.com/checkout",
});
```

Redirect the customer to `checkout.checkout_url`. Confirm payment from a verified webhook or by retrieving the order. Do not use the browser redirect as proof of payment.

## Pay a connected account

Create or connect the person's Glemad Pay account with the stable user ID from Glemad ID:

```js
const account = await pay.accounts.create({
  glemad_id: "1d932902-ff2d-4a82-98a3-9d12c36b7a6a",
});
```

Store `account.account_id` on your platform. When earnings become eligible, credit the connected account from your server:

```js
const transfer = await pay.transfers.create({
  transfer_id: "nabtap_2026_w37_creator_42",
  destination: account.account_id,
  amount: 250000,
  currency: "NGN",
  reference: "Nabtap creator earnings for 7–13 September 2026",
});
```

Use a permanent, unique `transfer_id` for each settlement. Retrying the same transfer is safe. Glemad Pay rejects it if the destination, amount, currency, or reference changes.

## Verify a webhook

Configure Express to preserve the original request body for this route:

```js
app.post(
  "/webhooks/glemad-pay",
  express.raw({ type: "application/json" }),
  async (request, response) => {
    let event;
    try {
      event = pay.webhooks.constructEvent(
        request.body,
        request.headers,
        process.env.GLEMAD_PAY_WEBHOOK_SECRET,
      );
    } catch {
      return response.sendStatus(401);
    }

    await processEventOnce(event.event_id, event);
    return response.sendStatus(204);
  },
);
```

Register this route before `express.json()`.

## Documentation

Read the public integration guide in Glemad Developer. The public OpenAPI 3.1 document is available in [openapi.json](openapi.json).

## Support

Use [Glemad Support](https://support.glemad.com) for integration help or to report a security concern.
