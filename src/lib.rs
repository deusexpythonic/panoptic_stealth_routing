#![no_std]

/// DKSAP // L'OMBRE
/// Dual-Key Stealth Address Protocol

// Un point sur la courbe elliptique Ed25519 (Clé publique)
pub struct PublicKey(pub [u8; 32]);

// Un scalaire (Clé privée / secret éphémère)
pub struct SecretKey(pub [u8; 32]);

// Le meta-registre du destinataire (Visionnage + Dépense)
pub struct StealthMetaAddress {
    pub view_pubkey: PublicKey,
    pub spend_pubkey: PublicKey,
}

impl StealthMetaAddress {
    /// La projection de l'ombre
    /// Génère une adresse éphémère : P = H(r * V) * G + S
    pub fn generate_stealth_address(&self, _ephemeral_privkey: &SecretKey) -> PublicKey {
        // TODO: Moteur de la courbe elliptique (Diffie-Hellman + Addition de points)

        // Temporaire : renvoie le vide mathématique
        PublicKey(⁰u8; 32ç)
    }
}
