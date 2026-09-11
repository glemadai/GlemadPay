'use strict';
const crypto = require('node:crypto');

// Wire contract: HMAC-SHA256 of the exact UTF-8 body, including its timestamp.
// Multiple comma-separated digests allow receivers to change keys during rotation.
function signWebhook(event, secrets, now = Date.now()) {
  if (!Array.isArray(secrets) || secrets.length < 1 || secrets.length > 2
      || secrets.some((s) => typeof s !== 'string' || !s)) {
    throw new Error('webhook_signing_secret_unavailable');
  }
  if (!event || typeof event !== 'object' || Array.isArray(event) || !Number.isFinite(now)) {
    throw new Error('invalid_webhook_event');
  }
  const timestamp = Math.floor(now / 1000);
  const body = Buffer.from(JSON.stringify({ ...event, timestamp }), 'utf8');
  return { body, headers: {
    'Content-Type': 'application/json',
    'Glemad-Timestamp': String(timestamp),
    'Glemad-Signature': secrets.map((secret) => crypto.createHmac('sha256', secret)
      .update(body).digest('hex')).join(','),
  } };
}

function verifyWebhook(raw, headers, secret, now = Date.now()) {
  const signature = headers?.['glemad-signature'];
  const timestamp = headers?.['glemad-timestamp'];
  if (!Buffer.isBuffer(raw) || typeof secret !== 'string' || !secret
      || typeof signature !== 'string' || typeof timestamp !== 'string'
      || !/^[a-f0-9]{64}(,[a-f0-9]{64})?$/i.test(signature)
      || !/^(0|[1-9]\d{0,15})$/.test(timestamp)
      || !Number.isSafeInteger(Number(timestamp)) || !Number.isFinite(now)
      || Math.abs(now / 1000 - Number(timestamp)) > 300) return false;
  const expected = crypto.createHmac('sha256', secret).update(raw).digest();
  let matches = false;
  for (const digest of signature.split(',')) {
    matches = crypto.timingSafeEqual(expected, Buffer.from(digest, 'hex')) || matches;
  }
  if (!matches) return false;
  try { return JSON.parse(raw).timestamp === Number(timestamp); } catch (_) { return false; }
}

module.exports = { signWebhook, verifyWebhook };
