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

## How the references map to the workspace

| Crate               | Primary references                                                            |
| ------------------- | ----------------------------------------------------------------------------- |
| `einvoice-core`     | `references/en16931.md`                                                       |
| `einvoice-facturx`  | `references/factur-x.md`, `references/cii-uncefact.md`                        |
| `einvoice-ubl`      | `references/ubl-2.1.md`, `references/peppol-bis-billing-3.0.md`               |
| `einvoice-delivery` | `references/chorus-pro-piste.md`, `references/peppol-bis-billing-3.0.md`      |
| `einvoice-api`      | `references/french-2026-reform.md`                                            |
| `einvoice-web`      | (none specific)                                                               |

## Editorial rules

- Each sheet starts with a one-paragraph "What it is" section so the reader
  can decide in 10 seconds whether the page is relevant.
- All field names, identifiers and URN values are verbatim and never
  paraphrased.
- URLs at the bottom are the upstream normative sources — always prefer
  them over these summaries when writing production code.
- When a sheet becomes stale (new version, new URN, new endpoint), update
  it in the same commit that adapts the corresponding crate.
