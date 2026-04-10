# einvoice-rs

Electronic invoicing in Rust — compliant with the European standard
**EN 16931** and the French 2026 e-invoicing reform.

The project covers the whole pipeline: invoice domain modelling, generation
of structured formats (**Factur-X**, **UBL 2.1**), transmission via
**Chorus Pro / PISTE** or email, lifecycle tracking, a REST API and an SSR
web frontend.

## Architecture

Cargo workspace organised into specialised crates:

| Crate               | Responsibility                                                         |
| ------------------- | ---------------------------------------------------------------------- |
| `einvoice-core`     | Domain model (`Invoice`, `Party`, `LineItem`), traits, error types     |
| `einvoice-facturx`  | Factur-X serializer (PDF/A-3 + CII XML, EN 16931 profiles)             |
| `einvoice-ubl`      | UBL 2.1 serializer (Chorus Pro / PEPPOL BIS Billing 3.0)               |
| `einvoice-delivery` | Delivery adapters: SMTP email (`lettre`) + Chorus Pro PISTE API        |
| `einvoice-api`      | `axum` REST API (entry point, PostgreSQL persistence)                  |
| `einvoice-web`      | SSR frontend (`axum` + `askama`)                                       |

See [`docs/`](docs/README.md) for a curated set of reference sheets covering
EN 16931, Factur-X, UBL 2.1, PEPPOL BIS Billing 3.0, Chorus Pro / PISTE and
the French 2026 reform.

## Requirements

- Rust stable (pinned in [`rust-toolchain.toml`](rust-toolchain.toml))
- PostgreSQL 16+
- [`just`](https://github.com/casey/just) (optional, for dev recipes)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli) for database migrations

## Quick start

```bash
cp .env.example .env
just db-up          # start PostgreSQL in Docker
just db-migrate     # apply migrations
just run-api        # start the REST API on :3000
just run-web        # start the web frontend on :8080
```

## Development

```bash
just check          # fmt + clippy + test
just test           # run workspace tests
just fmt            # cargo fmt --all
just lint           # cargo clippy -- -D warnings
```

## Project status

Project is in its bootstrap phase. `einvoice-facturx::FacturXSerializer`
produces a valid **Cross Industry Invoice** XML (UN/CEFACT D22B) via
`FacturXSerializer::serialize_xml`, and `einvoice-ubl::UblSerializer`
emits a canonical **UBL 2.1 Invoice** document for both the Chorus Pro
and PEPPOL BIS Billing 3.0 profiles. The `delivery` crate still exposes
typed stubs. The next implementation milestone is the PDF/A-3 envelope
wrapper around the CII XML and the Chorus Pro / PISTE HTTP client.

## References

Reference sheets are available under [`docs/references/`](docs/references/):

- [`docs/references/en16931.md`](docs/references/en16931.md) — European semantic standard
- [`docs/references/factur-x.md`](docs/references/factur-x.md) — hybrid PDF/A-3 + XML CII format
- [`docs/references/cii-uncefact.md`](docs/references/cii-uncefact.md) — UN/CEFACT Cross Industry Invoice
- [`docs/references/ubl-2.1.md`](docs/references/ubl-2.1.md) — OASIS Universal Business Language
- [`docs/references/peppol-bis-billing-3.0.md`](docs/references/peppol-bis-billing-3.0.md) — PEPPOL rules for cross-border billing
- [`docs/references/chorus-pro-piste.md`](docs/references/chorus-pro-piste.md) — French public platform API
- [`docs/references/french-2026-reform.md`](docs/references/french-2026-reform.md) — PPF, PDP, e-reporting
- [`docs/references/mandatory-invoice-fields.md`](docs/references/mandatory-invoice-fields.md) — mandatory invoice fields (EN 16931, CGI, 2026 reform)

## License

Dual-licensed under either MIT or Apache-2.0, at your option.
