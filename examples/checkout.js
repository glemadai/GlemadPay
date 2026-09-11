'use strict';
const { GlemadPay } = require('@glemad/pay');

const pay = new GlemadPay({
  apiKey: process.env.GLEMAD_PAY_SECRET_KEY,
  baseUrl: 'https://pay-api.glemad.com',
});

async function createCheckout() {
  return pay.payments.checkout({
    order_id: 'order_1001',
    amount: 250000,
    currency: 'NGN',
    country: 'NG',
    customer: {
      product_customer_id: 'customer_42',
      email: 'ada@example.com',
      name: 'Ada Okafor',
    },
    callback_url: 'https://example.com/webhooks/glemad-pay',
    success_url: 'https://example.com/orders/order_1001',
    cancel_url: 'https://example.com/checkout',
  });
}

module.exports = { createCheckout };
