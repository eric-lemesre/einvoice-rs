# UN/CEFACT Cross Industry Invoice (CII)

## What it is

The **Cross Industry Invoice (CII)** is the XML schema published by the
**United Nations Centre for Trade Facilitation and Electronic Business
(UN/CEFACT)** to describe invoices in a neutral, cross-industry way. It is
one of the two syntaxes explicitly bound to **EN 16931** (the other being
OASIS UBL), and it is the XML format embedded inside **Factur-X** and
**ZUGFeRD** hybrid documents.

## Schema versions

UN/CEFACT publishes its reference data periodically. Each release is
identified by a **year + batch letter**:

| Version | Release       | Used by                                                |
| ------- | ------------- | ------------------------------------------------------ |
| D16B    | 2016 batch B  | Factur-X 1.0 → 1.07, ZUGFeRD 2.0 → 2.3, EN 16931-3-3  |
| D22B    | 2022 batch B  | Factur-X 1.08, ZUGFeRD 2.4                             |

Versions are **cumulative**: any element defined in D16B is still present
in D22B. Going from D16B to D22B means replacing the XSD files; already
generated XML documents remain valid.

Going the other way is **not safe**: an XML that uses elements added in
D22B will fail to validate against the D16B XSD.

## Document structure

The root element is `rsm:CrossIndustryInvoice` and the document is made of
four top-level sections:

```xml
<rsm:CrossIndustryInvoice
    xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100"
    xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100"
    xmlns:qdt="urn:un:unece:uncefact:data:standard:QualifiedDataType:100"
    xmlns:udt="urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100">

  <rsm:ExchangedDocumentContext>        <!-- profile URN, test indicator -->
    ...
  </rsm:ExchangedDocumentContext>

  <rsm:ExchangedDocument>                <!-- invoice header: ID, date, type -->
    ...
  </rsm:ExchangedDocument>

  <rsm:SupplyChainTradeTransaction>      <!-- lines, agreement, settlement -->
    <ram:IncludedSupplyChainTradeLineItem>...</ram:IncludedSupplyChainTradeLineItem>
    <ram:ApplicableHeaderTradeAgreement>...</ram:ApplicableHeaderTradeAgreement>
    <ram:ApplicableHeaderTradeDelivery>...</ram:ApplicableHeaderTradeDelivery>
    <ram:ApplicableHeaderTradeSettlement>...</ram:ApplicableHeaderTradeSettlement>
  </rsm:SupplyChainTradeTransaction>

</rsm:CrossIndustryInvoice>
```

### Main XML namespaces

| Prefix | URI                                                                                    |
| ------ | -------------------------------------------------------------------------------------- |
| `rsm`  | `urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100`                         |
| `ram`  | `urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100`   |
| `qdt`  | `urn:un:unece:uncefact:data:standard:QualifiedDataType:100`                            |
| `udt`  | `urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100`                          |

The namespaces do **not** change between D16B and D22B.

## Key elements mapped to EN 16931 business terms

| BT      | CII element (XPath)                                                                                            |
| ------- | -------------------------------------------------------------------------------------------------------------- |
| BT-1    | `rsm:ExchangedDocument/ram:ID`                                                                                 |
| BT-2    | `rsm:ExchangedDocument/ram:IssueDateTime/udt:DateTimeString`                                                   |
| BT-3    | `rsm:ExchangedDocument/ram:TypeCode`                                                                           |
| BT-5    | `…/ram:ApplicableHeaderTradeSettlement/ram:InvoiceCurrencyCode`                                                |
| BT-27   | `…/ram:ApplicableHeaderTradeAgreement/ram:SellerTradeParty/ram:Name`                                           |
| BT-44   | `…/ram:ApplicableHeaderTradeAgreement/ram:BuyerTradeParty/ram:Name`                                            |
| BT-109  | `…/ram:SpecifiedTradeSettlementHeaderMonetarySummation/ram:TaxBasisTotalAmount`                                |
| BT-110  | `…/ram:SpecifiedTradeSettlementHeaderMonetarySummation/ram:TaxTotalAmount`                                     |
| BT-112  | `…/ram:SpecifiedTradeSettlementHeaderMonetarySummation/ram:GrandTotalAmount`                                   |
| BT-115  | `…/ram:SpecifiedTradeSettlementHeaderMonetarySummation/ram:DuePayableAmount`                                   |

The full mapping table lives in **EN 16931-3-3**.

## Implementation notes for `einvoice-rs`

- Pick **D22B** as the target XSD in `crates/facturx`. It's a strict
  superset of D16B and is the version used by Factur-X 1.08.
- Keep namespace URIs in a single `const` module so they are reused by
  both the CII writer (`einvoice-facturx`) and any future CII reader.
- Prefer a **manual `quick-xml::Writer`** pipeline over generic serde
  serialization: CII has strict element ordering constraints that are
  hard to enforce via derive macros and easy to enforce via explicit
  calls.

## Upstream references

- UN/CEFACT mainstandards — <https://unece.org/trade/uncefact/mainstandards>
- EN 16931 CII XSDs (D16B) —
  <https://github.com/ConnectingEurope/eInvoicing-EN16931/tree/master/cii>
- Factur-X 1.08 release notes —
  <https://fnfe-mpe.org/wp-content/uploads/2025/12/2025-12-04_Factur-X_1.08_ZUGFeRD_2.4_Press_Release_EN.pdf>
