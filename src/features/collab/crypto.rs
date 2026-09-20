use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// Symmetrisk 256-bit krypteringsnøgle til ChaCha20-Poly1305 AEAD.
#[derive(Clone, PartialEq, Eq)]
pub struct CollabKey(pub [u8; 32]);

impl std::fmt::Debug for CollabKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CollabKey([REDACTED])")
    }
}

impl Serialize for CollabKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base64())
    }
}

impl<'de> Deserialize<'de> for CollabKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_base64(&s).map_err(serde::de::Error::custom)
    }
}

impl CollabKey {
    /// Genererer en kryptografisk sikker tilfældig 256-bit nøgle.
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Serialiserer nøglen til en URL-sikker base64-streng uden padding.
    pub fn to_base64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.0)
    }

    /// Parser en nøgle fra URL-sikker eller standard base64.
    pub fn from_base64(s: &str) -> Result<Self, CryptoError> {
        let trimmed = s.trim();
        let bytes = URL_SAFE_NO_PAD
            .decode(trimmed)
            .or_else(|_| URL_SAFE.decode(trimmed))
            .or_else(|_| STANDARD.decode(trimmed))
            .map_err(|e| CryptoError::InvalidKey(format!("Ugyldig base64-nøgle: {e}")))?;

        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKey(format!(
                "Nøglelængde skal være 32 bytes, modtog {} bytes",
                bytes.len()
            )));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&bytes);
        Ok(Self(key))
    }
}

/// Alfanumerisk identifikator for et kollaboreringsrum (f.eks. "KU-4821").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoomId(String);

impl RoomId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into().trim().to_uppercase())
    }

    /// Genererer en læsevenlig, tilfældig rum-identifikator (f.eks. "KU-4821").
    pub fn generate() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let prefix: [char; 2] = [
            rng.random_range(b'A'..=b'Z') as char,
            rng.random_range(b'A'..=b'Z') as char,
        ];
        let num: u16 = rng.random_range(1000..=9999);
        Self(format!("{}{}-{}", prefix[0], prefix[1], num))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for RoomId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Fejl under klientside kryptering eller dekryptering.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Kryptografisk godkendelse fejlede: ugyldig nøgle eller manipuleret data")]
    AuthenticationFailed,
    #[error("Ugyldigt dataformat: {0}")]
    InvalidFormat(String),
    #[error("Ugyldig nøgle: {0}")]
    InvalidKey(String),
}

/// Krypterer en vilkårlig payload med ChaCha20-Poly1305.
/// Output-format: `[12-byte tilfældig nonce] || [ciphertext + 16-byte Poly1305 auth tag]`.
pub fn encrypt(key: &CollabKey, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::AuthenticationFailed)?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Dekrypterer en payload med ChaCha20-Poly1305 og validerer Poly1305 integritetstagget.
pub fn decrypt(key: &CollabKey, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // 12 bytes nonce + 16 bytes auth tag = minimum 28 bytes
    if ciphertext.len() < 28 {
        return Err(CryptoError::InvalidFormat(
            "Ciphertext for kort til at indeholde nonce og godkendelsestag".to_string(),
        ));
    }

    let (nonce_bytes, encrypted_payload) = ciphertext.split_at(12);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, encrypted_payload)
        .map_err(|_| CryptoError::AuthenticationFailed)
}

/// Sessionsbillet (Token) til deling mellem Host og Guest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTicket {
    pub relay_url: String,
    pub room_id: RoomId,
    pub key: CollabKey,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SessionTicketError {
    #[error("Ugyldigt sessionsbillet-præfiks (skal starte med 'edge:v1:')")]
    InvalidPrefix,
    #[error("Ugyldig base64-kodning: {0}")]
    InvalidBase64(String),
    #[error("Ugyldig JSON-struktur: {0}")]
    InvalidJson(String),
    #[error("Ugyldigt billetformat: {0}")]
    InvalidFormat(String),
    #[error("Ugyldig kryptonøgle i billet: {0}")]
    InvalidKey(#[from] CryptoError),
}

impl SessionTicket {
    pub fn new(relay_url: impl Into<String>, room_id: RoomId, key: CollabKey) -> Self {
        Self {
            relay_url: relay_url.into(),
            room_id,
            key,
        }
    }

    /// Serialiserer billetten til det kompakte, url-sikre format `edge:v1:<base64-payload>`.
    pub fn to_ticket_string(&self) -> String {
        let json = serde_json::to_vec(self).expect("SessionTicket skal altid kunne serialiseres");
        format!("edge:v1:{}", URL_SAFE_NO_PAD.encode(json))
    }

    /// Parser en sessionsbillet fra enten `edge:v1:<base64-payload>` eller
    /// `edge:v1:<base64(server)>:<room_id>:<base64(key)>`.
    pub fn from_ticket_string(s: &str) -> Result<Self, SessionTicketError> {
        let trimmed = s.trim();
        let payload = trimmed
            .strip_prefix("edge:v1:")
            .ok_or(SessionTicketError::InvalidPrefix)?;

        if payload.is_empty() {
            return Err(SessionTicketError::InvalidFormat(
                "Tom sessionsbillet".to_string(),
            ));
        }

        // Tjek om billetten er formateret med kolon-separerede felter
        let parts: Vec<&str> = payload.split(':').collect();
        if parts.len() == 3 {
            let server_bytes = URL_SAFE_NO_PAD
                .decode(parts[0])
                .or_else(|_| URL_SAFE.decode(parts[0]))
                .or_else(|_| STANDARD.decode(parts[0]))
                .map_err(|e| SessionTicketError::InvalidBase64(e.to_string()))?;

            let relay_url = String::from_utf8(server_bytes)
                .map_err(|e| SessionTicketError::InvalidFormat(e.to_string()))?;
            let room_id = RoomId::new(parts[1]);
            let key = CollabKey::from_base64(parts[2])?;

            return Ok(Self {
                relay_url,
                room_id,
                key,
            });
        }

        // Ellers tolkes payload som base64-kodet JSON
        let json_bytes = URL_SAFE_NO_PAD
            .decode(payload)
            .or_else(|_| URL_SAFE.decode(payload))
            .or_else(|_| STANDARD.decode(payload))
            .map_err(|e| SessionTicketError::InvalidBase64(e.to_string()))?;

        serde_json::from_slice(&json_bytes)
            .map_err(|e| SessionTicketError::InvalidJson(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_collab_key_generation_and_base64() {
        let key = CollabKey::generate();
        let b64 = key.to_base64();
        let parsed = CollabKey::from_base64(&b64).expect("skal kunne parses fra base64");
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_room_id_generation() {
        let room = RoomId::generate();
        assert_eq!(room.as_str().len(), 7); // f.eks. "AB-1234"
        assert!(room.as_str().contains('-'));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = CollabKey::from_bytes([42u8; 32]);
        let data = b"Hemmelig FDA modeltilstand med begreber og relationer";
        let ciphertext = encrypt(&key, data).expect("kryptering skal lykkes");
        assert_ne!(ciphertext, data);
        assert_eq!(ciphertext.len(), data.len() + 28); // 12 nonce + 16 auth tag
        let decrypted = decrypt(&key, &ciphertext).expect("dekryptering skal lykkes");
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails_auth() {
        let key1 = CollabKey::from_bytes([1u8; 32]);
        let key2 = CollabKey::from_bytes([2u8; 32]);
        let ciphertext = encrypt(&key1, b"hemmelighed").unwrap();
        let result = decrypt(&key2, &ciphertext);
        assert_eq!(result, Err(CryptoError::AuthenticationFailed));
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails_auth() {
        let key = CollabKey::from_bytes([3u8; 32]);
        let mut ciphertext = encrypt(&key, b"vigtig data").unwrap();
        let len = ciphertext.len();
        ciphertext[len - 1] ^= 0x01; // Bit flip i auth tag
        let result = decrypt(&key, &ciphertext);
        assert_eq!(result, Err(CryptoError::AuthenticationFailed));

        ciphertext[0] ^= 0x01; // Bit flip i nonce
        let result2 = decrypt(&key, &ciphertext);
        assert_eq!(result2, Err(CryptoError::AuthenticationFailed));
    }

    #[test]
    fn test_decrypt_truncated_ciphertext_fails() {
        let key = CollabKey::from_bytes([4u8; 32]);
        let short_data = vec![0u8; 20]; // for kort til 28 bytes
        let result = decrypt(&key, &short_data);
        assert!(matches!(result, Err(CryptoError::InvalidFormat(_))));
    }

    #[test]
    fn test_session_ticket_roundtrip_edge_v1() {
        let ticket = SessionTicket::new(
            "https://edge.relay.internal",
            RoomId::new("KU-1234"),
            CollabKey::from_bytes([9u8; 32]),
        );
        let s = ticket.to_ticket_string();
        assert!(s.starts_with("edge:v1:"));
        let parsed = SessionTicket::from_ticket_string(&s).expect("Billet skal parses");
        assert_eq!(ticket, parsed);
    }

    #[test]
    fn test_session_ticket_colon_format() {
        let server = "https://relay.ku.dk";
        let room_id = RoomId::new("KU-4821");
        let key = CollabKey::from_bytes([7u8; 32]);

        let s = format!(
            "edge:v1:{}:{}:{}",
            URL_SAFE_NO_PAD.encode(server),
            room_id.as_str(),
            key.to_base64()
        );

        let parsed = SessionTicket::from_ticket_string(&s).expect("Kolon-billet skal parses");
        assert_eq!(parsed.relay_url, server);
        assert_eq!(parsed.room_id, room_id);
        assert_eq!(parsed.key, key);
    }

    #[test]
    fn test_session_ticket_invalid_prefix() {
        let result = SessionTicket::from_ticket_string("invalid:prefix:xyz");
        assert_eq!(result, Err(SessionTicketError::InvalidPrefix));
    }

    #[test]
    fn test_session_ticket_empty_payload() {
        let result = SessionTicket::from_ticket_string("edge:v1:");
        assert!(matches!(result, Err(SessionTicketError::InvalidFormat(_))));
    }

    // Property-based tests for tabsløs kryptering og auth-validering
    proptest! {
        #[test]
        fn proptest_lossless_encrypt_decrypt(data in proptest::collection::vec(any::<u8>(), 0..2048)) {
            let key = CollabKey::generate();
            let ciphertext = encrypt(&key, &data).unwrap();
            let decrypted = decrypt(&key, &ciphertext).unwrap();
            prop_assert_eq!(data, decrypted);
        }

        #[test]
        fn proptest_tamper_detection(
            data in proptest::collection::vec(any::<u8>(), 1..512),
            byte_index in 0usize..500,
            bit_mask in 1u8..=255
        ) {
            let key = CollabKey::generate();
            let mut ciphertext = encrypt(&key, &data).unwrap();
            let idx = byte_index % ciphertext.len();
            ciphertext[idx] ^= bit_mask;

            let result = decrypt(&key, &ciphertext);
            prop_assert_eq!(result, Err(CryptoError::AuthenticationFailed));
        }
    }
}
