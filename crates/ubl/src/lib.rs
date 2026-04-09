//! Sérialiseur **UBL 2.1** (Universal Business Language).
//!
//! Produit un XML conforme à :
//! - **Chorus Pro** — syntaxe UBL acceptée par la plateforme publique française
//! - **PEPPOL BIS Billing 3.0** — profil européen le plus répandu pour le B2B
//!
//! Le format UBL est à 100 % XML (pas d'enveloppe PDF), ce qui en fait le
//! format de choix pour la transmission automatisée (EDI, PEPPOL, PPF/PDP).

use einvoice_core::{Error, Invoice, InvoiceSerializer, Result};

/// Profils UBL supportés.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UblProfile {
    /// Profil Chorus Pro (plateforme publique française actuelle).
    ChorusPro,
    /// PEPPOL BIS Billing 3.0 — profil européen standard.
    #[default]
    PeppolBis3,
}

impl UblProfile {
    /// Identifiant `CustomizationID` correspondant.
    pub fn customization_id(&self) -> &'static str {
        match self {
            Self::ChorusPro => "urn:cen.eu:en16931:2017",
            Self::PeppolBis3 => {
                "urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0"
            }
        }
    }

    /// Identifiant `ProfileID` correspondant.
    pub fn profile_id(&self) -> &'static str {
        match self {
            Self::ChorusPro => "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0",
            Self::PeppolBis3 => "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0",
        }
    }
}

/// Sérialiseur UBL 2.1.
#[derive(Debug, Clone, Default)]
pub struct UblSerializer {
    pub profile: UblProfile,
}

impl UblSerializer {
    pub fn new(profile: UblProfile) -> Self {
        Self { profile }
    }
}

impl InvoiceSerializer for UblSerializer {
    fn serialize(&self, _invoice: &Invoice) -> Result<Vec<u8>> {
        // TODO : construire l'élément racine `<Invoice>` avec les namespaces
        // UBL (cbc, cac…) et remplir en fonction du profil.
        Err(Error::serialization(
            "UBL 2.1 serializer not yet implemented",
        ))
    }

    fn content_type(&self) -> &'static str {
        "application/xml"
    }

    fn format_name(&self) -> &'static str {
        "UBL 2.1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_is_peppol() {
        assert_eq!(UblSerializer::default().profile, UblProfile::PeppolBis3);
    }

    #[test]
    fn profiles_have_customization_ids() {
        assert!(!UblProfile::ChorusPro.customization_id().is_empty());
        assert!(!UblProfile::PeppolBis3.customization_id().is_empty());
    }
}
