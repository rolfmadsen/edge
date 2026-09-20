use crate::features::concept_model::RelationKind;
use crate::features::concepts::Concept;
use crate::features::information_model::InformationClass;
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
}

impl Relation {
    pub fn new(from: Uuid, to: Uuid, kind: RelationKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            kind,
            label: None,
        }
    }

    pub fn with_label(from: Uuid, to: Uuid, kind: RelationKind, label: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            kind,
            label,
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
        }
    }
}

/// Inkrementel forretningsmutation af modelprojektet under en live session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelMutation {
    ConceptAdded(Concept),
    ConceptUpdated(Concept),
    ConceptDeleted(Uuid),
    InformationClassAdded(InformationClass),
    InformationClassUpdated(InformationClass),
    InformationClassDeleted(Uuid),
    RelationAdded(Relation),
    RelationDeleted(Uuid),
    NodeMoved { id: Uuid, x: f32, y: f32 },
}

/// Overordnet E2EE netværksprotokol for live kollaboration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollabPayload {
    /// Komplet modelsnapshot ved tilslutning eller resynkronisering.
    Snapshot(ModelProject),
    /// Inkrementel mutation under aktiv redigering.
    Mutation(ModelMutation),
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

