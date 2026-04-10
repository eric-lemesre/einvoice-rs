//! Client pour l'API **Chorus Pro** exposée via le portail **PISTE**
//! (<https://piste.gouv.fr>).
//!
//! Chorus Pro est la plateforme publique française de facturation
//! électronique. L'accès se fait via PISTE avec un flux `OAuth2`
//! `client_credentials` qui produit un jeton d'accès valide
//! ~1h, utilisé pour signer les appels à `/cpro/factures/v1/deposer`.

use einvoice_core::{Error, Invoice, Result};

use crate::DeliveryReceipt;

/// Client Chorus Pro.
#[derive(Debug, Clone)]
pub struct ChorusProClient {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub piste_token: Option<String>,
}

impl ChorusProClient {
    pub fn new(
        base_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            piste_token: None,
        }
    }

    /// Obtient un jeton `OAuth2` via le flow `client_credentials` de PISTE.
    pub async fn authenticate(&mut self) -> Result<()> {
        // TODO : POST {base_url}/oauth/api/v1/token
        // avec grant_type=client_credentials, scope=openid
        // et stocker la réponse dans `self.piste_token`.
        Err(Error::delivery(
            "chorus-pro / PISTE authentication not yet implemented",
        ))
    }

    /// Dépose une facture (binaire, déjà sérialisée en Factur-X ou UBL)
    /// sur Chorus Pro et renvoie l'accusé.
    pub async fn deposit(
        &self,
        invoice: &Invoice,
        _payload: &[u8],
        _format: ChorusProFormat,
    ) -> Result<DeliveryReceipt> {
        if self.piste_token.is_none() {
            return Err(Error::delivery(
                "chorus-pro client is not authenticated — call authenticate() first",
            ));
        }
        tracing::debug!(invoice = %invoice.number, "chorus-pro deposit stub");
        // TODO : POST {base_url}/cpro/factures/v1/deposer avec Authorization
        // Bearer, content-type multipart, et parser la réponse.
        Err(Error::delivery("chorus-pro deposit not yet implemented"))
    }
}

/// Formats acceptés par `deposer` côté Chorus Pro.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChorusProFormat {
    UblInvoice,
    CiiInvoice,
    FacturX,
}

impl ChorusProFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UblInvoice => "UBL_INVOICE",
            Self::CiiInvoice => "CII",
            Self::FacturX => "FACTUR-X",
        }
    }
}
