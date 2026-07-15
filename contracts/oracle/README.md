# Oracle Contract

> **Status: Future Module** — scaffolded and ready for integration.

Price feed and exchange rate oracle for the FusePay platform.

## Design

- Admin registers named feeds (e.g. `"XLM/USDC"`)
- Admin authorises trusted price providers
- Providers submit updated prices via `submit_price`
- Consumers call `get_price` — prices older than `max_staleness_ledgers` are rejected

## Public API

| Function | Auth | Description |
|----------|------|-------------|
| `initialize(admin, max_staleness)` | admin | One-time init |
| `register_feed(admin, feed_id, base, quote, decimals)` | admin | Register feed |
| `add_provider(admin, provider)` | admin | Authorise provider |
| `remove_provider(admin, provider)` | admin | Remove provider |
| `submit_price(provider, feed_id, price)` | provider | Update price |
| `get_price(feed_id)` | — | Get latest price |

## Planned Integrations

- [Reflector Protocol](https://reflector.network/) — decentralised Stellar oracle
- Multi-provider TWAP (time-weighted average price) aggregation
- Price bounds validation against a configurable deviation threshold

## Security Notes

- Only registered providers can submit prices
- Stale data is rejected automatically via `max_staleness_ledgers`
- Price of 0 or negative is rejected at submission time
- Admin cannot read prices that other providers haven't submitted
