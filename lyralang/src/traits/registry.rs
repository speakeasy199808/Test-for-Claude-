//! Internal trait/typeclass registry for the Stage 0 LyraLang compiler.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::traits::error::{TraitError, TraitErrorKind};
use crate::types::Type;

/// How an instance method is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitImplementationStyle {
    /// The instance resolves to a concrete builtin surface.
    ExplicitBuiltin,
    /// The instance uses canonical rendered equality as its default body.
    DefaultCanonicalEquality,
    /// The instance uses canonical rendered I/O output as its default body.
    DefaultCanonicalPrint,
}

impl TraitImplementationStyle {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExplicitBuiltin => "explicit_builtin",
            Self::DefaultCanonicalEquality => "default_canonical_equality",
            Self::DefaultCanonicalPrint => "default_canonical_print",
        }
    }
}

/// A canonical method signature carried by a trait definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitMethodSignature {
    /// Method name as surfaced to Stage 0 source.
    pub method_name: String,
    /// Canonical parameter types.
    pub parameters: Vec<String>,
    /// Canonical result type.
    pub result: String,
    /// Canonical latent effects.
    pub latent_effects: Vec<String>,
}

/// A trait definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitDefinition {
    /// Trait name.
    pub name: String,
    /// Ownership root for the trait.
    pub owner_root: String,
    /// Exported method surface.
    pub methods: Vec<TraitMethodSignature>,
    /// Canonical description of the default implementation strategy.
    pub default_implementation: String,
    /// Canonical derive macro that expands instances for kernel-owned types.
    pub derive_macro: String,
}

/// A concrete trait instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitInstance {
    /// Implemented trait name.
    pub trait_name: String,
    /// Canonical type receiving the instance.
    pub for_type: String,
    /// Ownership root declaring the instance.
    pub owner_root: String,
    /// Method target used by resolution.
    pub method_target: String,
    /// How the target body is provided.
    pub implementation_style: TraitImplementationStyle,
    /// Optional derive expansion responsible for the instance.
    pub derived_via: Option<String>,
}

/// Record of a derive expansion carried by the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExpansion {
    /// Macro spelling.
    pub macro_name: String,
    /// Canonical instance keys emitted by the expansion.
    pub generated_instances: Vec<String>,
}

/// A fully materialized trait registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitRegistry {
    /// Registry version.
    pub version: String,
    /// Canonical trait definitions.
    pub traits: Vec<TraitDefinition>,
    /// Canonical trait instances.
    pub instances: Vec<TraitInstance>,
    /// Recorded derive expansions.
    pub derive_expansions: Vec<DerivedExpansion>,
}

/// Deterministic result of trait-method resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitResolution {
    /// Requested source method name.
    pub method_name: String,
    /// Trait supplying the method.
    pub trait_name: String,
    /// Canonical receiver/argument type for the selected instance.
    pub for_type: String,
    /// Concrete target selected by coherence-safe resolution.
    pub target: String,
    /// Implementation style used by the instance.
    pub implementation_style: TraitImplementationStyle,
}

/// Result bundle returned by registry validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitCheckOutput {
    /// Validated registry when successful.
    pub registry: Option<TraitRegistry>,
    /// Diagnostics emitted during validation.
    pub errors: Vec<TraitError>,
}

impl TraitRegistry {
    /// Validates coherence and orphan rules for the registry.
    pub fn validate(&self) -> Result<(), TraitError> {
        let known_traits: BTreeSet<_> = self.traits.iter().map(|definition| definition.name.as_str()).collect();
        let trait_owners: BTreeMap<_, _> = self
            .traits
            .iter()
            .map(|definition| (definition.name.as_str(), definition.owner_root.as_str()))
            .collect();

        let mut keys = BTreeSet::new();
        for instance in &self.instances {
            if !known_traits.contains(instance.trait_name.as_str()) {
                return Err(TraitError::new(
                    TraitErrorKind::UnknownTrait,
                    format!(
                        "instance for `{}` references unknown trait `{}`",
                        instance.for_type, instance.trait_name
                    ),
                ));
            }

            let key = format!("{}::{}::{}", instance.trait_name, instance.for_type, instance.method_target);
            if !keys.insert(key.clone()) {
                return Err(TraitError::new(
                    TraitErrorKind::CoherenceViolation,
                    format!("overlapping trait instance detected for `{key}`"),
                ));
            }

            let trait_owner = trait_owners
                .get(instance.trait_name.as_str())
                .copied()
                .unwrap_or("<unknown>");
            let type_owner = type_owner_root(&instance.for_type);
            if instance.owner_root != trait_owner && instance.owner_root != type_owner {
                return Err(TraitError::new(
                    TraitErrorKind::OrphanInstance,
                    format!(
                        "instance `{}` for `{}` violates orphan rule: owner `{}` must match trait owner `{}` or type owner `{}`",
                        instance.trait_name, instance.for_type, instance.owner_root, trait_owner, type_owner,
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Resolves a Stage 0 method name against canonical argument types.
    #[must_use]
    pub fn resolve_method(&self, method_name: &str, argument_types: &[Type]) -> Option<TraitResolution> {
        let canonical_arguments: Vec<_> = argument_types.iter().map(Type::canonical_name).collect();

        match method_name {
            "eq" => {
                if canonical_arguments.len() != 2 || canonical_arguments[0] != canonical_arguments[1] {
                    return None;
                }
                self.find_instance("Eq", &canonical_arguments[0]).map(|instance| TraitResolution {
                    method_name: method_name.to_string(),
                    trait_name: "Eq".to_string(),
                    for_type: instance.for_type.clone(),
                    target: instance.method_target.clone(),
                    implementation_style: instance.implementation_style,
                })
            }
            "print" => {
                if canonical_arguments.len() != 1 {
                    return None;
                }
                self.find_instance("Print", &canonical_arguments[0]).map(|instance| TraitResolution {
                    method_name: method_name.to_string(),
                    trait_name: "Print".to_string(),
                    for_type: instance.for_type.clone(),
                    target: instance.method_target.clone(),
                    implementation_style: instance.implementation_style,
                })
            }
            _ => None,
        }
    }

    fn find_instance(&self, trait_name: &str, for_type: &str) -> Option<&TraitInstance> {
        self.instances
            .iter()
            .find(|instance| instance.trait_name == trait_name && instance.for_type == for_type)
    }
}

/// Returns the deterministic Stage 0 seed trait registry.
#[must_use]
pub fn seed_registry() -> TraitRegistry {
    let traits = vec![
        TraitDefinition {
            name: "Eq".to_string(),
            owner_root: "lyralang/".to_string(),
            methods: vec![TraitMethodSignature {
                method_name: "eq".to_string(),
                parameters: vec!["T".to_string(), "T".to_string()],
                result: "Bool".to_string(),
                latent_effects: Vec::new(),
            }],
            default_implementation: "canonical rendered equality".to_string(),
            derive_macro: "derive(Eq)".to_string(),
        },
        TraitDefinition {
            name: "Print".to_string(),
            owner_root: "lyralang/".to_string(),
            methods: vec![TraitMethodSignature {
                method_name: "print".to_string(),
                parameters: vec!["T".to_string()],
                result: "Unit".to_string(),
                latent_effects: vec!["io".to_string()],
            }],
            default_implementation: "canonical rendered I/O output".to_string(),
            derive_macro: "derive(Print)".to_string(),
        },
    ];

    let instances = vec![
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "Int".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "eq_int".to_string(),
            implementation_style: TraitImplementationStyle::ExplicitBuiltin,
            derived_via: None,
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "Bool".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "CurrentProgram".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "CurrentReceipt".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "LedgerState".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "File".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "Socket".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Eq".to_string(),
            for_type: "Capability".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_eq".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalEquality,
            derived_via: Some("derive(Eq)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "Int".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "print_int".to_string(),
            implementation_style: TraitImplementationStyle::ExplicitBuiltin,
            derived_via: None,
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "Bool".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "CurrentProgram".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "CurrentReceipt".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "LedgerState".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "File".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "Socket".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
        TraitInstance {
            trait_name: "Print".to_string(),
            for_type: "Capability".to_string(),
            owner_root: "lyralang/".to_string(),
            method_target: "canonical_print".to_string(),
            implementation_style: TraitImplementationStyle::DefaultCanonicalPrint,
            derived_via: Some("derive(Print)".to_string()),
        },
    ];

    let derive_expansions = vec![
        DerivedExpansion {
            macro_name: "derive(Eq)".to_string(),
            generated_instances: vec![
                "Eq:Bool".to_string(),
                "Eq:CurrentProgram".to_string(),
                "Eq:CurrentReceipt".to_string(),
                "Eq:LedgerState".to_string(),
                "Eq:File".to_string(),
                "Eq:Socket".to_string(),
                "Eq:Capability".to_string(),
            ],
        },
        DerivedExpansion {
            macro_name: "derive(Print)".to_string(),
            generated_instances: vec![
                "Print:Bool".to_string(),
                "Print:CurrentProgram".to_string(),
                "Print:CurrentReceipt".to_string(),
                "Print:LedgerState".to_string(),
                "Print:File".to_string(),
                "Print:Socket".to_string(),
                "Print:Capability".to_string(),
            ],
        },
    ];

    TraitRegistry {
        version: "lyralang-traits-v1".to_string(),
        traits,
        instances,
        derive_expansions,
    }
}

/// Validates the default seed registry.
#[must_use]
pub fn validate_seed_registry() -> TraitCheckOutput {
    let registry = seed_registry();
    match registry.validate() {
        Ok(()) => TraitCheckOutput {
            registry: Some(registry),
            errors: Vec::new(),
        },
        Err(error) => TraitCheckOutput {
            registry: None,
            errors: vec![error],
        },
    }
}

fn type_owner_root(canonical_type: &str) -> &str {
    match canonical_type {
        "Unit"
        | "Bool"
        | "Int"
        | "Nat"
        | "Rational"
        | "CurrentProgram"
        | "CurrentReceipt"
        | "LedgerState"
        | "File"
        | "Socket"
        | "Capability" => "lyralang/",
        _ => "external/unknown",
    }
}
