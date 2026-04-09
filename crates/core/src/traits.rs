//! Traits partagés : sérialiseurs et validateurs de factures.
//!
//! Implémentés dans les crates `einvoice-facturx` et `einvoice-ubl` pour la
//! production de formats structurés, et potentiellement dans `einvoice-api`
//! pour des validateurs métier spécifiques (Chorus Pro, PEPPOL…).

use crate::{error::Result, model::Invoice};

/// Un sérialiseur transforme une `Invoice` en représentation binaire
/// (XML, JSON, PDF/A-3…).
pub trait InvoiceSerializer {
    /// Produit la représentation sérialisée.
    fn serialize(&self, invoice: &Invoice) -> Result<Vec<u8>>;

    /// Type MIME du contenu produit.
    fn content_type(&self) -> &'static str;

    /// Nom humain du format (« Factur-X », « UBL 2.1 »…).
    fn format_name(&self) -> &'static str;
}

/// Un validateur vérifie qu'une facture satisfait un ensemble de règles
/// (EN 16931, Chorus Pro, PEPPOL BIS…).
pub trait InvoiceValidator {
    /// Retourne `Ok(())` si la facture est valide, une `Error::Validation`
    /// sinon.
    fn validate(&self, invoice: &Invoice) -> Result<()>;
}
