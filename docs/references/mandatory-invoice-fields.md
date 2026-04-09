# Mandatory invoice fields — EN 16931 + French law

## What it is

This sheet consolidates every field an invoice **must** carry to be
simultaneously:

1. compliant with **EN 16931** (European semantic data model),
2. compliant with the **French Code général des impôts** (CGI) and
   **Code de commerce** mandatory mentions for B2B invoicing,
3. ready for the **French 2026/2027 e-invoicing reform** — including the
   four new mandatory mentions introduced by **decree n° 2022-1299** of
   7 October 2022.

It is the checklist that `einvoice-core::InvoiceValidator` is expected
to enforce before any serializer (`einvoice-facturx`, `einvoice-ubl`)
produces a document. Values are expressed as EN 16931 Business Terms
(`BT-*`) and Business Groups (`BG-*`) — syntax bindings are in the
dedicated [`ubl-2.1.md`](ubl-2.1.md) and
[`cii-uncefact.md`](cii-uncefact.md) sheets.

## 1. EN 16931 core — mandatory business terms

EN 16931 defines ~170 business terms. Those listed below are the **core
mandatory set** — every conforming invoice must carry them regardless of
the chosen syntax (UBL or CII).

### Header — document identification

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-1    | Invoice number                         | Unique sequential identifier                     |
| BT-2    | Invoice issue date                     | ISO 8601 (`YYYY-MM-DD`)                          |
| BT-3    | Invoice type code                      | UN/ECE 1001 (`380` commercial, `381` credit note, …) |
| BT-5    | Invoice currency code                  | ISO 4217 (`EUR`, `USD`, …)                       |
| BT-24   | Specification identifier               | URN of the CIUS / customization (e.g. `urn:cen.eu:en16931:2017`) |

### Seller (BG-4)

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-27   | Seller name                            | Legal name                                       |
| BT-31   | Seller VAT identifier                  | Required if the seller is VAT-registered         |
| BT-35   | Seller address line 1                  | Part of BG-5 (postal address, mandatory)         |
| BT-37   | Seller city                            | Part of BG-5                                     |
| BT-38   | Seller post code                       | Part of BG-5                                     |
| BT-40   | Seller country code                    | ISO 3166-1 alpha-2 — always mandatory            |

### Buyer (BG-7)

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-44   | Buyer name                             | Legal name                                       |
| BT-48   | Buyer VAT identifier                   | Required for intra-EU B2B                        |
| BT-50   | Buyer address line 1                   | Part of BG-8 (postal address, mandatory)         |
| BT-52   | Buyer city                             | Part of BG-8                                     |
| BT-53   | Buyer post code                        | Part of BG-8                                     |
| BT-55   | Buyer country code                     | ISO 3166-1 alpha-2 — always mandatory            |

### Invoice line (BG-25) — at least one

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-126  | Invoice line identifier                | Sequential within the invoice                    |
| BT-129  | Invoiced quantity                      | Decimal                                          |
| BT-130  | Invoiced quantity unit of measure code | UN/ECE Rec 20 (`H87` piece, `MTR` metre, …)      |
| BT-131  | Invoice line net amount                | = (BT-129 × BT-146) − BT-136 + BT-141            |
| BT-146  | Item net price                         | Unit price excluding VAT                         |
| BT-151  | Invoiced item VAT category code        | `S` standard, `Z` zero, `E` exempt, `AE` reverse charge, `K` intra-EU, `G` export, `O` outside scope |
| BT-153  | Item name                              | Textual description                              |

### VAT breakdown (BG-23) — at least one subtotal

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-116  | VAT category taxable amount            | Σ of BT-131 for the category                     |
| BT-117  | VAT category tax amount                | BT-116 × (BT-119 / 100)                          |
| BT-118  | VAT category code                      | Same codes as BT-151                             |
| BT-119  | VAT category rate                      | Percentage (mandatory except when `E`, `AE`, `K`, `G`, `O`) |

### Document totals (BG-22) — always mandatory

| BT      | Name                                   | Notes                                            |
| ------- | -------------------------------------- | ------------------------------------------------ |
| BT-106  | Sum of invoice line net amounts        | Σ(BT-131)                                        |
| BT-109  | Invoice total amount **without VAT**   | Tax exclusive total                              |
| BT-110  | Invoice total VAT amount               | Σ(BT-117)                                        |
| BT-112  | Invoice total amount **with VAT**      | Tax inclusive total                              |
| BT-115  | Amount due for payment                 | BT-112 − prepaid amount                          |

### Business rules touching mandatory fields (selection)

- `BR-CO-10` — BT-106 must equal Σ(BT-131).
- `BR-CO-13` — BT-109 must equal BT-106 − BT-107 + BT-108 (doc-level allowances/charges).
- `BR-CO-15` — BT-112 must equal BT-109 + BT-110.
- `BR-CO-16` — BT-115 must equal BT-112 − BT-113 (prepaid amount).
- `BR-S-*`, `BR-Z-*`, `BR-E-*`, `BR-AE-*`, `BR-IC-*`, `BR-G-*`, `BR-O-*`
  — one family per VAT category code, each enforces the presence /
  absence of BT-119 and the BT-120 exemption reason.

The full list is in the Schematron artefacts shipped by
[ConnectingEurope/eInvoicing-EN16931](https://github.com/ConnectingEurope/eInvoicing-EN16931).

## 2. PEPPOL BIS Billing 3.0 — extras on top of EN 16931

PEPPOL promotes a few optional EN 16931 fields to **mandatory** for
cross-border interoperability:

- **BT-24** `CustomizationID` must be exactly
  `urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0`.
- **BT-23** `ProfileID` must be
  `urn:fdc:peppol.eu:2017:poacc:billing:01:1.0`.
- **Seller and Buyer electronic address** (`EndpointID` with PEPPOL
  scheme, e.g. `0088`, `0009`, `0002`, `9906`, …) — mandatory for the
  routing through the 4-corner network.
- **Seller legal entity identifier** (BT-30) — mandatory in the PEPPOL
  CIUS for most member states.

See [`peppol-bis-billing-3.0.md`](peppol-bis-billing-3.0.md) for the
participant identifier schemes.

## 3. French mandatory mentions — CGI + Code de commerce

French B2B invoicing mandatory mentions come from two legal sources:

- **CGI art. 242 nonies A** (tax / VAT mentions)
- **Code de commerce art. L441-9** and **L441-10** (commercial mentions,
  payment terms, late payment penalties)

### Seller identification

- Legal name (`dénomination sociale`) or first/last name for a sole
  trader (`entrepreneur individuel`, `micro-entrepreneur`).
- Postal address of the registered office.
- **SIREN** (9 digits) or **SIRET** (14 digits) of the issuing
  establishment.
- **RCS** (*Registre du Commerce et des Sociétés*) + city of
  registration for commercial companies.
- **Répertoire des Métiers** + department for artisans.
- Legal form and share capital for sociétés (SA, SAS, SARL…).
- **Intra-EU VAT number** of the seller (mandatory as soon as total
  HT > €150 or the customer is in another EU member state).
- For a member of an *association de gestion agréée* or *centre de
  gestion agréé*: the mention *"Membre d'une association agréée, le
  règlement par chèque et par carte bancaire est accepté"*.
- **"TVA non applicable, art. 293 B du CGI"** if the seller is under
  the `franchise en base` regime.

### Buyer identification

- Legal name (or first/last name).
- Billing address.
- **Intra-EU VAT number** of the buyer (mandatory ≥ €150 HT or for
  intra-EU B2B under the reverse charge mechanism).
- Delivery address if different from the billing address (**new** in
  2026, see §4 below).

### Invoice metadata

- **Issue date** of the invoice.
- **Unique sequential number** based on a continuous chronological
  sequence, with no gap — one single sequence per site / per activity
  is acceptable.
- **Date of sale / of the end of the service**, if different from the
  issue date.
- **Purchase order number** if the buyer has provided one.

### Pricing per line

- Designation and precise quantity of each good or service.
- Unit price excluding VAT (HT).
- Any rebate / discount (*rabais, remises, ristournes*) directly
  attached to the transaction.
- **VAT rate** applied to each line (or, in mixed invoices, a
  per-line indication of the applicable rate).
- Total **HT** per rate.
- Total **TVA** per rate.
- Grand total **HT** (BT-109).
- Grand total **TTC** (BT-112).

### VAT special mentions

- **"Auto-liquidation"** for a reverse-charge invoice (e.g. subcontracted
  construction work, intra-EU B2B services under art. 196 VAT directive).
- **"TVA non applicable, art. 293 B du CGI"** for franchise en base.
- **"Exonération de TVA, art. 262 ter I du CGI"** for intra-EU
  deliveries of goods.
- **"TVA sur les encaissements"** or **"TVA sur les débits"** depending
  on the chosen regime.
- Exemption reference (CGI article number) for any exempt line.

### Payment terms — Code de commerce L441-9 / L441-10

- **Date or deadline** for payment (including any negotiated longer
  term up to the 60-day / 45-day-end-of-month cap).
- **Early-payment discount conditions** (*conditions d'escompte*) or the
  explicit mention **"Escompte pour paiement anticipé : néant"**.
- **Late payment interest rate** (BCE refinancing rate + 10 pts by
  default).
- **Fixed recovery fee of €40** for late payment (*indemnité forfaitaire
  pour frais de recouvrement*).

### Sectoral mentions (non-exhaustive)

- **Professional liability insurance (RCP)** name, address of the
  insurer and geographical scope, mandatory for regulated artisanal
  activities (construction, food handling, hairdressing…).
- **DEEE eco-participation** (*Déchets d'Équipements Électriques et
  Électroniques*) separately shown for EEE sales.
- **"Membre d'un centre de gestion agréé"** where applicable.

### Sanctions

Missing mentions are sanctioned under **CGI art. 1737-II**:

- **€15** per mission mention, **per invoice**.
- Capped at **1/4 of the invoice amount**.
- Missing / false VAT-related mentions can trigger up to **50 %** of
  the VAT reclaimed.

## 4. Four new mandatory mentions — French 2026 reform

Introduced by **decree n° 2022-1299 of 7 October 2022**, these four
fields become mandatory alongside the phased e-invoicing / e-reporting
calendar:

| #   | New mandatory mention                                  | BT mapping (proposed)                  |
| --- | ------------------------------------------------------ | -------------------------------------- |
| 1   | **Buyer SIREN** (9 digits)                             | Buyer legal registration ID — BT-47 + scheme `0002` / `0009` |
| 2   | **Delivery address** of the goods, if different from the billing address | BG-15 *Deliver To Address* + BT-72 actual delivery date |
| 3   | **Nature of the operation**: *livraison de biens*, *prestation de services*, or *mixed* | New `InvoiceNote` with subject code (draft DGFiP codes `SRV`, `GDS`, `MIX`) |
| 4   | **"Option pour le paiement de la taxe d'après les débits"** if the supplier has elected this regime | `InvoiceNote` / document-level note with code `AAK` |

These four mentions are in addition to the pre-existing CGI art. 242
nonies A list. They are **mandatory from** the day the supplier becomes
subject to the e-invoicing obligation:

| Date            | Concerned companies                              |
| --------------- | ------------------------------------------------ |
| 1 Sept 2026     | *Grandes entreprises* and ETI (mid-caps)         |
| 1 Sept 2027     | PME (< 250 employees) and TPE (< 10 employees)   |

Until those dates the four mentions are **recommended** but not yet
legally required.

## 5. Cross-check matrix — where each obligation is enforced

| Obligation                                              | Enforced in                                                   |
| ------------------------------------------------------- | ------------------------------------------------------------- |
| Presence of BT-1 / BT-2 / BT-3 / BT-5                   | `einvoice-core::InvoiceValidator` (semantic check)            |
| Presence of BT-24 CustomizationID                       | Serializer that owns the URN (`einvoice-facturx`, `einvoice-ubl`) |
| Seller / Buyer BG-4 / BG-7 cardinalities                | `einvoice-core::InvoiceValidator`                             |
| At least one BG-25 line + BT-126/129/131/151            | `einvoice-core::InvoiceValidator`                             |
| VAT breakdown BG-23 consistency (BR-CO-15/16)           | `einvoice-core::InvoiceValidator`                             |
| PEPPOL `EndpointID` with valid scheme                   | `einvoice-ubl::UblSerializer` (Peppol profile only)           |
| French legal mentions (text blocks)                     | `einvoice-core::Invoice` — free-text fields + template layer  |
| **SIREN buyer** (new mention 1)                         | `einvoice-core::InvoiceValidator` **+** template              |
| **Delivery address** (new mention 2)                    | `einvoice-core::Invoice` BG-15 — validator checks presence    |
| **Nature of operation** (new mention 3)                 | `einvoice-core::Invoice` enum, propagated to InvoiceNote code |
| **"Option débits"** (new mention 4)                     | Configuration flag on the supplier profile                    |
| Payment terms (L441-9/-10)                              | `einvoice-core::Invoice` + template                           |
| €40 flat fee, late penalties text                       | Template layer (legal boilerplate, not validated)             |

`InvoiceValidator` must fail fast with `Error::Validation` for every
missing field. The template layer is responsible for rendering the
legally required text blocks that are **not** part of the EN 16931
semantic model (e.g. the €40 flat fee mention).

## 6. Activation calendar

| Obligation                                                               | From                |
| ------------------------------------------------------------------------ | ------------------- |
| EN 16931 mandatory BT set                                                | immediately         |
| PEPPOL BIS 3.0 customization URN (for Peppol profile)                    | immediately         |
| French CGI art. 242 nonies A mandatory mentions                          | already in force    |
| 4 new mandatory mentions (decree 2022-1299)                              | 1 Sept 2026 (GE/ETI), 1 Sept 2027 (PME/TPE) |
| Reception of e-invoices via a PA                                         | 1 Sept 2026 — all VAT-registered businesses |

Until the new mentions become legally required the validator should
emit a **warning** (not an error) when they are missing, so customers
can opt-in early.

## Upstream references

- EN 16931 validation artefacts — <https://github.com/ConnectingEurope/eInvoicing-EN16931>
- PEPPOL BIS Billing 3.0 — <https://docs.peppol.eu/poacc/billing/3.0/>
- CGI art. 242 nonies A —
  <https://www.legifrance.gouv.fr/codes/article_lc/LEGIARTI000044988041>
- Decree n° 2022-1299 of 7 October 2022 —
  <https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000046388331>
- Service Public — *Facture entre professionnels : mentions obligatoires* —
  <https://entreprendre.service-public.gouv.fr/vosdroits/F31808>
- impots.gouv.fr — *Je passe à la facturation électronique* —
  <https://www.impots.gouv.fr/professionnel/je-passe-la-facturation-electronique>
- Code de commerce art. L441-9 —
  <https://www.legifrance.gouv.fr/codes/article_lc/LEGIARTI000038414160>
- Code de commerce art. L441-10 —
  <https://www.legifrance.gouv.fr/codes/article_lc/LEGIARTI000038414143>
