//! Sérialiseur **Cross Industry Invoice (CII)** — UN/CEFACT D22B.
//!
//! Produit un document `rsm:CrossIndustryInvoice` conforme à la norme
//! UN/CEFACT utilisée par Factur-X 1.08 / ZUGFeRD 2.4. L'écriture se fait
//! via une pipeline explicite `quick_xml::Writer` car CII impose un ordre
//! strict des éléments que les dériveurs serde ne savent pas garantir.
//!
//! Voir [`docs/references/cii-uncefact.md`](../../../../docs/references/cii-uncefact.md)
//! pour un rappel de la structure générale et du mapping BT → XPath.

use einvoice_core::{Error, Invoice, LineItem, Party, Result};
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use rust_decimal::Decimal;

use crate::FacturXProfile;

/// Namespaces CII (UN/CEFACT D22B — inchangés depuis D16B).
pub(crate) mod ns {
    pub const RSM: &str = "urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100";
    pub const RAM: &str =
        "urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100";
    pub const QDT: &str = "urn:un:unece:uncefact:data:standard:QualifiedDataType:100";
    pub const UDT: &str = "urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100";
}

type XmlWriter = Writer<Vec<u8>>;
type XmlResult = std::result::Result<(), quick_xml::Error>;

/// Sérialise une [`Invoice`] en XML CII pour le profil Factur-X donné.
pub fn write_cross_industry_invoice(invoice: &Invoice, profile: FacturXProfile) -> Result<Vec<u8>> {
    let mut writer: XmlWriter = Writer::new_with_indent(Vec::new(), b' ', 2);
    write_document(&mut writer, invoice, profile)
        .map_err(|e| Error::serialization(format!("failed to write CII XML: {e}")))?;
    Ok(writer.into_inner())
}

fn write_document(w: &mut XmlWriter, invoice: &Invoice, profile: FacturXProfile) -> XmlResult {
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    let mut root = BytesStart::new("rsm:CrossIndustryInvoice");
    root.push_attribute(("xmlns:rsm", ns::RSM));
    root.push_attribute(("xmlns:ram", ns::RAM));
    root.push_attribute(("xmlns:qdt", ns::QDT));
    root.push_attribute(("xmlns:udt", ns::UDT));
    w.write_event(Event::Start(root))?;

    write_exchanged_document_context(w, profile)?;
    write_exchanged_document(w, invoice)?;
    write_supply_chain_trade_transaction(w, invoice)?;

    w.write_event(Event::End(BytesEnd::new("rsm:CrossIndustryInvoice")))?;
    Ok(())
}

// ---------- ExchangedDocumentContext ----------

fn write_exchanged_document_context(w: &mut XmlWriter, profile: FacturXProfile) -> XmlResult {
    open(w, "rsm:ExchangedDocumentContext")?;
    open(w, "ram:GuidelineSpecifiedDocumentContextParameter")?;
    element(w, "ram:ID", profile.specification_identifier())?;
    close(w, "ram:GuidelineSpecifiedDocumentContextParameter")?;
    close(w, "rsm:ExchangedDocumentContext")?;
    Ok(())
}

// ---------- ExchangedDocument ----------

fn write_exchanged_document(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    open(w, "rsm:ExchangedDocument")?;
    element(w, "ram:ID", &invoice.number)?;
    // UN/ECE 1001 : 380 = commercial invoice.
    element(w, "ram:TypeCode", "380")?;
    open(w, "ram:IssueDateTime")?;
    element_with_attrs(
        w,
        "udt:DateTimeString",
        &[("format", "102")],
        &format_date_compact(&invoice.issue_date),
    )?;
    close(w, "ram:IssueDateTime")?;

    if let Some(note) = invoice.notes.as_deref().filter(|s| !s.is_empty()) {
        open(w, "ram:IncludedNote")?;
        element(w, "ram:Content", note)?;
        close(w, "ram:IncludedNote")?;
    }

    close(w, "rsm:ExchangedDocument")?;
    Ok(())
}

// ---------- SupplyChainTradeTransaction ----------

fn write_supply_chain_trade_transaction(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    open(w, "rsm:SupplyChainTradeTransaction")?;

    for (idx, line) in invoice.lines.iter().enumerate() {
        write_trade_line_item(w, idx + 1, line)?;
    }

    write_header_trade_agreement(w, invoice)?;
    write_header_trade_delivery(w)?;
    write_header_trade_settlement(w, invoice)?;

    close(w, "rsm:SupplyChainTradeTransaction")?;
    Ok(())
}

fn write_trade_line_item(w: &mut XmlWriter, line_id: usize, line: &LineItem) -> XmlResult {
    open(w, "ram:IncludedSupplyChainTradeLineItem")?;

    open(w, "ram:AssociatedDocumentLineDocument")?;
    element(w, "ram:LineID", &line_id.to_string())?;
    close(w, "ram:AssociatedDocumentLineDocument")?;

    open(w, "ram:SpecifiedTradeProduct")?;
    element(w, "ram:Name", &line.description)?;
    close(w, "ram:SpecifiedTradeProduct")?;

    open(w, "ram:SpecifiedLineTradeAgreement")?;
    open(w, "ram:NetPriceProductTradePrice")?;
    element(w, "ram:ChargeAmount", &format_amount(line.unit_price))?;
    close(w, "ram:NetPriceProductTradePrice")?;
    close(w, "ram:SpecifiedLineTradeAgreement")?;

    open(w, "ram:SpecifiedLineTradeDelivery")?;
    // C62 = "one" (UN/ECE Rec 20) — unité par défaut tant que le modèle
    // `LineItem` ne porte pas d'unité explicite.
    element_with_attrs(
        w,
        "ram:BilledQuantity",
        &[("unitCode", "C62")],
        &line.quantity.to_string(),
    )?;
    close(w, "ram:SpecifiedLineTradeDelivery")?;

    open(w, "ram:SpecifiedLineTradeSettlement")?;
    open(w, "ram:ApplicableTradeTax")?;
    element(w, "ram:TypeCode", "VAT")?;
    // EN 16931 / UNCL5305 : S = standard rate.
    element(w, "ram:CategoryCode", "S")?;
    element(
        w,
        "ram:RateApplicablePercent",
        &format_percent(line.tax_rate),
    )?;
    close(w, "ram:ApplicableTradeTax")?;
    open(w, "ram:SpecifiedTradeSettlementLineMonetarySummation")?;
    element(w, "ram:LineTotalAmount", &format_amount(line.line_total()))?;
    close(w, "ram:SpecifiedTradeSettlementLineMonetarySummation")?;
    close(w, "ram:SpecifiedLineTradeSettlement")?;

    close(w, "ram:IncludedSupplyChainTradeLineItem")?;
    Ok(())
}

fn write_header_trade_agreement(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    open(w, "ram:ApplicableHeaderTradeAgreement")?;
    write_party(w, "ram:SellerTradeParty", &invoice.seller)?;
    write_party(w, "ram:BuyerTradeParty", &invoice.buyer)?;
    close(w, "ram:ApplicableHeaderTradeAgreement")?;
    Ok(())
}

fn write_party(w: &mut XmlWriter, element_name: &str, party: &Party) -> XmlResult {
    open(w, element_name)?;
    element(w, "ram:Name", &party.name)?;

    open(w, "ram:PostalTradeAddress")?;
    element(w, "ram:PostcodeCode", &party.address.postal_code)?;
    element(w, "ram:LineOne", &party.address.street)?;
    element(w, "ram:CityName", &party.address.city)?;
    element(w, "ram:CountryID", &party.address.country_code)?;
    close(w, "ram:PostalTradeAddress")?;

    if let Some(email) = party.email.as_deref().filter(|s| !s.is_empty()) {
        open(w, "ram:URIUniversalCommunication")?;
        element_with_attrs(w, "ram:URIID", &[("schemeID", "EM")], email)?;
        close(w, "ram:URIUniversalCommunication")?;
    }

    if let Some(vat) = party.vat_number.as_deref().filter(|s| !s.is_empty()) {
        open(w, "ram:SpecifiedTaxRegistration")?;
        element_with_attrs(w, "ram:ID", &[("schemeID", "VA")], vat)?;
        close(w, "ram:SpecifiedTaxRegistration")?;
    }

    close(w, element_name)?;
    Ok(())
}

fn write_header_trade_delivery(w: &mut XmlWriter) -> XmlResult {
    // Cardinalité 1..1 dans CII, mais tous les enfants sont optionnels pour
    // le profil EN 16931 minimum. On émet un élément vide.
    w.write_event(Event::Empty(BytesStart::new(
        "ram:ApplicableHeaderTradeDelivery",
    )))
}

fn write_header_trade_settlement(w: &mut XmlWriter, invoice: &Invoice) -> XmlResult {
    open(w, "ram:ApplicableHeaderTradeSettlement")?;
    element(w, "ram:InvoiceCurrencyCode", &invoice.currency)?;

    for group in vat_breakdown(invoice) {
        open(w, "ram:ApplicableTradeTax")?;
        element(w, "ram:CalculatedAmount", &format_amount(group.tax))?;
        element(w, "ram:TypeCode", "VAT")?;
        element(w, "ram:BasisAmount", &format_amount(group.basis))?;
        element(w, "ram:CategoryCode", "S")?;
        element(w, "ram:RateApplicablePercent", &format_percent(group.rate))?;
        close(w, "ram:ApplicableTradeTax")?;
    }

    if let Some(due) = invoice.due_date {
        open(w, "ram:SpecifiedTradePaymentTerms")?;
        open(w, "ram:DueDateDateTime")?;
        element_with_attrs(
            w,
            "udt:DateTimeString",
            &[("format", "102")],
            &format_date_compact(&due),
        )?;
        close(w, "ram:DueDateDateTime")?;
        close(w, "ram:SpecifiedTradePaymentTerms")?;
    }

    let subtotal = invoice.subtotal();
    let tax_total = invoice.tax_total();
    let grand_total = invoice.total();

    open(w, "ram:SpecifiedTradeSettlementHeaderMonetarySummation")?;
    element(w, "ram:LineTotalAmount", &format_amount(subtotal))?;
    element(w, "ram:TaxBasisTotalAmount", &format_amount(subtotal))?;
    element_with_attrs(
        w,
        "ram:TaxTotalAmount",
        &[("currencyID", invoice.currency.as_str())],
        &format_amount(tax_total),
    )?;
    element(w, "ram:GrandTotalAmount", &format_amount(grand_total))?;
    element(w, "ram:DuePayableAmount", &format_amount(grand_total))?;
    close(w, "ram:SpecifiedTradeSettlementHeaderMonetarySummation")?;

    close(w, "ram:ApplicableHeaderTradeSettlement")?;
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

fn format_date_compact(date: &chrono::NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
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

    fn write(invoice: &Invoice, profile: FacturXProfile) -> String {
        let bytes = write_cross_industry_invoice(invoice, profile).expect("serialize");
        String::from_utf8(bytes).expect("valid utf-8")
    }

    #[test]
    fn declares_xml_header_and_namespaces() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains(
            "xmlns:rsm=\"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100\""
        ));
        assert!(xml.contains("xmlns:ram=\"urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100\""));
        assert!(xml.contains("xmlns:qdt="));
        assert!(xml.contains("xmlns:udt="));
    }

    #[test]
    fn embeds_profile_identifier() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.contains("<ram:ID>urn:cen.eu:en16931:2017</ram:ID>"));
        let xml_min = write(&sample_invoice(), FacturXProfile::Minimum);
        assert!(xml_min.contains("<ram:ID>urn:factur-x.eu:1p0:minimum</ram:ID>"));
    }

    #[test]
    fn writes_header_metadata() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.contains("<ram:ID>FAC-2026-0001</ram:ID>"));
        assert!(xml.contains("<ram:TypeCode>380</ram:TypeCode>"));
        assert!(xml.contains("<udt:DateTimeString format=\"102\">20260409</udt:DateTimeString>"));
        assert!(xml.contains("<ram:Content>Paiement à 30 jours</ram:Content>"));
    }

    #[test]
    fn writes_seller_and_buyer() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.contains("<ram:SellerTradeParty>"));
        assert!(xml.contains("<ram:Name>Acme SAS</ram:Name>"));
        assert!(xml.contains("<ram:ID schemeID=\"VA\">FR12345678900</ram:ID>"));
        assert!(xml.contains("<ram:BuyerTradeParty>"));
        assert!(xml.contains("<ram:Name>Client &amp; Co</ram:Name>"));
        assert!(xml.contains("<ram:CountryID>FR</ram:CountryID>"));
        assert!(xml.contains("<ram:URIID schemeID=\"EM\">billing@acme.test</ram:URIID>"));
    }

    #[test]
    fn writes_lines_and_line_totals() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.contains("<ram:LineID>1</ram:LineID>"));
        assert!(xml.contains("<ram:LineID>2</ram:LineID>"));
        assert!(xml.contains("<ram:Name>Prestation de conseil</ram:Name>"));
        assert!(xml.contains("<ram:BilledQuantity unitCode=\"C62\">2</ram:BilledQuantity>"));
        // Line 1 net = 2 * 100 = 200.00
        assert!(xml.contains("<ram:LineTotalAmount>200.00</ram:LineTotalAmount>"));
        // Line 2 net = 1 * 50 = 50.00
        assert!(xml.contains("<ram:LineTotalAmount>50.00</ram:LineTotalAmount>"));
    }

    #[test]
    fn writes_vat_breakdown_and_totals() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        // VAT : base 250.00 @ 20% = 50.00
        assert!(xml.contains("<ram:CalculatedAmount>50.00</ram:CalculatedAmount>"));
        assert!(xml.contains("<ram:BasisAmount>250.00</ram:BasisAmount>"));
        assert!(xml.contains("<ram:RateApplicablePercent>20.00</ram:RateApplicablePercent>"));
        // Totals
        assert!(xml.contains("<ram:TaxBasisTotalAmount>250.00</ram:TaxBasisTotalAmount>"));
        assert!(xml.contains("<ram:TaxTotalAmount currencyID=\"EUR\">50.00</ram:TaxTotalAmount>"));
        assert!(xml.contains("<ram:GrandTotalAmount>300.00</ram:GrandTotalAmount>"));
        assert!(xml.contains("<ram:DuePayableAmount>300.00</ram:DuePayableAmount>"));
    }

    #[test]
    fn writes_payment_terms_when_due_date_set() {
        let xml = write(&sample_invoice(), FacturXProfile::En16931);
        assert!(xml.contains("<ram:SpecifiedTradePaymentTerms>"));
        assert!(xml.contains("<udt:DateTimeString format=\"102\">20260509</udt:DateTimeString>"));
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
        let xml = write(&invoice, FacturXProfile::En16931);
        // Base à 5,5% = 150.00, TVA = 8.25
        assert!(xml.contains("<ram:BasisAmount>150.00</ram:BasisAmount>"));
        assert!(xml.contains("<ram:CalculatedAmount>8.25</ram:CalculatedAmount>"));
        assert!(xml.contains("<ram:RateApplicablePercent>5.50</ram:RateApplicablePercent>"));
        // Toujours présent le groupe 20%
        assert!(xml.contains("<ram:RateApplicablePercent>20.00</ram:RateApplicablePercent>"));
    }

    #[test]
    fn document_is_well_formed_xml() {
        use quick_xml::events::Event as RxEvent;
        use quick_xml::Reader;

        let xml = write(&sample_invoice(), FacturXProfile::En16931);
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
}
