# UBL 2.1 — Universal Business Language

## What it is

**UBL (Universal Business Language)** is the OASIS family of XML schemas
covering the whole procure-to-pay and order-to-cash business document
lifecycle. Version **2.1** was ratified as an **OASIS Standard on
2013-11-04** and is the version explicitly bound to EN 16931 (see
`EN 16931-3-2`) and used as the default syntax in **PEPPOL BIS Billing
3.0**.

Unlike Factur-X, a UBL invoice is a **plain XML document** (no PDF
envelope). It is the format of choice for automated flows — Chorus Pro
submission, PEPPOL networks, EDI pipelines.

## Document types relevant to this project

UBL 2.1 defines **65+ document types**. For `einvoice-rs` we care about:

| Document        | Root element     | Purpose                                     |
| --------------- | ---------------- | ------------------------------------------- |
| **Invoice**     | `ubl:Invoice`    | Commercial invoice                          |
| **CreditNote**  | `ubl:CreditNote` | Credit note (negative invoice)              |
| **DebitNote**   | `ubl:DebitNote`  | Debit note                                  |

Out of scope (for now): `Order`, `OrderResponse`, `DespatchAdvice`,
`ReceiptAdvice`, `Catalogue`, `Quotation`, `RemittanceAdvice`, etc.

## XML namespaces

UBL separates reusable components into well-known namespaces. Three of
them appear in almost every invoice document:

| Prefix | Namespace URI                                                                           |
| ------ | --------------------------------------------------------------------------------------- |
| `ubl`  | `urn:oasis:names:specification:ubl:schema:xsd:Invoice-2`                                |
| `cbc`  | `urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2`                  |
| `cac`  | `urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2`              |

The `ubl` prefix changes depending on the document root
(`Invoice-2`, `CreditNote-2`, etc.). `cbc` and `cac` are stable and
always present.

## Invoice document skeleton

```xml
<Invoice
    xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
    xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
    xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2">

  <cbc:CustomizationID>...</cbc:CustomizationID>
  <cbc:ProfileID>...</cbc:ProfileID>
  <cbc:ID>INV-2026-0001</cbc:ID>
  <cbc:IssueDate>2026-04-09</cbc:IssueDate>
  <cbc:DueDate>2026-05-09</cbc:DueDate>
  <cbc:InvoiceTypeCode>380</cbc:InvoiceTypeCode>
  <cbc:DocumentCurrencyCode>EUR</cbc:DocumentCurrencyCode>

  <cac:AccountingSupplierParty>
    <cac:Party>...</cac:Party>
  </cac:AccountingSupplierParty>

  <cac:AccountingCustomerParty>
    <cac:Party>...</cac:Party>
  </cac:AccountingCustomerParty>

  <cac:TaxTotal>
    <cbc:TaxAmount currencyID="EUR">...</cbc:TaxAmount>
    <cac:TaxSubtotal>...</cac:TaxSubtotal>
  </cac:TaxTotal>

  <cac:LegalMonetaryTotal>
    <cbc:LineExtensionAmount currencyID="EUR">...</cbc:LineExtensionAmount>
    <cbc:TaxExclusiveAmount  currencyID="EUR">...</cbc:TaxExclusiveAmount>
    <cbc:TaxInclusiveAmount  currencyID="EUR">...</cbc:TaxInclusiveAmount>
    <cbc:PayableAmount       currencyID="EUR">...</cbc:PayableAmount>
  </cac:LegalMonetaryTotal>

  <cac:InvoiceLine>
    <cbc:ID>1</cbc:ID>
    <cbc:InvoicedQuantity unitCode="H87">2</cbc:InvoicedQuantity>
    <cbc:LineExtensionAmount currencyID="EUR">200.00</cbc:LineExtensionAmount>
    <cac:Item>...</cac:Item>
    <cac:Price>...</cac:Price>
  </cac:InvoiceLine>

</Invoice>
```

## EN 16931 binding highlights

| BT      | UBL XPath                                                           |
| ------- | ------------------------------------------------------------------- |
| BT-1    | `/Invoice/cbc:ID`                                                   |
| BT-2    | `/Invoice/cbc:IssueDate`                                            |
| BT-3    | `/Invoice/cbc:InvoiceTypeCode`                                      |
| BT-5    | `/Invoice/cbc:DocumentCurrencyCode`                                 |
| BT-24   | `/Invoice/cbc:CustomizationID` (specification identifier URN)       |
| BT-27   | `/Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyName/cbc:Name` |
| BT-44   | `/Invoice/cac:AccountingCustomerParty/cac:Party/cac:PartyName/cbc:Name` |
| BT-109  | `/Invoice/cac:LegalMonetaryTotal/cbc:TaxExclusiveAmount`            |
| BT-110  | `/Invoice/cac:TaxTotal/cbc:TaxAmount`                               |
| BT-112  | `/Invoice/cac:LegalMonetaryTotal/cbc:TaxInclusiveAmount`            |
| BT-115  | `/Invoice/cac:LegalMonetaryTotal/cbc:PayableAmount`                 |

Full mapping lives in **EN 16931-3-2**.

## Customization / profile identifiers

The `CustomizationID` tells a validator which set of business rules to
apply on top of plain UBL. For Chorus Pro and PEPPOL we use:

| Profile                    | CustomizationID                                                                             |
| -------------------------- | ------------------------------------------------------------------------------------------- |
| Plain EN 16931             | `urn:cen.eu:en16931:2017`                                                                   |
| PEPPOL BIS Billing 3.0     | `urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0`                |
| PEPPOL BIS Billing ProfileID | `urn:fdc:peppol.eu:2017:poacc:billing:01:1.0`                                             |

Chorus Pro accepts `urn:cen.eu:en16931:2017` as well as the PEPPOL
customization.

## Implementation notes for `einvoice-rs`

- `crates/ubl` exposes a `UblProfile` enum with `ChorusPro` and
  `PeppolBis3` variants. The methods `customization_id()` and
  `profile_id()` must return the URNs verbatim.
- Produce the XML with an explicit `quick-xml::Writer` pipeline. UBL is
  far more forgiving than CII on element ordering, but currency codes and
  `currencyID` attributes must be consistent across the whole document.
- Do not parse back your own output for tests — validate against the
  official **PEPPOL BIS** Schematron artefacts (see references below).

## Upstream references

- UBL 2.1 OASIS Standard — <https://docs.oasis-open.org/ubl/UBL-2.1.html>
- UBL 2.1 XSDs — <https://docs.oasis-open.org/ubl/os-UBL-2.1/xsd/>
- EN 16931-3-2 UBL binding (via CEN) — <https://standards.cencenelec.eu/>
- PEPPOL BIS Billing 3.0 — <https://docs.peppol.eu/poacc/billing/3.0/>
