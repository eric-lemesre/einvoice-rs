# French e-invoicing reform — 2026 / 2027

## What it is

France is rolling out a mandatory **electronic invoicing** and
**e-reporting** regime for every VAT-registered business. The reform is
grounded in **Article 153 of the 2020 Finance Law** and operationalised
by the **DGFiP** (*Direction Générale des Finances Publiques*). After
several delays, the schedule confirmed in 2025 has the first obligations
entering into force on **1 September 2026**.

## Two parallel obligations

| Obligation         | Scope                                                                               |
| ------------------ | ----------------------------------------------------------------------------------- |
| **E-invoicing**    | Domestic **B2B** transactions between French VAT-registered businesses (invoices). |
| **E-reporting**    | Non-invoice data: B2C transactions, cross-border B2B, payment data.                 |

E-invoicing carries the invoice itself between supplier and buyer.
E-reporting carries transactional data to the tax administration for
VAT reconciliation.

## Phased timeline

| Date              | Obligation                              | Companies concerned                                                 |
| ----------------- | --------------------------------------- | ------------------------------------------------------------------- |
| **1 Sept 2026**   | **Receive** e-invoices                  | **Every** French VAT-registered business, regardless of size.       |
| **1 Sept 2026**   | **Issue** e-invoices + e-reporting      | Large enterprises (*grandes entreprises*) and **ETI** (mid-caps).   |
| **1 Sept 2027**   | **Issue** e-invoices + e-reporting      | **PME** (SMEs, <250 employees) and **TPE** (micro, <10 employees).  |

Simplification measures announced on 29 August 2025:

- E-reporting for **non-established taxpayers** deferred to September 2027.
- Transactions **outside the EU** between French-established taxpayers
  excluded from e-reporting.
- No new data fields required before 1 September 2026 (to protect
  ongoing IT development).

## Architecture: PPF + PA (formerly PDP)

The original design had two operational channels: the state platform
**PPF** and the certified private platforms **PDPs**. The architecture
was significantly revised in late 2024:

| Actor                                              | Role (as of 2025)                                                                                 |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| **PPF** — *Portail Public de Facturation*          | **Not** an operational sending/receiving service anymore. Runs the **central business directory** and the **tax data hub** to the DGFiP. |
| **PA** — *Plateforme Agréée* (formerly **PDP**)    | Certified private intermediaries. Format, validate, transmit invoices and reports. **Every** business must use a PA. |
| **DGFiP**                                          | Ultimate consumer of the aggregated data.                                                         |

> In the French communications and in tools still using the old
> vocabulary you will read **PDP**. Read it as an alias for today's
> **PA** (*Plateforme Agréée*). The certification process is the same.

### What the PPF does (and doesn't do)

- ✅ Maintains the **annuaire** (central directory of businesses) used
  for routing invoices between PAs.
- ✅ Acts as the **tax data hub** — PAs forward tax-relevant data to the
  PPF, which forwards it to the DGFiP.
- ❌ Does **not** send or receive invoices on behalf of businesses.
- ❌ Does **not** store or display invoices for suppliers.

## Accepted invoice formats

All EN 16931-compliant, the **three mandated formats** are:

1. **UBL 2.1** (`Invoice` / `CreditNote`)
2. **UN/CEFACT CII** (`CrossIndustryInvoice`, D16B or D22B)
3. **Factur-X** (hybrid PDF/A-3 + CII XML)

**Paper invoices and unstructured PDFs are no longer valid** for domestic
B2B transactions once the company is subject to the mandate.

## Chorus Pro vs. PPF

| Today (B2G)                              | Tomorrow (B2B + B2G)                                                |
| ---------------------------------------- | ------------------------------------------------------------------- |
| **Chorus Pro** handles supplier-to-public-sector invoices only. | **Chorus Pro** remains the B2G channel but becomes a component of the broader **PPF / PA** ecosystem. |

Existing Chorus Pro / PISTE integrations remain valid — they will be
reused or wrapped inside the new PA flows.

## Data points a PA must transmit

E-reporting transmissions to the DGFiP include at minimum:

- Supplier / buyer identification (SIREN / VAT number).
- Invoice totals (HT, VAT, TTC).
- VAT breakdown per rate.
- Lifecycle statuses (deposited, received, paid, rejected, …).
- Transaction timestamps.

The exact structure is defined by the DGFiP in the
*Spécifications externes de la facturation électronique et de
l'e-reporting* document published on impots.gouv.fr.

## Implementation notes for `einvoice-rs`

- The **API + persistence layer** (`einvoice-api` + PostgreSQL) is the
  place where the **lifecycle of an invoice** is tracked. The
  `invoice_events` table in the initial migration is designed to record
  every status transition expected by e-reporting.
- The project targets the **three mandated formats**: Factur-X (via
  `einvoice-facturx`), UBL (via `einvoice-ubl`) and plain CII (a thin
  reuse of the CII writer inside `einvoice-facturx`).
- Certification as a **PA** is out of scope for this project. Realistic
  production use therefore means integrating with an already-certified
  PA via its API — a good parallel to the existing Chorus Pro / PISTE
  adapter in `einvoice-delivery`.

## Upstream references

- impots.gouv.fr — *Je passe à la facturation électronique* —
  <https://www.impots.gouv.fr/professionnel/je-passe-la-facturation-electronique>
- Service Public (EN) —
  <https://entreprendre.service-public.gouv.fr/actualites/A15683?lang=en>
- EY alert — *France revises schedule for adopting e-invoicing reform* —
  <https://www.ey.com/en_gl/technical/tax-alerts/france-revises-schedule-for-adopting-e-invoicing-reform>
- EY alert — *France announces simplification for September 2026* —
  <https://www.ey.com/en_gl/technical/tax-alerts/french-government-announces-simplification-measures-as-part-of-september-2026-e-invoicing-mandate>
- FNFE-MPE — <https://fnfe-mpe.org/>
