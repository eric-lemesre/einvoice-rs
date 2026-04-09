//! Modèle domaine : `Invoice`, `Party`, `Address`, `LineItem`.
//!
//! Les types sont volontairement neutres vis-à-vis du format (Factur-X, UBL…).
//! Les sérialiseurs spécifiques consomment ce modèle et produisent la
//! représentation cible.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Statut du cycle de vie d'une facture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Sent,
    Received,
    Accepted,
    Rejected,
    Paid,
    Cancelled,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Issued => "issued",
            Self::Sent => "sent",
            Self::Received => "received",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Paid => "paid",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Facture — racine du modèle domaine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub number: String,
    pub issue_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    /// Code devise ISO 4217 (ex. `"EUR"`).
    pub currency: String,
    pub seller: Party,
    pub buyer: Party,
    pub lines: Vec<LineItem>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default = "default_status")]
    pub status: InvoiceStatus,
}

fn default_status() -> InvoiceStatus {
    InvoiceStatus::Draft
}

impl Invoice {
    /// Total hors taxes (somme des lignes).
    pub fn subtotal(&self) -> Decimal {
        self.lines.iter().map(LineItem::line_total).sum()
    }

    /// Total des taxes (TVA) toutes lignes confondues.
    pub fn tax_total(&self) -> Decimal {
        self.lines.iter().map(LineItem::tax_amount).sum()
    }

    /// Total TTC.
    pub fn total(&self) -> Decimal {
        self.subtotal() + self.tax_total()
    }
}

/// Partie prenante : émetteur ou destinataire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub name: String,
    /// Identifiant légal (SIRET pour la France).
    #[serde(default)]
    pub legal_id: Option<String>,
    /// Numéro de TVA intracommunautaire.
    #[serde(default)]
    pub vat_number: Option<String>,
    pub address: Address,
    #[serde(default)]
    pub email: Option<String>,
}

/// Adresse postale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    /// Code pays ISO 3166-1 alpha-2 (ex. `"FR"`).
    pub country_code: String,
}

/// Ligne de facture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    /// Taux de TVA exprimé en fraction décimale (ex. `0.20` pour 20 %).
    pub tax_rate: Decimal,
}

impl LineItem {
    /// Montant hors taxes de la ligne.
    pub fn line_total(&self) -> Decimal {
        self.quantity * self.unit_price
    }

    /// Montant de TVA de la ligne.
    pub fn tax_amount(&self) -> Decimal {
        self.line_total() * self.tax_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn sample_line(qty: Decimal, price: Decimal, rate: Decimal) -> LineItem {
        LineItem {
            description: "Prestation".into(),
            quantity: qty,
            unit_price: price,
            tax_rate: rate,
        }
    }

    #[test]
    fn invoice_totals_are_correct() {
        let invoice = Invoice {
            id: Uuid::nil(),
            number: "FAC-2026-0001".into(),
            issue_date: NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(),
            due_date: None,
            currency: "EUR".into(),
            seller: Party {
                name: "Acme".into(),
                legal_id: None,
                vat_number: None,
                address: Address {
                    street: "1 rue Test".into(),
                    city: "Paris".into(),
                    postal_code: "75000".into(),
                    country_code: "FR".into(),
                },
                email: None,
            },
            buyer: Party {
                name: "Client".into(),
                legal_id: None,
                vat_number: None,
                address: Address {
                    street: "2 rue Test".into(),
                    city: "Lyon".into(),
                    postal_code: "69000".into(),
                    country_code: "FR".into(),
                },
                email: None,
            },
            lines: vec![
                sample_line(dec!(2), dec!(100), dec!(0.20)),
                sample_line(dec!(1), dec!(50), dec!(0.20)),
            ],
            notes: None,
            status: InvoiceStatus::Draft,
        };

        assert_eq!(invoice.subtotal(), dec!(250));
        assert_eq!(invoice.tax_total(), dec!(50));
        assert_eq!(invoice.total(), dec!(300));
    }
}
