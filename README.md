# Glemad Pay SDK

Use the Glemad Pay SDK from your server to create checkout sessions, save payment methods, issue refunds, verify customers, send payouts, and verify payment webhooks.

## Install from GitHub

Pin a release tag in production:

```bash
npm install https://github.com/glemadai/GlemadPay/archive/refs/tags/v1.1.1.tar.gz
```

For Rust:

```toml
[dependencies]
glemad-pay = { git = "https://github.com/glemadai/GlemadPay", tag = "v1.1.1" }
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
