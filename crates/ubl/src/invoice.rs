//! Sérialiseur **UBL 2.1 Invoice** (OASIS).
//!
//! Produit un élément racine `<Invoice>` conforme à
//! [`docs/references/ubl-2.1.md`](../../../../docs/references/ubl-2.1.md),
//! lié à **EN 16931** via `cbc:CustomizationID` et, au choix, au profil
//! **PEPPOL BIS Billing 3.0**.
//!
//! L'écriture passe par une pipeline `quick_xml::Writer` explicite pour
//! garder le contrôle de l'ordre canonique des éléments imposé à la fois
//! par le XSD OASIS et par les Schematron PEPPOL.

use einvoice_core::{Error, Invoice, LineItem, Party, Result};
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use rust_decimal::Decimal;

use crate::UblProfile;

/// Namespaces OASIS UBL 2.1.
pub(crate) mod ns {
    pub const UBL_INVOICE: &str = "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2";
    pub const CBC: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
    pub const CAC: &str =
        "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
}

type XmlWriter = Writer<Vec<u8>>;
type XmlResult = std::result::Result<(), quick_xml::Error>;

/// Sérialise une [`Invoice`] en XML UBL 2.1 pour le profil donné.
pub fn write_invoice(invoice: &Invoice, profile: UblProfile) -> Result<Vec<u8>> {
    let mut writer: XmlWriter = Writer::new_with_indent(Vec::new(), b' ', 2);
    write_document(&mut writer, invoice, profile)
        .map_err(|e| Error::serialization(format!("failed to write UBL XML: {e}")))?;
    Ok(writer.into_inner())
}

fn write_document(w: &mut XmlWriter, invoice: &Invoice, profile: UblProfile) -> XmlResult {
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    let mut root = BytesStart::new("Invoice");
    root.push_attribute(("xmlns", ns::UBL_INVOICE));
    root.push_attribute(("xmlns:cbc", ns::CBC));
    root.push_attribute(("xmlns:cac", ns::CAC));
    w.write_event(Event::Start(root))?;

    write_header(w, invoice, profile)?;
    write_supplier_party(w, &invoice.seller)?;
    write_customer_party(w, &invoice.buyer)?;
    write_tax_total(w, invoice)?;
    write_legal_monetary_total(w, invoice)?;
    for (idx, line) in invoice.lines.iter().enumerate() {
        write_invoice_line(w, idx + 1, line, &invoice.currency)?;
    }

    w.write_event(Event::End(BytesEnd::new("Invoice")))?;
    Ok(())
}

// ---------- Header (scalar fields) ----------

fn write_header(w: &mut XmlWriter, invoice: &Invoice, profile: UblProfile) -> XmlResult {
    element(w, "cbc:CustomizationID", profile.customization_id())?;
    element(w, "cbc:ProfileID", profile.profile_id())?;
    element(w, "cbc:ID", &invoice.number)?;
    element(w, "cbc:IssueDate", &format_date(invoice.issue_date))?;
    if let Some(due) = invoice.due_date {
        element(w, "cbc:DueDate", &format_date(due))?;
    }
    // UN/ECE 1001 : 380 = commercial invoice.
    element(w, "cbc:InvoiceTypeCode", "380")?;
    if let Some(note) = invoice.notes.as_deref().filter(|s| !s.is_empty()) {
        element(w, "cbc:Note", note)?;
    }
    element(w, "cbc:DocumentCurrencyCode", &invoice.currency)?;
    Ok(())
}

// ---------- Parties ----------

fn write_supplier_party(w: &mut XmlWriter, party: &Party) -> XmlResult {
    open(w, "cac:AccountingSupplierParty")?;
    write_party(w, party)?;
    close(w, "cac:AccountingSupplierParty")?;
    Ok(())
}

fn write_customer_party(w: &mut XmlWriter, party: &Party) -> XmlResult {
    open(w, "cac:AccountingCustomerParty")?;
    write_party(w, party)?;
    close(w, "cac:AccountingCustomerParty")?;
    Ok(())
}

fn write_party(w: &mut XmlWriter, party: &Party) -> XmlResult {
    open(w, "cac:Party")?;

    open(w, "cac:PartyName")?;
    element(w, "cbc:Name", &party.name)?;
    close(w, "cac:PartyName")?;

    open(w, "cac:PostalAddress")?;
    element(w, "cbc:StreetName", &party.address.street)?;
    element(w, "cbc:CityName", &party.address.city)?;
    element(w, "cbc:PostalZone", &party.address.postal_code)?;
    open(w, "cac:Country")?;
    element(w, "cbc:IdentificationCode", &party.address.country_code)?;
    close(w, "cac:Country")?;
    close(w, "cac:PostalAddress")?;

    if let Some(vat) = party.vat_number.as_deref().filter(|s| !s.is_empty()) {
        open(w, "cac:PartyTaxScheme")?;
        element(w, "cbc:CompanyID", vat)?;
        open(w, "cac:TaxScheme")?;
        element(w, "cbc:ID", "VAT")?;
        close(w, "cac:TaxScheme")?;
        close(w, "cac:PartyTaxScheme")?;
    }

    open(w, "cac:PartyLegalEntity")?;
    element(w, "cbc:RegistrationName", &party.name)?;
    if let Some(legal) = party.legal_id.as_deref().filter(|s| !s.is_empty()) {
        element(w, "cbc:CompanyID", legal)?;
    }
    close(w, "cac:PartyLegalEntity")?;

    if let Some(email) = party.email.as_deref().filter(|s| !s.is_empty()) {
        open(w, "cac:Contact")?;
        element(w, "cbc:ElectronicMail", email)?;
        close(w, "cac:Contact")?;
    }

    close(w, "cac:Party")?;
    Ok(())
}

// ---------- Tax totals ----------

fn write_tax_total(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    let currency = invoice.currency.as_str();
    open(w, "cac:TaxTotal")?;
    element_with_attrs(
        w,
        "cbc:TaxAmount",
        &[("currencyID", currency)],
        &format_amount(invoice.tax_total()),
    )?;
    for group in vat_breakdown(invoice) {
        open(w, "cac:TaxSubtotal")?;
        element_with_attrs(
            w,
            "cbc:TaxableAmount",
            &[("currencyID", currency)],
            &format_amount(group.basis),
        )?;
        element_with_attrs(
            w,
            "cbc:TaxAmount",
            &[("currencyID", currency)],
            &format_amount(group.tax),
        )?;
        open(w, "cac:TaxCategory")?;
        // EN 16931 / UNCL5305 : S = standard rate.
        element(w, "cbc:ID", "S")?;
        element(w, "cbc:Percent", &format_percent(group.rate))?;
        open(w, "cac:TaxScheme")?;
        element(w, "cbc:ID", "VAT")?;
        close(w, "cac:TaxScheme")?;
        close(w, "cac:TaxCategory")?;
        close(w, "cac:TaxSubtotal")?;
    }
    close(w, "cac:TaxTotal")?;
    Ok(())
}

// ---------- Legal monetary total ----------

fn write_legal_monetary_total(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    let currency = invoice.currency.as_str();
    let subtotal = invoice.subtotal();
    let grand_total = invoice.total();

    open(w, "cac:LegalMonetaryTotal")?;
    element_with_attrs(
        w,
        "cbc:LineExtensionAmount",
        &[("currencyID", currency)],
        &format_amount(subtotal),
    )?;
    element_with_attrs(
        w,
        "cbc:TaxExclusiveAmount",
        &[("currencyID", currency)],
        &format_amount(subtotal),
    )?;
    element_with_attrs(
        w,
        "cbc:TaxInclusiveAmount",
        &[("currencyID", currency)],
        &format_amount(grand_total),
    )?;
    element_with_attrs(
        w,
        "cbc:PayableAmount",
        &[("currencyID", currency)],
        &format_amount(grand_total),
    )?;
    close(w, "cac:LegalMonetaryTotal")?;
    Ok(())
}

// ---------- Invoice lines ----------

fn write_invoice_line(
    w: &mut XmlWriter,
    line_id: usize,
    line: &LineItem,
    currency: &str,
) -> XmlResult {
    open(w, "cac:InvoiceLine")?;
    element(w, "cbc:ID", &line_id.to_string())?;
    // C62 = "one" (UN/ECE Rec 20) — unité par défaut tant que le modèle
    // `LineItem` ne porte pas d'unité explicite.
    element_with_attrs(
        w,
        "cbc:InvoicedQuantity",
        &[("unitCode", "C62")],
        &line.quantity.to_string(),
    )?;
    element_with_attrs(
        w,
        "cbc:LineExtensionAmount",
        &[("currencyID", currency)],
        &format_amount(line.line_total()),
    )?;

    open(w, "cac:Item")?;
    element(w, "cbc:Name", &line.description)?;
    open(w, "cac:ClassifiedTaxCategory")?;
    element(w, "cbc:ID", "S")?;
    element(w, "cbc:Percent", &format_percent(line.tax_rate))?;
    open(w, "cac:TaxScheme")?;
    element(w, "cbc:ID", "VAT")?;
    close(w, "cac:TaxScheme")?;
    close(w, "cac:ClassifiedTaxCategory")?;
    close(w, "cac:Item")?;

    open(w, "cac:Price")?;
    element_with_attrs(
        w,
        "cbc:PriceAmount",
        &[("currencyID", currency)],
        &format_amount(line.unit_price),
    )?;
    close(w, "cac:Price")?;

    close(w, "cac:InvoiceLine")?;
    Ok(())
}

// ---------- helpers ----------

fn open(w: &mut XmlWriter, name: &str) -> XmlResult {
    w.write_event(Event::Start(BytesStart::new(name)))
}

fn close(w: &mut XmlWriter, name: &str) -> XmlResult {
    w.write_event(Event::End(BytesEnd::new(name)))
}

fn element(w: &mut XmlWriter, name: &str, text: &str) -> XmlResult {
    w.write_event(Event::Start(BytesStart::new(name)))?;
    w.write_event(Event::Text(BytesText::new(text)))?;
    w.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

fn element_with_attrs(
    w: &mut XmlWriter,
    name: &str,
    attrs: &[(&str, &str)],
    text: &str,
) -> XmlResult {
    let mut start = BytesStart::new(name);
    for (k, v) in attrs {
        start.push_attribute((*k, *v));
    }
    w.write_event(Event::Start(start))?;
    w.write_event(Event::Text(BytesText::new(text)))?;
    w.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

fn format_amount(amount: Decimal) -> String {
    format!("{amount:.2}")
}

fn format_percent(rate_fraction: Decimal) -> String {
    format!("{:.2}", rate_fraction * Decimal::from(100))
}

fn format_date(date: chrono::NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

// ---------- VAT breakdown ----------

struct VatGroup {
    rate: Decimal,
    basis: Decimal,
    tax: Decimal,
}

fn vat_breakdown(invoice: &Invoice) -> Vec<VatGroup> {
    let mut groups: Vec<VatGroup> = Vec::new();
    for line in &invoice.lines {
        if let Some(g) = groups.iter_mut().find(|g| g.rate == line.tax_rate) {
            g.basis += line.line_total();
            g.tax += line.tax_amount();
        } else {
            groups.push(VatGroup {
                rate: line.tax_rate,
                basis: line.line_total(),
                tax: line.tax_amount(),
            });
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use einvoice_core::{Address, InvoiceStatus, LineItem, Party};
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    fn sample_invoice() -> Invoice {
        Invoice {
            id: Uuid::nil(),
            number: "FAC-2026-0001".into(),
            issue_date: NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(),
            due_date: Some(NaiveDate::from_ymd_opt(2026, 5, 9).unwrap()),
            currency: "EUR".into(),
            seller: Party {
                name: "Acme SAS".into(),
                legal_id: Some("12345678900012".into()),
                vat_number: Some("FR12345678900".into()),
                address: Address {
                    street: "1 rue de la Paix".into(),
                    city: "Paris".into(),
                    postal_code: "75001".into(),
                    country_code: "FR".into(),
                },
                email: Some("billing@acme.test".into()),
            },
            buyer: Party {
                name: "Client & Co".into(),
                legal_id: Some("98765432100021".into()),
                vat_number: Some("FR98765432100".into()),
                address: Address {
                    street: "2 avenue Foch".into(),
                    city: "Lyon".into(),
                    postal_code: "69006".into(),
                    country_code: "FR".into(),
                },
                email: None,
            },
            lines: vec![
                LineItem {
                    description: "Prestation de conseil".into(),
                    quantity: dec!(2),
                    unit_price: dec!(100),
                    tax_rate: dec!(0.20),
                },
                LineItem {
                    description: "Licence logicielle".into(),
                    quantity: dec!(1),
                    unit_price: dec!(50),
                    tax_rate: dec!(0.20),
                },
            ],
            notes: Some("Paiement à 30 jours".into()),
            status: InvoiceStatus::Issued,
        }
    }

    fn write(invoice: &Invoice, profile: UblProfile) -> String {
        let bytes = write_invoice(invoice, profile).expect("serialize");
        String::from_utf8(bytes).expect("valid utf-8")
    }

    #[test]
    fn declares_xml_header_and_namespaces() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:Invoice-2\""));
        assert!(xml.contains(
            "xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\""
        ));
        assert!(xml.contains(
            "xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\""
        ));
    }

    #[test]
    fn embeds_profile_identifiers() {
        let xml_peppol = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml_peppol.contains("<cbc:CustomizationID>urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0</cbc:CustomizationID>"));
        assert!(xml_peppol.contains(
            "<cbc:ProfileID>urn:fdc:peppol.eu:2017:poacc:billing:01:1.0</cbc:ProfileID>"
        ));

        let xml_cpro = write(&sample_invoice(), UblProfile::ChorusPro);
        assert!(
            xml_cpro.contains("<cbc:CustomizationID>urn:cen.eu:en16931:2017</cbc:CustomizationID>")
        );
    }

    #[test]
    fn writes_header_metadata() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.contains("<cbc:ID>FAC-2026-0001</cbc:ID>"));
        assert!(xml.contains("<cbc:IssueDate>2026-04-09</cbc:IssueDate>"));
        assert!(xml.contains("<cbc:DueDate>2026-05-09</cbc:DueDate>"));
        assert!(xml.contains("<cbc:InvoiceTypeCode>380</cbc:InvoiceTypeCode>"));
        assert!(xml.contains("<cbc:Note>Paiement à 30 jours</cbc:Note>"));
        assert!(xml.contains("<cbc:DocumentCurrencyCode>EUR</cbc:DocumentCurrencyCode>"));
    }

    #[test]
    fn writes_supplier_and_customer_parties() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.contains("<cac:AccountingSupplierParty>"));
        assert!(xml.contains("<cbc:Name>Acme SAS</cbc:Name>"));
        assert!(xml.contains("<cbc:StreetName>1 rue de la Paix</cbc:StreetName>"));
        assert!(xml.contains("<cbc:PostalZone>75001</cbc:PostalZone>"));
        assert!(xml.contains("<cbc:IdentificationCode>FR</cbc:IdentificationCode>"));
        assert!(xml.contains("<cbc:CompanyID>FR12345678900</cbc:CompanyID>"));
        assert!(xml.contains("<cbc:CompanyID>12345678900012</cbc:CompanyID>"));
        assert!(xml.contains("<cbc:ElectronicMail>billing@acme.test</cbc:ElectronicMail>"));

        assert!(xml.contains("<cac:AccountingCustomerParty>"));
        assert!(xml.contains("<cbc:Name>Client &amp; Co</cbc:Name>"));
        assert!(xml.contains("<cbc:CompanyID>FR98765432100</cbc:CompanyID>"));
    }

    #[test]
    fn writes_tax_total_and_subtotal() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.contains("<cac:TaxTotal>"));
        assert!(xml.contains("<cbc:TaxAmount currencyID=\"EUR\">50.00</cbc:TaxAmount>"));
        assert!(xml.contains("<cbc:TaxableAmount currencyID=\"EUR\">250.00</cbc:TaxableAmount>"));
        assert!(xml.contains("<cbc:Percent>20.00</cbc:Percent>"));
        // TaxScheme VAT présent au moins deux fois (subtotal + ligne)
        assert!(xml.matches("<cbc:ID>VAT</cbc:ID>").count() >= 2);
    }

    #[test]
    fn writes_legal_monetary_total() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.contains("<cac:LegalMonetaryTotal>"));
        assert!(xml.contains(
            "<cbc:LineExtensionAmount currencyID=\"EUR\">250.00</cbc:LineExtensionAmount>"
        ));
        assert!(xml.contains(
            "<cbc:TaxExclusiveAmount currencyID=\"EUR\">250.00</cbc:TaxExclusiveAmount>"
        ));
        assert!(xml.contains(
            "<cbc:TaxInclusiveAmount currencyID=\"EUR\">300.00</cbc:TaxInclusiveAmount>"
        ));
        assert!(xml.contains("<cbc:PayableAmount currencyID=\"EUR\">300.00</cbc:PayableAmount>"));
    }

    #[test]
    fn writes_invoice_lines() {
        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        assert!(xml.contains("<cac:InvoiceLine>"));
        assert!(xml.contains("<cbc:InvoicedQuantity unitCode=\"C62\">2</cbc:InvoicedQuantity>"));
        assert!(xml.contains(
            "<cbc:LineExtensionAmount currencyID=\"EUR\">200.00</cbc:LineExtensionAmount>"
        ));
        assert!(xml.contains("<cbc:Name>Prestation de conseil</cbc:Name>"));
        assert!(xml.contains("<cbc:PriceAmount currencyID=\"EUR\">100.00</cbc:PriceAmount>"));
        assert!(xml.contains("<cbc:Name>Licence logicielle</cbc:Name>"));
        assert!(xml.contains("<cbc:PriceAmount currencyID=\"EUR\">50.00</cbc:PriceAmount>"));
    }

    #[test]
    fn groups_multiple_vat_rates() {
        let mut invoice = sample_invoice();
        invoice.lines.push(LineItem {
            description: "Livre".into(),
            quantity: dec!(10),
            unit_price: dec!(15),
            tax_rate: dec!(0.055),
        });
        let xml = write(&invoice, UblProfile::PeppolBis3);
        // Base à 5,5% = 150.00, TVA = 8.25
        assert!(xml.contains("<cbc:TaxableAmount currencyID=\"EUR\">150.00</cbc:TaxableAmount>"));
        assert!(xml.contains("<cbc:TaxAmount currencyID=\"EUR\">8.25</cbc:TaxAmount>"));
        assert!(xml.contains("<cbc:Percent>5.50</cbc:Percent>"));
        // Toujours présent le groupe 20%
        assert!(xml.contains("<cbc:Percent>20.00</cbc:Percent>"));
        // Total TVA = 50.00 + 8.25 = 58.25
        assert!(xml.contains("<cbc:TaxAmount currencyID=\"EUR\">58.25</cbc:TaxAmount>"));
    }

    #[test]
    fn document_is_well_formed_xml() {
        use quick_xml::events::Event as RxEvent;
        use quick_xml::Reader;

        let xml = write(&sample_invoice(), UblProfile::PeppolBis3);
        let mut reader = Reader::from_str(&xml);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        loop {
            let event = reader.read_event_into(&mut buf).expect("well-formed xml");
            if matches!(event, RxEvent::Eof) {
                break;
            }
            buf.clear();
        }
    }

    #[test]
    fn omits_optional_fields_when_missing() {
        let mut invoice = sample_invoice();
        invoice.due_date = None;
        invoice.notes = None;
        invoice.seller.email = None;
        invoice.seller.legal_id = None;
        invoice.seller.vat_number = None;

        let xml = write(&invoice, UblProfile::PeppolBis3);
        assert!(!xml.contains("<cbc:DueDate>"));
        assert!(!xml.contains("<cbc:Note>"));
        // La ligne 1 (seller) ne doit plus avoir de PartyTaxScheme ni Contact
        let supplier_section = &xml[xml.find("<cac:AccountingSupplierParty>").unwrap()
            ..xml.find("</cac:AccountingSupplierParty>").unwrap()];
        assert!(!supplier_section.contains("<cac:PartyTaxScheme>"));
        assert!(!supplier_section.contains("<cac:Contact>"));
        // Mais RegistrationName reste présent (mandatory)
        assert!(supplier_section.contains("<cbc:RegistrationName>Acme SAS</cbc:RegistrationName>"));
    }
}
