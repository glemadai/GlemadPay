'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const { GlemadPay, GlemadPayError, verifyWebhook } = require('..');

test('encodes paths and returns typed API errors', async () => {
  const calls = [];
  let status = 200;
  let responseBody = { order_id: 'order_1', status: 'paid', amount_minor: 100, currency: 'NGN' };
  const pay = new GlemadPay({
    apiKey: 'test-placeholder',
    baseUrl: 'https://pay.example',
    fetch: async (url, options) => {
      calls.push({ url, options });
      return new Response(JSON.stringify(responseBody), { status, headers: { 'x-request-id': 'request_1' } });
    },
  });

  await pay.payments.retrieve('order/?one');
  assert.equal(calls[0].url, 'https://pay.example/v1/orders/order%2F%3Fone');
  assert.equal(calls[0].options.redirect, 'error');
  assert.equal(JSON.stringify(pay).includes('test-placeholder'), false);

  status = 409;
  responseBody = { error: 'idempotency_conflict' };
  await assert.rejects(
    pay.payments.checkout({}),
    error => error instanceof GlemadPayError && error.status === 409 && error.requestId === 'request_1',
  );
});

test('verifies the original webhook bytes and timestamp', () => {
  const secret = 'test-webhook-secret';
  const timestamp = Math.floor(Date.now() / 1000);
  const body = Buffer.from(JSON.stringify({ event_id: 'event_1', timestamp, payment_status: 'paid' }));
  const signature = crypto.createHmac('sha256', secret).update(body).digest('hex');
  const headers = { 'glemad-signature': signature, 'glemad-timestamp': String(timestamp) };
  assert.equal(verifyWebhook(body, headers, secret), true);
  assert.equal(verifyWebhook(Buffer.from('{}'), headers, secret), false);
});
