# Factur-X / ZUGFeRD — hybrid PDF/A-3 + CII XML

## What it is

**Factur-X** is the Franco-German hybrid electronic invoice standard,
published jointly by **FNFE-MPE** (France) and **FeRD** (Germany). It is a
**PDF/A-3 document** that embeds a machine-readable **UN/CEFACT Cross
Industry Invoice (CII) XML** file, giving a single artefact that is at the
same time a human-readable PDF and a structured invoice.

Factur-X is technically **identical** to **ZUGFeRD 2.x**: the two names
refer to the same specification published under two different brands.

It was the first implementation of the European semantic standard
**EN 16931**.

## Current version

| Artefact | Version | In force                      |
| -------- | ------- | ----------------------------- |
| Factur-X | 1.08    | effective **2026-01-15**      |
| ZUGFeRD  | 2.4     | effective **2026-01-15**      |

Version 1.08 / 2.4 introduces **sub-line management** (subtotal lines,
kits, bundles, composite items) required by the French and German
e-invoicing reforms.

## Technical structure

| Element              | Value                                                     |
| -------------------- | --------------------------------------------------------- |
| Container            | **PDF/A-3** (ISO 19005-3)                                 |
| Embedded file name   | `factur-x.xml`                                            |
| XML schema           | **UN/CEFACT CII D22B** (backward compatible with D16B)    |
| XML root element     | `rsm:CrossIndustryInvoice`                                |
| Metadata             | XMP extension schema `urn:factur-x:pdfa:CrossIndustryDocument` |

The CII XML must be declared as an **embedded file** in the PDF with
`AFRelationship = Alternative` and the MIME type `application/xml`.

## The five profiles

Each profile has its own **XSD** and **Schematron** validation artefacts.

| Profile         | Purpose                                                                    | Specification identifier (URN)                                                      |
| --------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| **MINIMUM**     | Bare minimum required by Chorus Pro — header/footer, equivalent to OCR    | `urn:factur-x.eu:1p0:minimum`                                                       |
| **BASIC WL**    | Header/footer data commonly needed by buyers; **no line items**           | `urn:factur-x.eu:1p0:basicwl`                                                       |
| **BASIC**       | BASIC WL plus essential invoice lines                                      | `urn:cen.eu:en16931:2017#compliant#urn:factur-x.eu:1p0:basic`                       |
| **EN 16931**    | Full conformance to EN 16931 core invoice                                  | `urn:cen.eu:en16931:2017`                                                           |
| **EXTENDED**    | EN 16931 + additional fields; **EXTENDED-CTC-FR** variant for France      | `urn:cen.eu:en16931:2017#conformant#urn:factur-x.eu:1p0:extended`                   |

These identifiers are carried by the CII element
`ram:GuidelineSpecifiedDocumentContextParameter/ram:ID`.

## Relationship with EN 16931

- Factur-X 1.07 and earlier were aligned on **CII D16B**, the syntax
  explicitly referenced by EN 16931-3-3.
- Factur-X 1.08 is aligned on **CII D22B** but remains backward compatible:
  any document valid against the D16B schema is still valid against D22B.
  The reverse is **not** true.
- The **EN 16931 Schematron** artefacts published by CEN TC 434 still
  target D16B, and they validate semantics (amount consistency, business
  rules `BR-*`), not the D16B-vs-D22B XSD difference.

## Migration 1.07 → 1.08 (developer checklist)

For profiles `MINIMUM` to `EN 16931`, migration is essentially an XSD swap:

1. Replace `CrossIndustryInvoice_100pD16B.xsd` with the D22B version.
2. Bump the refreshed Schematron / XSLT artefacts from the FNFE-MPE
   release.
3. Add sub-line support if you produce kits / bundles.
4. Leave the XML generation code itself unchanged — namespaces and the
   document root are identical.

## Implementation notes for `einvoice-rs`

- `crates/facturx` exposes `FacturXProfile` variants that map 1:1 to the
  five profiles above. The method
  `FacturXProfile::specification_identifier` returns the URN verbatim.
- The CII XML generator (`FacturXSerializer::serialize_xml`) is the next
  implementation milestone. Target **CII D22B** from the start — it costs
  nothing versus D16B and keeps the library future-proof.
- The PDF/A-3 encapsulation should use a dedicated Rust PDF crate
  (`lopdf`, `printpdf`, or an XMP-aware wrapper). The XMP metadata, the
  `AFRelationship = Alternative` attribute and the exact
  `factur-x.xml` file name are **mandatory** for a conformant document.

## Upstream references

- FNFE-MPE Factur-X landing page — <https://fnfe-mpe.org/factur-x/>
- FNFE-MPE English page — <https://fnfe-mpe.org/factur-x/factur-x_en/>
- Factur-X 1.08 / ZUGFeRD 2.4 press release —
  <https://fnfe-mpe.org/wp-content/uploads/2025/12/2025-12-04_Factur-X_1.08_ZUGFeRD_2.4_Press_Release_EN.pdf>
- ConnectingEurope EN 16931 XSD artefacts —
  <https://github.com/ConnectingEurope/eInvoicing-EN16931>
- ZUGFeRD (FeRD) — <https://www.ferd-net.de/>
