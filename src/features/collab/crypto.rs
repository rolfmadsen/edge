use serde::{Deserialize, Serialize};

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
    pub fn generate() -> Self {
        todo!("RED: not implemented yet")
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_base64(&self) -> String {
        todo!("RED: not implemented yet")
    }

    pub fn from_base64(_s: &str) -> Result<Self, CryptoError> {
        todo!("RED: not implemented yet")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoomId(String);

impl RoomId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        todo!("RED: not implemented yet")
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

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Kryptografisk godkendelse fejlede: ugyldig nøgle eller manipuleret data")]
    AuthenticationFailed,
    #[error("Ugyldigt dataformat: {0}")]
    InvalidFormat(String),
    #[error("Ugyldig nøgle: {0}")]
    InvalidKey(String),
}

pub fn encrypt(_key: &CollabKey, _plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    todo!("RED: not implemented yet")
}

pub fn decrypt(_key: &CollabKey, _ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    todo!("RED: not implemented yet")
}

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

    pub fn to_ticket_string(&self) -> String {
        todo!("RED: not implemented yet")
    }

    pub fn from_ticket_string(_s: &str) -> Result<Self, SessionTicketError> {
        todo!("RED: not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!room.as_str().is_empty());
        assert!(room.as_str().contains('-'));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = CollabKey::from_bytes([42u8; 32]);
        let data = b"Hemmelig FDA modeltilstand";
        let ciphertext = encrypt(&key, data).expect("kryptering skal lykkes");
        assert_ne!(ciphertext, data);
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
        // Manipuler en vilkårlig byte i ciphertext
        let len = ciphertext.len();
        ciphertext[len - 1] ^= 0x01;
        let result = decrypt(&key, &ciphertext);
        assert_eq!(result, Err(CryptoError::AuthenticationFailed));
    }

    #[test]
    fn test_decrypt_truncated_ciphertext_fails() {
        let key = CollabKey::from_bytes([4u8; 32]);
        let short_data = vec![0u8; 10]; // for kort til nonce (12) + auth tag (16)
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
    fn test_session_ticket_invalid_prefix() {
        let result = SessionTicket::from_ticket_string("invalid:prefix:xyz");
        assert_eq!(result, Err(SessionTicketError::InvalidPrefix));
    }
}
