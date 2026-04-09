//! Adaptateurs de transmission de factures.
//!
//! Deux canaux sont prévus :
//! - [`email`] — envoi SMTP via la crate `lettre` (facture en pièce jointe)
//! - [`chorus_pro`] — dépôt sur la plateforme publique française via l'API
//!   PISTE (OAuth2 client_credentials).
//!
//! Les adaptateurs retournent tous un [`DeliveryReceipt`] commun qui permet
//! de tracer dans la base (`invoice_events`) le canal utilisé, l'identifiant
//! externe et le statut renvoyé par le destinataire.

pub mod chorus_pro;
pub mod email;

pub use chorus_pro::ChorusProClient;
pub use email::EmailDelivery;

/// Accusé de dépôt commun à tous les canaux de transmission.
#[derive(Debug, Clone)]
pub struct DeliveryReceipt {
    /// Nom du canal (« email », « chorus-pro »…).
    pub provider: String,
    /// Identifiant externe attribué par le canal (Message-ID SMTP, ID Chorus…).
    pub external_id: String,
    /// Statut textuel renvoyé par le canal.
    pub status: String,
}
