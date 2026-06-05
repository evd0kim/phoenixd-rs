# Rust SDK for [Phoenixd](https://phoenix.acinq.co/server)


## Status

### Receive
- [x] Create invoice
- [x] Create offer
- [x] Get offer
- [x] Get LN address
- [x] Get Invoice
- [x] Get Incoming Invoice
- [x] List incoming payments

### Pay
- [x] Get LN payment quote
- [x] Execute LN Payment Quote
- [x] Pay LN address
- [x] List outgoing payments
- [x] Get outgoing payment by hash
- [x] Get outgoing payment by UUID

### Node
- [x] Get node info
- [x] Get balance
- [x] Estimate liquidity fees
- [x] List channels
- [x] Decode invoice
- [x] Decode offer

### Webhook

## Minimum Supported Rust Version (MSRV)

The `phoenixd` library should always compile with any combination of features on Rust **1.63.0**.

To build and test with the MSRV you will need to pin the below dependency versions:

```shell
cargo update -p tokio --precise 1.38.1
cargo update -p reqwest --precise 0.12.4
```
