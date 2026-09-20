use crate::features::concept_model::RelationKind;
use crate::features::concepts::Concept;
use crate::features::information_model::{InformationClass, Multiplicity};
use crate::features::model::ModelProject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Enkeltstående relation mellem to noder på diagrammet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relation {
    pub id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    pub kind: RelationKind,
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directed: Option<bool>,
}

impl Relation {
    pub fn new(from: Uuid, to: Uuid, kind: RelationKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            kind,
            label: None,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
        }
    }

    pub fn with_label(from: Uuid, to: Uuid, kind: RelationKind, label: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            kind,
            label,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
        }
    }

    pub fn with_id(
        id: Uuid,
        from: Uuid,
        to: Uuid,
        kind: RelationKind,
        label: Option<String>,
    ) -> Self {
        Self {
            id,
            from,
            to,
            kind,
            label,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
        }
    }

    pub fn with_all(
        id: Uuid,
        from: Uuid,
        to: Uuid,
        kind: RelationKind,
        label: Option<String>,
        directed: Option<bool>,
    ) -> Self {
        Self {
            id,
            from,
            to,
            kind,
            label,
            directed,
        }
    }
}

/// Inkrementel forretningsmutation af modelprojektet under en live session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelMutation {
    ConceptAdded(Concept),
    ConceptUpdated(Concept),
    ConceptDeleted(Uuid),
    ConceptDiagramNodeAdded(Uuid),
    ConceptDiagramNodeRemoved(Uuid),
    InformationClassAdded(InformationClass),
    InformationClassUpdated(InformationClass),
    InformationClassDeleted(Uuid),
    ClassDiagramNodeAdded(Uuid),
    ClassDiagramNodeRemoved(Uuid),
    RelationAdded(Relation),
    RelationUpdated(Relation),
    RelationDeleted {
        from: Uuid,
        to: Uuid,
    },
    ClassRelationAdded {
        from_class: Uuid,
        to_class: Uuid,
        kind: RelationKind,
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_multiplicity: Option<Multiplicity>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target_multiplicity: Option<Multiplicity>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        directed: Option<bool>,
    },
    ClassRelationDeleted {
        from_class: Uuid,
        to_class: Uuid,
    },
    NodeMoved {
        id: Uuid,
        x: f32,
        y: f32,
    },
}

/// Overordnet E2EE netværksprotokol for live kollaboration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollabPayload {
    /// Komplet modelsnapshot ved tilslutning eller resynkronisering.
    Snapshot(ModelProject),
    /// Inkrementel mutation under aktiv redigering.
    Mutation(ModelMutation),
}

/// Eksplicitte transport frame-typer mod nonce-kollision og protokolforveksling.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameType {
    Snapshot = 0x01,
    Mutation = 0x02,
    Presence = 0x03,
    HostLeft = 0x04,
}

impl FrameType {
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(Self::Snapshot),
            0x02 => Some(Self::Mutation),
            0x03 => Some(Self::Presence),
            0x04 => Some(Self::HostLeft),
            _ => None,
        }
    }
}

/// Sikkerhedskuvert med sekvensnummer og tidsstempel til beskyttelse mod replay-angreb.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollabEnvelope {
    pub seq: u64,
    pub timestamp: u64,
    pub payload: CollabPayload,
}

impl CollabEnvelope {
    pub fn new(seq: u64, timestamp: u64, payload: CollabPayload) -> Self {
        Self {
            seq,
            timestamp,
            payload,
        }
    }

    pub fn is_newer_than(&self, last_seq: u64) -> bool {
        self.seq > last_seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::concepts::BelongsToDomain;

    #[test]
    fn test_relation_constructors() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        let rel = Relation::new(from, to, RelationKind::Generalization);
        assert_eq!(rel.from, from);
        assert_eq!(rel.to, to);
        assert_eq!(rel.kind, RelationKind::Generalization);
        assert_eq!(rel.label, None);

        let labeled = Relation::with_label(
            from,
            to,
            RelationKind::Association,
            Some("forbinder".to_string()),
        );
        assert_eq!(labeled.label, Some("forbinder".to_string()));

        let fixed_id = Uuid::new_v4();
        let with_id = Relation::with_id(
            fixed_id,
            from,
            to,
            RelationKind::Composition,
            Some("indeholder".to_string()),
        );
        assert_eq!(with_id.id, fixed_id);
    }

    #[test]
    fn test_frametype_bytes() {
        assert_eq!(FrameType::Snapshot as u8, 0x01);
        assert_eq!(FrameType::Mutation as u8, 0x02);
        assert_eq!(FrameType::Presence as u8, 0x03);
        assert_eq!(FrameType::HostLeft as u8, 0x04);

        assert_eq!(FrameType::from_u8(0x01), Some(FrameType::Snapshot));
        assert_eq!(FrameType::from_u8(0x02), Some(FrameType::Mutation));
        assert_eq!(FrameType::from_u8(0x03), Some(FrameType::Presence));
        assert_eq!(FrameType::from_u8(0x04), Some(FrameType::HostLeft));
        assert_eq!(FrameType::from_u8(0xFF), None);
    }

    #[test]
    fn test_collab_envelope_replay_protection() {
        let concept = Concept::new("Vej", "Færdselsareal", BelongsToDomain::Yes);
        let payload = CollabPayload::Mutation(ModelMutation::ConceptAdded(concept));

        let envelope1 = CollabEnvelope::new(1, 1000, payload.clone());
        let envelope2 = CollabEnvelope::new(2, 1050, payload.clone());
        let replay_envelope = CollabEnvelope::new(1, 1000, payload.clone());

        assert!(envelope1.is_newer_than(0));
        assert!(envelope2.is_newer_than(1));
        assert!(!replay_envelope.is_newer_than(1));
        assert!(!replay_envelope.is_newer_than(2));
    }
}
