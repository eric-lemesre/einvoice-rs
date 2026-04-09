//! Type d'erreur centralisé utilisé par tous les crates du workspace.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// Données de facture invalides par rapport aux règles métier
    /// (EN 16931, Chorus Pro, PEPPOL…).
    #[error("validation: {0}")]
    Validation(String),

    /// Erreur de (dé)sérialisation d'un format structuré
    /// (Factur-X, UBL, JSON…).
    #[error("serialization: {0}")]
    Serialization(String),

    /// Erreur de transmission vers un canal de livraison
    /// (SMTP, PISTE / Chorus Pro…).
    #[error("delivery: {0}")]
    Delivery(String),

    /// Erreur d'entrée / sortie bas niveau.
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// Erreur JSON standard (serde_json).
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

impl Error {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::Serialization(msg.into())
    }

    pub fn delivery(msg: impl Into<String>) -> Self {
        Self::Delivery(msg.into())
    }
}
