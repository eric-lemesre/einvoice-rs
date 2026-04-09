# AGENTS.md

Operational guide for AI coding agents working on **einvoice-rs**. Human
contributors should start with [`README.md`](README.md); this file holds
the agent-oriented context: conventions, commands, gotchas.

## Project overview

`einvoice-rs` is a Rust workspace implementing electronic invoicing
compliant with **EN 16931** and the French 2026 e-invoicing reform. It
produces structured invoice payloads (**Factur-X**, **UBL 2.1**),
transmits them via **Chorus Pro / PISTE** or email, tracks lifecycle
events in PostgreSQL, and exposes a REST API + an SSR web frontend.

Reference sheets (standards, formats, platforms) live under
[`docs/references/`](docs/references/). Treat them as the source of
truth — the files are condensed for agent consumption and always link
back to the upstream normative documents.

## Workspace layout

```
crates/
  core/       # Domain model (Invoice, Party, LineItem), traits, errors
  facturx/    # Factur-X serializer (CII XML + PDF/A-3 wrapping TODO)
  ubl/        # UBL 2.1 serializer (Chorus Pro / PEPPOL BIS 3.0) — stub
  delivery/   # SMTP email + Chorus Pro PISTE client — stubs
  api/        # axum REST API + sqlx + PostgreSQL
  web/        # axum + askama SSR frontend
docs/
  references/ # Reference sheets: EN 16931, Factur-X, CII, UBL, PEPPOL,
              # Chorus Pro, French 2026 reform, mandatory fields
migrations/   # sqlx migrations (initial schema: invoices + invoice_events)
.github/
  workflows/  # CI: rustfmt, clippy -D warnings, test (with Postgres), cargo-audit
```

Each crate is pinned via `workspace.dependencies` in the root
`Cargo.toml`. Internal crates are referenced as
`einvoice-core = { workspace = true }`, etc. **Always add new shared
dependencies to the workspace table first**, then reference them from
crate manifests with `{ workspace = true }`.

## Dev environment

- Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml)
  (`stable`, `minimal` profile, with `rustfmt` + `clippy`).
- PostgreSQL 16+ required for the `api` crate and migrations.
- [`just`](https://github.com/casey/just) is the canonical task runner.
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli) ≥ 0.8 for migrations.
  Install via `just install-sqlx`.
- Copy `.env.example` to `.env` before running `api` / `web`; the
  `justfile` loads `.env` automatically (`set dotenv-load := true`).

## Build, test, lint commands

Always prefer `just` recipes — they are the single source of truth for
local and CI parity.

| Recipe             | Command                                               |
| ------------------ | ----------------------------------------------------- |
| `just build`       | `cargo build --workspace`                             |
| `just test`        | `cargo test --workspace`                              |
| `just fmt`         | `cargo fmt --all`                                     |
| `just fmt-check`   | `cargo fmt --all -- --check` (CI-equivalent)          |
| `just lint`        | `cargo clippy --workspace --all-targets -- -D warnings` |
| `just check`       | `fmt-check` + `lint` + `test` — **run before every commit** |
| `just run-api`     | `cargo run -p einvoice-api`                           |
| `just run-web`     | `cargo run -p einvoice-web`                           |
| `just db-up`       | `docker run` a local PostgreSQL 16                    |
| `just db-down`     | Destroy the local PostgreSQL container                |
| `just db-migrate`  | `sqlx migrate run` against `$DATABASE_URL`            |
| `just db-revert`   | Revert the last migration                             |

Per-crate tests: `cargo test -p einvoice-facturx` (substitute the crate
name). Per-crate checks: `cargo check -p einvoice-ubl`.

**CI must stay green.** GitHub Actions runs `cargo fmt -- --check`,
`cargo clippy -D warnings`, `cargo test --workspace --all-targets`
against a live PostgreSQL service, and `cargo audit`. Reproduce the full
pipeline locally with `just check` and `cargo audit` before pushing.

## Code style and conventions

- **Edition**: `2021`. **MSRV**: `1.80`.
- **`cargo fmt`** with default settings is mandatory. Never hand-format.
- **Clippy with `-D warnings`** — any lint failure blocks CI. Prefer
  fixing the root cause over `#[allow(...)]`. If an allow is necessary,
  scope it as narrowly as possible and leave a comment.
- **Module documentation** — every `lib.rs` starts with a `//!` module
  doc explaining the crate's responsibility and pointing to the
  relevant reference sheet in `docs/references/`.
- **Public items** carry `///` doc comments. French is allowed in docs
  (the project targets the French market) but identifiers, types and
  error messages stay in **English**.
- **Error handling**: every crate funnels through `einvoice_core::Error`
  (`Validation` / `Serialization` / `Delivery` / `Io` / `Json`). Build
  errors via the helper constructors: `Error::validation(...)`,
  `Error::serialization(...)`, `Error::delivery(...)`. Do **not**
  introduce per-crate error enums.
- **Money** is `rust_decimal::Decimal`. Never use `f64` for amounts or
  percentages. Format amounts with `format!("{value:.2}")`. VAT rates
  are stored as fractions (`dec!(0.20)`) and converted to percents at
  serialization time only.
- **Dates** are `chrono::NaiveDate` for date-only values. For CII
  documents use the compact form `YYYYMMDD` via
  `date.format("%Y%m%d")`.
- **XML writers** for CII and UBL must be manual `quick_xml::Writer`
  pipelines. Do **not** rely on serde derives for output: element
  ordering is normative in CII and mismatches break Schematron
  validation.
- **Namespaces** are collected in a private `mod ns` inside each
  serializer crate (see `crates/facturx/src/cii.rs`). Keep URNs
  verbatim — they are copy-pasted from upstream specs.
- **No backwards-compat shims.** This project is pre-1.0. If a rename
  or delete is needed, do it outright.

## Testing instructions

- Unit tests live next to the code they test in `#[cfg(test)] mod
  tests` blocks. Integration tests go in `crates/<name>/tests/`.
- Use `rust_decimal_macros::dec!` for decimal literals in tests
  (already in `dev-dependencies` where needed).
- When touching serializers (Factur-X / UBL), add at least:
  1. a happy-path test that asserts key substrings and attribute
     values (e.g. `xml.contains("<ram:TypeCode>380</ram:TypeCode>")`);
  2. a "well-formed" test that re-parses the output with
     `quick_xml::Reader` to ensure no malformed XML slips through;
  3. a total-computation test with mixed VAT rates so the VAT
     breakdown grouping logic stays covered.
- Use `cargo test -p <crate> -- --nocapture` to inspect generated XML
  / PDF bytes while debugging.
- The `api` crate integration tests require a reachable PostgreSQL
  (CI injects `DATABASE_URL=postgres://postgres:postgres@localhost:5432/einvoice_test`).
  Locally: `just db-up && just db-migrate` first.

## Security and secrets

- **Never** commit `.env`, `.pem`, `.p12`, credential files, or
  anything under `crates/*/fixtures/real-*`. Only `.env.example`
  belongs in version control.
- **Never** log the values of `CHORUS_PRO_CLIENT_SECRET`,
  `Authorization` headers, PISTE access tokens, SMTP passwords, or
  invoice PII. Use `tracing` fields carefully and prefer redacted
  identifiers.
- Chorus Pro integration must default to the **sandbox** base URL
  (`https://sandbox-api.piste.gouv.fr`). Production (`api.piste.gouv.fr`)
  is opt-in via an explicit env var change.
- Database connections use `rustls` (never native-tls) — see the
  workspace `sqlx` features. Do **not** enable `tls-native-tls`.
- All HTTP clients (`reqwest`, `lettre`) go through `rustls-tls`. Stay
  away from `default-features = true` on these crates.
- `cargo audit` runs in CI. When it flags an advisory, upgrade rather
  than ignore. An ignore requires a justification in the commit body.

## Invoice domain rules

- The validator layer (`einvoice_core::InvoiceValidator`) is the only
  place to enforce EN 16931 business rules. See
  [`docs/references/mandatory-invoice-fields.md`](docs/references/mandatory-invoice-fields.md)
  for the exhaustive list (core BTs, PEPPOL extras, French CGI
  mentions, and the four new 2026-reform mentions).
- Serializers must **refuse** to emit a document when the validator
  returns an error. Return `Error::Validation` verbatim; do not
  downgrade to `Error::Serialization`.
- Factur-X profile defaults to `En16931`. Changing the default is a
  breaking API change — flag it loudly in the PR description.
- French-specific mandatory mentions that have no semantic BT mapping
  (late payment penalties, €40 flat fee, sector-specific notices) are
  rendered by the **template layer**, not by the validator. Keep that
  split.

## PR and commit guidelines

- **Commits** are small and focused. Prefer several commits over a
  large mixed one. Each commit must compile and pass `just check`.
- **Commit messages** follow the existing style:
  - First line: imperative, ≤ 72 chars, no trailing period. Examples
    already in history: `"Bootstrap workspace, CI, migrations and
    reference docs"`, `"Implement CII XML serializer for Factur-X"`.
  - Blank line, then a wrapped (≤ 72 cols) body explaining **why** and
    listing the concrete scope.
  - Co-author trailer when produced with an agent:
    `Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>`.
- **Do not commit** unless the user explicitly asks ("commit", "push",
  "PR"). Agents should propose and let the human confirm.
- **Never** use `git commit --amend`, `git push --force`, or
  `git rebase -i` autonomously. If a pre-commit hook fails, fix the
  issue and create a **new** commit.
- **Never** run `git add -A` or `git add .` — stage files explicitly
  by name to avoid including `.idea/`, `.env`, or stray build
  artefacts.
- Use `HEREDOC` for multi-line commit messages through `git commit -m`
  to preserve formatting.
- When a change touches a reference sheet in `docs/references/` and the
  related crate, **bundle them in the same commit** so documentation
  never lags behind the code.

## Documentation expectations

- Adding a new standard, format or platform → write a new sheet under
  `docs/references/` following the existing structure (What it is /
  Structure / Implementation notes for `einvoice-rs` / Upstream
  references). Add it to the index tables in
  [`docs/README.md`](docs/README.md) **and** the "References" section
  of [`README.md`](README.md).
- Adding a new crate → register it under `[workspace] members` in
  `Cargo.toml`, add a workspace dependency entry, and document its
  responsibility in `README.md` and `docs/README.md`.
- Reference sheets must quote field identifiers (`BT-*`, URNs,
  endpoint paths) **verbatim** — never paraphrase them.

## Gotchas and known quirks

- `rust_decimal::Decimal::to_string()` preserves scale. `dec!(2)` gives
  `"2"` but `dec!(2.00)` gives `"2.00"`. Always use `format!("{v:.2}")`
  for monetary amounts to get a stable 2-decimal representation.
- `quick_xml` 0.36 auto-escapes text via `BytesText::new` and
  attributes via `push_attribute`. Do not pre-escape.
- `ram:ApplicableHeaderTradeDelivery` in CII has cardinality `1..1` but
  may be empty — use `Event::Empty` to produce a self-closing tag.
- `ram:TaxTotalAmount` in CII **must** carry a `currencyID` attribute;
  sibling monetary elements in the same summation block **must not**.
- The PPF is no longer an operational sending channel (see
  `docs/references/french-2026-reform.md`). Any code that routes via
  `PPF` is stale — target a **PA** (Plateforme Agréée) instead.
- `Chorus Pro` lifecycle status names are French and carried verbatim
  in API responses (`deposee`, `mise_a_disposition`, …). Keep them as
  string literals in `invoice_events.payload` rather than translating.

## When in doubt

1. Consult the matching sheet in `docs/references/`.
2. Cross-check with the upstream URL listed at the bottom of the sheet.
3. If the spec is ambiguous, prefer the **interpretation enforced by
   the EN 16931 Schematron artefacts** from
   [ConnectingEurope/eInvoicing-EN16931](https://github.com/ConnectingEurope/eInvoicing-EN16931).
4. If still unclear, ask the human reviewer in the PR rather than
   guessing — invoicing errors have legal consequences.
