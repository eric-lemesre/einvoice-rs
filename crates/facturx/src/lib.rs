//! Sérialiseur **Factur-X** (ZUGFeRD 2.x).
//!
//! Factur-X est un format hybride : un PDF/A-3 contient un fichier XML CII
//! (UN/CEFACT Cross Industry Invoice) conforme à **EN 16931**. Plusieurs
//! profils existent, du plus minimal (« Minimum ») au plus riche
//! (« Extended »).
//!
//! Ce crate expose un [`FacturXSerializer`] qui implémente le trait
//! [`InvoiceSerializer`]. La génération du XML CII est prise en charge par
//! le module [`cii`] ; l'encapsulation dans un PDF/A-3 reste à faire.

pub mod cii;

use einvoice_core::{Error, Invoice, InvoiceSerializer, Result};

/// Profils Factur-X définis par la norme (ordre croissant de richesse).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FacturXProfile {
    /// Profil minimal — données strictement nécessaires à la comptabilité.
    Minimum,
    /// Profil « Basic WL » (without lines) — pas de lignes détaillées.
    BasicWl,
    /// Profil « Basic » — lignes détaillées minimales.
    Basic,
    /// Profil de référence conforme à EN 16931.
    #[default]
    En16931,
    /// Profil étendu — structures additionnelles hors EN 16931.
    Extended,
}

impl FacturXProfile {
    /// Identifiant URN du profil, à inscrire dans l'élément
    /// `GuidelineSpecifiedDocumentContextParameter` du CII.
    pub fn specification_identifier(&self) -> &'static str {
        match self {
            Self::Minimum => "urn:factur-x.eu:1p0:minimum",
            Self::BasicWl => "urn:factur-x.eu:1p0:basicwl",
            Self::Basic => "urn:cen.eu:en16931:2017#compliant#urn:factur-x.eu:1p0:basic",
            Self::En16931 => "urn:cen.eu:en16931:2017",
            Self::Extended => "urn:cen.eu:en16931:2017#conformant#urn:factur-x.eu:1p0:extended",
        }
    }
}

/// Sérialiseur Factur-X.
#[derive(Debug, Clone, Default)]
pub struct FacturXSerializer {
    pub profile: FacturXProfile,
}

impl FacturXSerializer {
    pub fn new(profile: FacturXProfile) -> Self {
        Self { profile }
    }

    /// Produit uniquement le XML CII (sans encapsulation PDF/A-3).
    ///
    /// Utile pour les tests et pour les scénarios où l'on n'a besoin que de
    /// la partie structurée (dépôt sur Chorus Pro en « facture mixte », par
    /// exemple).
    pub fn serialize_xml(&self, invoice: &Invoice) -> Result<Vec<u8>> {
        cii::write_cross_industry_invoice(invoice, self.profile)
    }
}

impl InvoiceSerializer for FacturXSerializer {
    /// Produit un PDF/A-3 Factur-X (XML CII embarqué). Non implémenté.
    fn serialize(&self, _invoice: &Invoice) -> Result<Vec<u8>> {
        // TODO : encapsuler le XML généré dans un PDF/A-3 (crate lopdf ou
        // printpdf) avec les métadonnées XMP requises et la pièce jointe
        // `factur-x.xml`.
        Err(Error::serialization(
            "Factur-X PDF/A-3 serializer not yet implemented",
        ))
    }

    fn content_type(&self) -> &'static str {
        "application/pdf"
    }

    fn format_name(&self) -> &'static str {
        "Factur-X"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_have_distinct_identifiers() {
        let all = [
            FacturXProfile::Minimum,
            FacturXProfile::BasicWl,
            FacturXProfile::Basic,
            FacturXProfile::En16931,
            FacturXProfile::Extended,
        ];
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                assert_ne!(a.specification_identifier(), b.specification_identifier());
            }
        }
    }

    #[test]
    fn default_profile_is_en16931() {
        assert_eq!(
            FacturXSerializer::default().profile,
            FacturXProfile::En16931
        );
    }
}
