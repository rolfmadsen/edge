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
