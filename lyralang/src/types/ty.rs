//! Canonical type representations for the Stage 0 LyraLang kernel.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::types::effect::EffectSet;

/// Compiler-generated type variable identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TypeVariableId(pub u32);

impl TypeVariableId {
    /// Returns the canonical textual spelling of the type variable.
    #[must_use]
    pub fn canonical_name(self) -> String {
        format!("t{}", self.0)
    }
}

/// Primitive kernel types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    /// The unit type.
    Unit,
    /// The boolean type.
    Bool,
    /// The signed integer type.
    Int,
    /// The natural-number type.
    Nat,
    /// The exact rational type.
    Rational,
    /// A canonical stack-trace payload.
    StackTrace,
}

impl PrimitiveType {
    /// Returns the canonical textual spelling of the primitive type.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unit => "Unit",
            Self::Bool => "Bool",
            Self::Int => "Int",
            Self::Nat => "Nat",
            Self::Rational => "Rational",
            Self::StackTrace => "StackTrace",
        }
    }
}

/// Linear resource kernel types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResourceType {
    /// An owned file handle.
    File,
    /// An owned socket handle.
    Socket,
    /// An owned capability token.
    Capability,
}

impl ResourceType {
    /// Returns the canonical textual spelling of the resource type.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Socket => "Socket",
            Self::Capability => "Capability",
        }
    }
}


/// Canonical self-reference metadata types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MetaType {
    /// The current program descriptor.
    CurrentProgram,
    /// The current execution receipt descriptor.
    CurrentReceipt,
    /// The current ledger-state descriptor.
    LedgerState,
}

impl MetaType {
    /// Returns the canonical textual spelling of the metadata type.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentProgram => "CurrentProgram",
            Self::CurrentReceipt => "CurrentReceipt",
            Self::LedgerState => "LedgerState",
        }
    }
}

/// Epistemic modalities supported by the Stage 0 kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ModalKind {
    /// Proven, stable knowledge.
    Fact,
    /// Supported but not yet proven knowledge.
    Hypothesis,
    /// Knowledge whose truth value is not yet established.
    Unknown,
    /// Knowledge proven to hold in all admissible worlds.
    Necessary,
    /// Knowledge shown to hold in at least one admissible world.
    Possible,
}

impl ModalKind {
    /// Returns the canonical textual spelling of the modality.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "Fact",
            Self::Hypothesis => "Hypothesis",
            Self::Unknown => "Unknown",
            Self::Necessary => "Necessary",
            Self::Possible => "Possible",
        }
    }
}

/// Evidence kinds required to justify modal promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvidenceKind {
    /// Observation that narrows an unknown into a hypothesis.
    Observation,
    /// Proof artifact that certifies a fact-like promotion.
    Proof,
    /// Necessitation evidence that lifts a fact into necessity.
    Necessity,
    /// Feasibility evidence that lifts an unknown into possibility.
    Possibility,
}

impl EvidenceKind {
    /// Returns the canonical textual spelling of the evidence kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::Proof => "proof",
            Self::Necessity => "necessity",
            Self::Possibility => "possibility",
        }
    }
}

/// Canonical modal type representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalType {
    /// The epistemic modality carried by the type.
    pub modality: ModalKind,
    /// The underlying payload type.
    pub body: Box<Type>,
}


/// Canonical temporal proposition type representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalType {
    /// The payload observed across time.
    pub body: Box<Type>,
}

/// Canonical function-type representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionType {
    /// Ordered parameter types.
    pub parameters: Vec<Type>,
    /// Effect set produced by the function body.
    pub effects: EffectSet,
    /// Result type.
    pub result: Box<Type>,
}


/// Canonical composed error type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorType {
    /// Ordered, deduplicated error labels.
    pub labels: Vec<String>,
    /// Whether stack-trace integration is attached.
    pub stack_trace: bool,
}

impl ErrorType {
    /// Creates a canonical error label set.
    #[must_use]
    pub fn new(labels: Vec<String>, stack_trace: bool) -> Self {
        let mut ordered = BTreeSet::new();
        for label in labels {
            ordered.insert(label);
        }
        Self {
            labels: ordered.into_iter().collect(),
            stack_trace,
        }
    }

    /// Creates a single-label error.
    #[must_use]
    pub fn single(label: impl Into<String>) -> Self {
        Self::new(vec![label.into()], false)
    }

    /// Returns a copy with stack-trace integration enabled.
    #[must_use]
    pub fn with_trace(&self) -> Self {
        Self::new(self.labels.clone(), true)
    }

    /// Returns the canonical textual spelling.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        let joined = self.labels.join(" | ");
        if self.stack_trace {
            format!("Error[{} @trace]", joined)
        } else {
            format!("Error[{}]", joined)
        }
    }

    /// Returns the union of two error types.
    #[must_use]
    pub fn compose(&self, other: &Self) -> Self {
        let mut labels = self.labels.clone();
        labels.extend(other.labels.clone());
        Self::new(labels, self.stack_trace || other.stack_trace)
    }
}

/// Canonical result type representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultType {
    /// Successful payload type.
    pub ok: Box<Type>,
    /// Error payload type.
    pub err: Box<Type>,
}

/// Canonical Stage 0 type representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// A primitive type.
    Primitive(PrimitiveType),
    /// A linear resource type.
    Resource(ResourceType),
    /// A self-reference metadata type.
    Meta(MetaType),
    /// A canonical error type.
    Error(Box<ErrorType>),
    /// An option payload type.
    Option(Box<Type>),
    /// A result payload type.
    Result(Box<ResultType>),
    /// A structured-concurrency task handle.
    Task(Box<Type>),
    /// A typed channel surface.
    Channel(Box<Type>),
    /// An inference variable.
    Variable(TypeVariableId),
    /// A product type with ordered members.
    Product(Vec<Type>),
    /// A sum type with ordered members.
    Sum(Vec<Type>),
    /// A function type.
    Function(Box<FunctionType>),
    /// A modality-qualified payload type.
    Modal(Box<ModalType>),
    /// A temporal proposition value.
    Temporal(Box<TemporalType>),
    /// Evidence token required for modal promotion.
    Evidence(EvidenceKind),
}

impl Type {
    /// Returns the unit type.
    #[must_use]
    pub const fn unit() -> Self {
        Self::Primitive(PrimitiveType::Unit)
    }

    /// Returns the boolean type.
    #[must_use]
    pub const fn bool() -> Self {
        Self::Primitive(PrimitiveType::Bool)
    }

    /// Returns the integer type.
    #[must_use]
    pub const fn int() -> Self {
        Self::Primitive(PrimitiveType::Int)
    }

    /// Returns the natural-number type.
    #[must_use]
    pub const fn nat() -> Self {
        Self::Primitive(PrimitiveType::Nat)
    }

    /// Returns the rational type.
    #[must_use]
    pub const fn rational() -> Self {
        Self::Primitive(PrimitiveType::Rational)
    }

    /// Returns the stack-trace type.
    #[must_use]
    pub const fn stack_trace() -> Self {
        Self::Primitive(PrimitiveType::StackTrace)
    }

    /// Returns a canonical single-label error type.
    #[must_use]
    pub fn error(label: impl Into<String>) -> Self {
        Self::Error(Box::new(ErrorType::single(label)))
    }

    /// Returns a canonical composed error type.
    #[must_use]
    pub fn composed_error(labels: Vec<String>) -> Self {
        Self::Error(Box::new(ErrorType::new(labels, false)))
    }

    /// Returns an error type with stack-trace integration enabled.
    #[must_use]
    pub fn traced_error(labels: Vec<String>) -> Self {
        Self::Error(Box::new(ErrorType::new(labels, true)))
    }

    /// Returns an option type.
    #[must_use]
    pub fn option(body: Type) -> Self {
        Self::Option(Box::new(body))
    }

    /// Returns a result type.
    #[must_use]
    pub fn result(ok: Type, err: Type) -> Self {
        Self::Result(Box::new(ResultType {
            ok: Box::new(ok),
            err: Box::new(err),
        }))
    }


    /// Returns a structured-concurrency task type.
    #[must_use]
    pub fn task(body: Type) -> Self {
        Self::Task(Box::new(body))
    }

    /// Returns a typed channel surface.
    #[must_use]
    pub fn channel(body: Type) -> Self {
        Self::Channel(Box::new(body))
    }

    /// Returns the file resource type.
    #[must_use]
    pub const fn file() -> Self {
        Self::Resource(ResourceType::File)
    }

    /// Returns the socket resource type.
    #[must_use]
    pub const fn socket() -> Self {
        Self::Resource(ResourceType::Socket)
    }

    /// Returns the capability resource type.
    #[must_use]
    pub const fn capability() -> Self {
        Self::Resource(ResourceType::Capability)
    }

    /// Returns the current-program metadata type.
    #[must_use]
    pub const fn current_program() -> Self {
        Self::Meta(MetaType::CurrentProgram)
    }

    /// Returns the current-receipt metadata type.
    #[must_use]
    pub const fn current_receipt() -> Self {
        Self::Meta(MetaType::CurrentReceipt)
    }

    /// Returns the ledger-state metadata type.
    #[must_use]
    pub const fn ledger_state() -> Self {
        Self::Meta(MetaType::LedgerState)
    }

    /// Returns an evidence token type.
    #[must_use]
    pub const fn evidence(kind: EvidenceKind) -> Self {
        Self::Evidence(kind)
    }

    /// Constructs a generic modal wrapper.
    #[must_use]
    pub fn modal(modality: ModalKind, body: Type) -> Self {
        Self::Modal(Box::new(ModalType {
            modality,
            body: Box::new(body),
        }))
    }

    /// Returns a fact-qualified type.
    #[must_use]
    pub fn fact(body: Type) -> Self {
        Self::modal(ModalKind::Fact, body)
    }

    /// Returns a hypothesis-qualified type.
    #[must_use]
    pub fn hypothesis(body: Type) -> Self {
        Self::modal(ModalKind::Hypothesis, body)
    }

    /// Returns an unknown-qualified type.
    #[must_use]
    pub fn unknown(body: Type) -> Self {
        Self::modal(ModalKind::Unknown, body)
    }

    /// Returns a necessity-qualified type.
    #[must_use]
    pub fn necessary(body: Type) -> Self {
        Self::modal(ModalKind::Necessary, body)
    }

    /// Returns a possibility-qualified type.
    #[must_use]
    pub fn possible(body: Type) -> Self {
        Self::modal(ModalKind::Possible, body)
    }


    /// Constructs a temporal proposition type.
    #[must_use]
    pub fn temporal(body: Type) -> Self {
        Self::Temporal(Box::new(TemporalType { body: Box::new(body) }))
    }

    /// Constructs a function type.
    #[must_use]
    pub fn function(parameters: Vec<Type>, effects: EffectSet, result: Type) -> Self {
        Self::Function(Box::new(FunctionType {
            parameters,
            effects,
            result: Box::new(result),
        }))
    }

    /// Returns the resource kind when the type is linear.
    #[must_use]
    pub const fn resource_kind(&self) -> Option<ResourceType> {
        match self {
            Self::Resource(resource) => Some(*resource),
            _ => None,
        }
    }

    /// Returns the modal kind when the type is modal.
    #[must_use]
    pub const fn modal_kind(&self) -> Option<ModalKind> {
        match self {
            Self::Modal(modal) => Some(modal.modality),
            _ => None,
        }
    }

    /// Returns the modal payload when the type is modal.
    #[must_use]
    pub fn modal_body(&self) -> Option<&Type> {
        match self {
            Self::Modal(modal) => Some(modal.body.as_ref()),
            _ => None,
        }
    }

    /// Returns `true` when the type is a linear resource.
    #[must_use]
    pub const fn is_linear(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    /// Returns the canonical textual spelling of the type.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        match self {
            Self::Primitive(primitive) => primitive.as_str().to_string(),
            Self::Resource(resource) => resource.as_str().to_string(),
            Self::Meta(meta) => meta.as_str().to_string(),
            Self::Error(error) => error.canonical_name(),
            Self::Option(body) => format!("Option[{}]", body.canonical_name()),
            Self::Result(result) => format!(
                "Result[{}, {}]",
                result.ok.canonical_name(),
                result.err.canonical_name()
            ),
            Self::Task(body) => format!("Task[{}]", body.canonical_name()),
            Self::Channel(body) => format!("Channel[{}]", body.canonical_name()),
            Self::Variable(variable) => variable.canonical_name(),
            Self::Product(members) => format!(
                "Product[{}]",
                members
                    .iter()
                    .map(|member| member.canonical_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Sum(members) => format!(
                "Sum[{}]",
                members
                    .iter()
                    .map(|member| member.canonical_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Function(function) => format!(
                "Fn(({}) -{{{}}}-> {})",
                function
                    .parameters
                    .iter()
                    .map(|member| member.canonical_name())
                    .collect::<Vec<_>>()
                    .join(", "),
                function.effects.canonical_name(),
                function.result.canonical_name()
            ),
            Self::Modal(modal) => format!(
                "{}[{}]",
                modal.modality.as_str(),
                modal.body.canonical_name()
            ),
            Self::Temporal(temporal) => format!("Temporal[{}]", temporal.body.canonical_name()),
            Self::Evidence(kind) => format!("Evidence[{}]", kind.as_str()),
        }
    }

    /// Returns the set of free type variables in the type.
    #[must_use]
    pub fn free_type_variables(&self) -> BTreeSet<TypeVariableId> {
        match self {
            Self::Primitive(_) | Self::Resource(_) | Self::Meta(_) | Self::Error(_) | Self::Evidence(_) => BTreeSet::new(),
            Self::Option(body) | Self::Task(body) | Self::Channel(body) => body.free_type_variables(),
            Self::Result(result) => {
                let mut variables = result.ok.free_type_variables();
                variables.extend(result.err.free_type_variables());
                variables
            }
            Self::Variable(variable) => BTreeSet::from([*variable]),
            Self::Product(members) | Self::Sum(members) => members
                .iter()
                .flat_map(|member| member.free_type_variables())
                .collect(),
            Self::Function(function) => {
                let mut variables: BTreeSet<_> = function
                    .parameters
                    .iter()
                    .flat_map(|member| member.free_type_variables())
                    .collect();
                variables.extend(function.result.free_type_variables());
                variables
            }
            Self::Modal(modal) => modal.body.free_type_variables(),
            Self::Temporal(temporal) => temporal.body.free_type_variables(),
        }
    }

    /// Returns `true` when the type contains the given variable.
    #[must_use]
    pub fn contains_variable(&self, variable: TypeVariableId) -> bool {
        self.free_type_variables().contains(&variable)
    }

    /// Applies a substitution to the type.
    #[must_use]
    pub fn substitute(&self, substitutions: &BTreeMap<TypeVariableId, Type>) -> Type {
        match self {
            Self::Primitive(_) | Self::Resource(_) | Self::Meta(_) | Self::Error(_) | Self::Evidence(_) => self.clone(),
            Self::Option(body) => Self::option(body.substitute(substitutions)),
            Self::Task(body) => Self::task(body.substitute(substitutions)),
            Self::Channel(body) => Self::channel(body.substitute(substitutions)),
            Self::Result(result) => Self::result(
                result.ok.substitute(substitutions),
                result.err.substitute(substitutions),
            ),
            Self::Variable(variable) => substitutions
                .get(variable)
                .cloned()
                .unwrap_or_else(|| self.clone()),
            Self::Product(members) => Self::Product(
                members
                    .iter()
                    .map(|member| member.substitute(substitutions))
                    .collect(),
            ),
            Self::Sum(members) => Self::Sum(
                members
                    .iter()
                    .map(|member| member.substitute(substitutions))
                    .collect(),
            ),
            Self::Function(function) => Self::function(
                function
                    .parameters
                    .iter()
                    .map(|parameter| parameter.substitute(substitutions))
                    .collect(),
                function.effects.clone(),
                function.result.substitute(substitutions),
            ),
            Self::Modal(modal) => Self::modal(modal.modality, modal.body.substitute(substitutions)),
            Self::Temporal(temporal) => Self::temporal(temporal.body.substitute(substitutions)),
        }
    }
}

/// A Hindley-Milner type scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeScheme {
    /// Quantified variables in deterministic order.
    pub variables: Vec<TypeVariableId>,
    /// Monotype body.
    pub body: Type,
}

impl TypeScheme {
    /// Creates a monomorphic type scheme.
    #[must_use]
    pub fn mono(body: Type) -> Self {
        Self {
            variables: Vec::new(),
            body,
        }
    }

    /// Returns the set of free type variables in the scheme.
    #[must_use]
    pub fn free_type_variables(&self) -> BTreeSet<TypeVariableId> {
        let mut variables = self.body.free_type_variables();
        for variable in &self.variables {
            variables.remove(variable);
        }
        variables
    }

    /// Returns the canonical textual spelling of the scheme.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        if self.variables.is_empty() {
            return self.body.canonical_name();
        }

        let quantifiers = self
            .variables
            .iter()
            .map(|variable| variable.canonical_name())
            .collect::<Vec<_>>()
            .join(", ");
        format!("forall {}. {}", quantifiers, self.body.canonical_name())
    }

    /// Applies a substitution to all non-quantified variables in the scheme.
    #[must_use]
    pub fn substitute(&self, substitutions: &BTreeMap<TypeVariableId, Type>) -> Self {
        let filtered: BTreeMap<_, _> = substitutions
            .iter()
            .filter(|(variable, _)| !self.variables.contains(variable))
            .map(|(variable, ty)| (*variable, ty.clone()))
            .collect();
        Self {
            variables: self.variables.clone(),
            body: self.body.substitute(&filtered),
        }
    }
}
