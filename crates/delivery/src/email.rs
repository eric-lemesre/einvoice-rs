//! Adaptateur de transmission par email (SMTP via `lettre`).

use einvoice_core::{Error, Invoice, Result};

use crate::DeliveryReceipt;

/// Client SMTP configurable pour envoyer une facture en pièce jointe.
#[derive(Debug, Clone)]
pub struct EmailDelivery {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub from: String,
}

impl EmailDelivery {
    pub fn new(smtp_host: impl Into<String>, smtp_port: u16, from: impl Into<String>) -> Self {
        Self {
            smtp_host: smtp_host.into(),
            smtp_port,
            from: from.into(),
        }
    }

    /// Envoie la facture au destinataire (`invoice.buyer.email`) avec
    /// `attachment` en pièce jointe (PDF Factur-X, XML UBL…).
    pub async fn send(
        &self,
        invoice: &Invoice,
        _attachment: Vec<u8>,
        _attachment_name: &str,
        _content_type: &str,
    ) -> Result<DeliveryReceipt> {
        let _to = invoice
            .buyer
            .email
            .as_deref()
            .ok_or_else(|| Error::delivery("buyer has no email address"))?;

        // TODO : construire un `lettre::Message` multipart avec la pièce
        // jointe et l'envoyer via `AsyncSmtpTransport::<Tokio1Executor>::relay`.
        tracing::debug!(
            invoice = %invoice.number,
            host = %self.smtp_host,
            "email delivery stub"
        );
        Err(Error::delivery("email delivery not yet implemented"))
    }
}
