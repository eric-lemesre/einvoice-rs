# EN 16931 Requirements Traceability Matrix

> **SUPERSEDED** — This manual Markdown matrix has been replaced by
> StrictDoc `.sdoc` files under `docs/requirements/` with automated
> bidirectional traceability via `@relation` markers in the Rust source.
> Use `just requirements-html` to generate the navigable HTML report.
> This file is kept for historical reference only.

This matrix maps EN 16931 Business Terms (BT) and Business Rules (BR) to
the source code and tests that implement them. Status values:

- **Done** — implemented and tested
- **Partial** — implemented, not fully tested or missing edge cases
- **TODO** — not yet implemented

## Header (BG-2 / top-level)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-1 | Invoice number | `crates/core/src/model.rs` — `Invoice::number` | `invoice_totals_are_correct` | Done |
| BT-2 | Invoice issue date | `crates/core/src/model.rs` — `Invoice::issue_date` | `invoice_totals_are_correct` | Done |
| BT-3 | Invoice type code | hardcoded `380` in serializers | `writes_header_metadata` (cii, ubl) | Done |
| BT-5 | Invoice currency code | `crates/core/src/model.rs` — `Invoice::currency` | `invoice_totals_are_correct` | Done |
| BT-9 | Payment due date | `crates/core/src/model.rs` — `Invoice::due_date` | `writes_payment_terms_when_due_date_set` | Done |
| BT-22 | Invoice note | `crates/core/src/model.rs` — `Invoice::notes` | `omits_optional_fields_when_missing` (ubl) | Done |
| BT-24 | Specification identifier | `FacturXProfile::specification_identifier`, `UblProfile::customization_id` | `embeds_profile_identifier`, `embeds_profile_identifiers` | Done |

## Seller (BG-4)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-27 | Seller name | `crates/core/src/model.rs` — `Party::name` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-30 | Seller legal registration ID | `crates/core/src/model.rs` — `Party::legal_id` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-31 | Seller VAT identifier | `crates/core/src/model.rs` — `Party::vat_number` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-34 | Seller electronic address | `crates/core/src/model.rs` — `Party::email` | — | Partial |
| BT-35 | Seller address line 1 | `crates/core/src/model.rs` — `Address::street` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-37 | Seller city | `crates/core/src/model.rs` — `Address::city` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-38 | Seller post code | `crates/core/src/model.rs` — `Address::postal_code` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-40 | Seller country code | `crates/core/src/model.rs` — `Address::country_code` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |

## Buyer (BG-7)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-44 | Buyer name | `crates/core/src/model.rs` — `Party::name` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-47 | Buyer legal registration ID | `crates/core/src/model.rs` — `Party::legal_id` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-48 | Buyer VAT identifier | `crates/core/src/model.rs` — `Party::vat_number` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-49 | Buyer electronic address | `crates/core/src/model.rs` — `Party::email` | — | Partial |
| BT-50 | Buyer address line 1 | `crates/core/src/model.rs` — `Address::street` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-52 | Buyer city | `crates/core/src/model.rs` — `Address::city` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-53 | Buyer post code | `crates/core/src/model.rs` — `Address::postal_code` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |
| BT-55 | Buyer country code | `crates/core/src/model.rs` — `Address::country_code` | `writes_seller_and_buyer`, `writes_supplier_and_customer_parties` | Done |

## Invoice Line (BG-25)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-126 | Invoice line identifier | sequential index in serializers | `writes_lines_and_line_totals`, `writes_invoice_lines` | Done |
| BT-129 | Invoiced quantity | `crates/core/src/model.rs` — `LineItem::quantity` | `invoice_totals_are_correct` | Done |
| BT-130 | Invoiced quantity unit of measure | hardcoded `C62` (unit) | `writes_lines_and_line_totals`, `writes_invoice_lines` | Done |
| BT-131 | Invoice line net amount | `crates/core/src/model.rs` — `LineItem::line_total()` | `invoice_totals_are_correct`, `writes_lines_and_line_totals` | Done |
| BT-146 | Item net price | `crates/core/src/model.rs` — `LineItem::unit_price` | `invoice_totals_are_correct` | Done |
| BT-151 | Invoiced item VAT category code | hardcoded `S` (standard) | `writes_vat_breakdown_and_totals` | Done |
| BT-152 | Invoiced item VAT rate | `crates/core/src/model.rs` — `LineItem::tax_rate` | `invoice_totals_are_correct`, `groups_multiple_vat_rates` | Done |
| BT-153 | Item name | `crates/core/src/model.rs` — `LineItem::description` | `writes_lines_and_line_totals`, `writes_invoice_lines` | Done |

## VAT Breakdown (BG-23)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-116 | VAT category taxable amount | computed in serializers (group by rate) | `writes_vat_breakdown_and_totals`, `groups_multiple_vat_rates` | Done |
| BT-117 | VAT category tax amount | `crates/core/src/model.rs` — `LineItem::tax_amount()` | `invoice_totals_are_correct`, `writes_vat_breakdown_and_totals` | Done |
| BT-118 | VAT category code | hardcoded `S` | `writes_vat_breakdown_and_totals` | Done |
| BT-119 | VAT category rate | derived from `LineItem::tax_rate` | `groups_multiple_vat_rates` | Done |

## Document Totals (BG-22)

| BT/BR | Description | Source | Test(s) | Status |
|-------|-------------|--------|---------|--------|
| BT-106 | Sum of Invoice line net amount | `crates/core/src/model.rs` — `Invoice::subtotal()` | `invoice_totals_are_correct` | Done |
| BT-109 | Invoice total amount without VAT | `crates/core/src/model.rs` — `Invoice::subtotal()` | `invoice_totals_are_correct`, `writes_legal_monetary_total` | Done |
| BT-110 | Invoice total VAT amount | `crates/core/src/model.rs` — `Invoice::tax_total()` | `invoice_totals_are_correct`, `writes_tax_total_and_subtotal` | Done |
| BT-112 | Invoice total amount with VAT | `crates/core/src/model.rs` — `Invoice::total()` | `invoice_totals_are_correct`, `writes_legal_monetary_total` | Done |
| BT-115 | Amount due for payment | `crates/core/src/model.rs` — `Invoice::total()` | `invoice_totals_are_correct`, `writes_legal_monetary_total` | Done |

## Business Rules (BR-CO-*)

| BR | Description | Status |
|----|-------------|--------|
| BR-CO-10 | Sum of line net amounts = Invoice total without VAT | Done (`Invoice::subtotal()` sums line totals) |
| BR-CO-11 | Sum of allowances at document level | TODO (no document-level allowances yet) |
| BR-CO-12 | Sum of charges at document level | TODO (no document-level charges yet) |
| BR-CO-13 | Invoice total without VAT = sum of line net amounts - allowances + charges | Partial (no allowances/charges) |
| BR-CO-14 | Invoice total with VAT = total without VAT + total VAT | Done (`Invoice::total()`) |
| BR-CO-15 | Amount due = total with VAT - paid amount | Partial (no prepaid amount yet) |

## French 2026 Reform Specific

| Requirement | Description | Status |
|-------------|-------------|--------|
| SIREN/SIRET | Seller and buyer legal ID | Done (`Party::legal_id`) |
| Delivery address | Address of goods delivery | TODO |
| Payment means | Payment method reference | TODO |
| Billing period | Service period start/end | TODO |
