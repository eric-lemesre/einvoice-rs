# Documentation

This folder contains contributor-facing documentation for `einvoice-rs`.

- [`references/`](references/) — condensed reference sheets for the
  standards, formats and platforms that the project implements or talks
  to. Each sheet is meant to give a contributor just enough context to
  start working on a specific crate without re-reading the full upstream
  specification every time.

## Reference index

| File                                                                     | Scope                                                              |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------ |
| [`references/en16931.md`](references/en16931.md)                         | European semantic data model (Directive 2014/55/EU, CEN TC 434)    |
| [`references/factur-x.md`](references/factur-x.md)                       | Factur-X / ZUGFeRD hybrid PDF/A-3 + CII XML format and profiles    |
| [`references/cii-uncefact.md`](references/cii-uncefact.md)               | UN/CEFACT Cross Industry Invoice schema (D16B / D22B)              |
| [`references/ubl-2.1.md`](references/ubl-2.1.md)                         | OASIS Universal Business Language 2.1 (Invoice / CreditNote)       |
| [`references/peppol-bis-billing-3.0.md`](references/peppol-bis-billing-3.0.md) | PEPPOL BIS Billing 3.0, 4-corner model, AS4, SML/SMP         |
| [`references/chorus-pro-piste.md`](references/chorus-pro-piste.md)       | Chorus Pro B2G platform and PISTE API gateway                      |
| [`references/french-2026-reform.md`](references/french-2026-reform.md)   | French e-invoicing reform (PPF, PDP, e-reporting, lifecycle)       |
| [`references/mandatory-invoice-fields.md`](references/mandatory-invoice-fields.md) | Mandatory invoice fields — EN 16931, CGI, 2026 reform new mentions |
| [`references/traceability-matrix.md`](references/traceability-matrix.md)           | EN 16931 requirements traceability — BT/BR to code and tests       |

## How the references map to the workspace

| Crate               | Primary references                                                            |
| ------------------- | ----------------------------------------------------------------------------- |
| `einvoice-core`     | `references/en16931.md`, `references/mandatory-invoice-fields.md`             |
| `einvoice-facturx`  | `references/factur-x.md`, `references/cii-uncefact.md`                        |
| `einvoice-ubl`      | `references/ubl-2.1.md`, `references/peppol-bis-billing-3.0.md`               |
| `einvoice-delivery` | `references/chorus-pro-piste.md`, `references/peppol-bis-billing-3.0.md`      |
| `einvoice-api`      | `references/french-2026-reform.md`                                            |
| `einvoice-web`      | (none specific)                                                               |

## Requirements (StrictDoc)

The [`requirements/`](requirements/) folder contains EN 16931
requirements in StrictDoc `.sdoc` format, with automated bidirectional
traceability to the Rust source code via `@relation` markers.

| File                                                                 | Scope                                              |
| -------------------------------------------------------------------- | -------------------------------------------------- |
| [`requirements/strictdoc.toml`](requirements/strictdoc.toml)         | StrictDoc project configuration                    |
| [`requirements/einvoice.sdoc`](requirements/einvoice.sdoc)           | Root document (introduction)                       |
| [`requirements/header.sdoc`](requirements/header.sdoc)               | BG-2: BT-1, BT-2, BT-3, BT-5, BT-9, BT-22, BT-24 |
| [`requirements/seller.sdoc`](requirements/seller.sdoc)               | BG-4: BT-27, BT-30, BT-31, BT-34, BT-35, BT-37, BT-38, BT-40 |
| [`requirements/buyer.sdoc`](requirements/buyer.sdoc)                 | BG-7: BT-44, BT-47, BT-48, BT-49, BT-50, BT-52, BT-53, BT-55 |
| [`requirements/line.sdoc`](requirements/line.sdoc)                   | BG-25: BT-126, BT-129, BT-130, BT-131, BT-146, BT-151, BT-152, BT-153 |
| [`requirements/vat.sdoc`](requirements/vat.sdoc)                     | BG-23: BT-116, BT-117, BT-118, BT-119             |
| [`requirements/totals.sdoc`](requirements/totals.sdoc)               | BG-22: BT-106, BT-109, BT-110, BT-112, BT-115    |
| [`requirements/business-rules.sdoc`](requirements/business-rules.sdoc) | BR-CO-10 through BR-CO-15                        |
| [`requirements/french-2026.sdoc`](requirements/french-2026.sdoc)     | French 2026 reform: SIREN, delivery, payment, period |

Generate the navigable HTML report with `just requirements-html`.
Validate `.sdoc` files and `@relation` markers with `just requirements-check`.

## Editorial rules

- Each sheet starts with a one-paragraph "What it is" section so the reader
  can decide in 10 seconds whether the page is relevant.
- All field names, identifiers and URN values are verbatim and never
  paraphrased.
- URLs at the bottom are the upstream normative sources — always prefer
  them over these summaries when writing production code.
- When a sheet becomes stale (new version, new URN, new endpoint), update
  it in the same commit that adapts the corresponding crate.
