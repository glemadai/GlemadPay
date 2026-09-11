'use strict';
const { operations } = require('./contract.json');
const { verifyWebhook } = require('./webhookSignature');

class GlemadPayError extends Error {
  constructor(status, data, requestId) {
    super(data?.error || `glemad_pay_http_${status}`);
    this.name = 'GlemadPayError';
    this.status = status;
    this.code = data?.error || 'invalid_response';
    this.data = data;
    this.requestId = requestId;
  }
}
class GlemadPay {
  constructor({ apiKey, baseUrl, timeoutMs = 25000, fetch: transport = globalThis.fetch }) {
    if (typeof apiKey !== 'string' || !apiKey.trim()) throw new Error('apiKey is required');
    const url = new URL(baseUrl);
    if (url.protocol !== 'https:' || url.username || url.password || url.search || url.hash || !['','/'].includes(url.pathname)) {
      throw new Error('baseUrl must be an HTTPS origin');
    }
    if (!Number.isSafeInteger(timeoutMs) || timeoutMs <= 0) throw new Error('timeoutMs must be positive');
    if (typeof transport !== 'function') throw new Error('fetch is required');
    // Credentials live in a closure and are not enumerable or JSON-serializable.
    for (const op of operations) {
      this[op.group] ||= {};
      this[op.group][op.name] = async (...args) => {
        let route = op.path;
        for (const param of op.params) {
          const value = args.shift();
          if (typeof value !== 'string' || !value || value === '.' || value === '..') throw new Error(`${param} is required`);
          route = route.replace(':' + param, encodeURIComponent(value));
        }
        const endpoint = new URL('/v1' + route, url.origin);
        let body;
        if (Object.keys(op.query).length) {
          const query = args.shift() || {};
          for (const [name, schema] of Object.entries(op.query)) {
            if (query[name] == null && !schema.optional) throw new Error(`${name} is required`);
            if (query[name] != null) endpoint.searchParams.set(name, String(query[name]));
          }
        } else if (op.request) body = JSON.stringify(args.shift() || {});
        const options = args.shift() || {};
        const response = await transport(endpoint.href, {
          method: op.method.toUpperCase(), redirect: 'error',
          headers: { Authorization: `Bearer ${apiKey}`, 'Content-Type': 'application/json',
            'Glemad-SDK': 'node/1.0.0', ...(options.idempotencyKey ? {'Idempotency-Key': options.idempotencyKey} : {}) },
          body, signal: options.signal || AbortSignal.timeout(timeoutMs),
        });
        const raw = await response.text();
        let data;
        try { data = raw ? JSON.parse(raw) : null; }
        catch (_) { throw new GlemadPayError(response.status, {error:'invalid_json_response'}, response.headers.get('x-request-id')); }
        if (!response.ok) throw new GlemadPayError(response.status, data, response.headers.get('x-request-id'));
        return data;
      };
    }
    this.webhooks.verify = verifyWebhook;
    this.webhooks.constructEvent = (raw, headers, secret) => {
      if (!verifyWebhook(raw, headers, secret)) throw new GlemadPayError(401, {error:'invalid_webhook_signature'});
      return JSON.parse(raw);
    };
  }
}
module.exports = { GlemadPay, GlemadPayError, verifyWebhook };
