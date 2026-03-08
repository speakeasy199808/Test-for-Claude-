//! Type-system kernel structures for LyraLang Stage 0.
//!
//! This module defines the canonical internal type algebra used by the seed
//! type checker. Concrete source-level type syntax is intentionally deferred.

pub mod effect;
pub mod ty;

pub use effect::{EffectAtom, EffectSet};
pub use ty::{
    ErrorType, EvidenceKind, FunctionType, MetaType, ModalKind, ModalType, PrimitiveType,
    ResourceType, ResultType, TemporalType, Type, TypeScheme, TypeVariableId,
};
