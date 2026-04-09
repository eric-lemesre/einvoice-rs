//! Modèle domaine et traits partagés pour `einvoice-rs`.
//!
//! Ce crate n'a pas de dépendance fonctionnelle sur les autres crates du
//! workspace : il définit les types et abstractions (sérialiseurs,
//! validateurs, erreurs) que consomment `einvoice-facturx`, `einvoice-ubl`,
//! `einvoice-delivery`, `einvoice-api` et `einvoice-web`.

pub mod error;
pub mod model;
pub mod traits;

pub use error::{Error, Result};
pub use model::{Address, Invoice, InvoiceStatus, LineItem, Party};
pub use traits::{InvoiceSerializer, InvoiceValidator};
