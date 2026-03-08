//! Shared seed builtin signatures for LyraLang Stage 0.

use std::collections::BTreeMap;

use crate::types::{
    EffectAtom, EffectSet, EvidenceKind, MetaType, ModalKind, ResourceType, Type,
    TypeScheme, TypeVariableId,
};

/// Minimal callable metadata required by the seed effect checker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallableSignature {
    /// Positional arity.
    pub arity: usize,
    /// Latent effects of the callable.
    pub effects: EffectSet,
}

/// Linear ownership behavior for a builtin callable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinearBuiltinBehavior {
    /// The callable neither consumes nor produces linear resources.
    None,
    /// The callable constructs and returns a fresh resource.
    Produce(ResourceType),
    /// The callable consumes the resource at the given argument index.
    Consume { index: usize, resource: ResourceType },
    /// The callable forwards the resource at the given argument index.
    Forward { index: usize },
}

/// Minimal callable metadata required by the seed linear checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LinearCallableSignature {
    /// Positional arity.
    pub arity: usize,
    /// Builtin linear-resource behavior.
    pub behavior: LinearBuiltinBehavior,
}

/// Modal behavior for a builtin callable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalBuiltinBehavior {
    /// Wraps a base value in the given modality.
    Introduce { modality: ModalKind },
    /// Promotes a modal value using explicit evidence.
    Promote {
        /// Required source modality.
        from: ModalKind,
        /// Target modality.
        to: ModalKind,
        /// Required evidence token kind.
        evidence: EvidenceKind,
    },
    /// Eliminates a modal wrapper back to the payload.
    Eliminate { from: ModalKind },
}

/// Minimal callable metadata required by the seed modal checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalCallableSignature {
    /// Positional arity.
    pub arity: usize,
    /// Modal behavior.
    pub behavior: ModalBuiltinBehavior,
}

/// Temporal operator behavior for a builtin callable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TemporalBuiltinBehavior {
    /// Unary `always` operator.
    Always,
    /// Unary `eventually` operator.
    Eventually,
    /// Binary `until` operator.
    Until,
    /// Binary `since` operator.
    Since,
}

/// Minimal callable metadata required by the seed temporal checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TemporalCallableSignature {
    /// Positional arity.
    pub arity: usize,
    /// Temporal behavior.
    pub behavior: TemporalBuiltinBehavior,
}

/// Returns the deterministic Stage 0 builtin type environment.
pub(crate) fn builtin_type_environment() -> BTreeMap<String, TypeScheme> {
    let mut environment = BTreeMap::new();
    let generic = TypeVariableId(10_000);
    let modal_generic = TypeVariableId(10_100);
    let eq_generic = TypeVariableId(10_200);
    let print_generic = TypeVariableId(10_201);
    let option_generic = TypeVariableId(10_300);
    let task_generic = TypeVariableId(10_350);
    let ok_generic = TypeVariableId(10_301);
    let ok_error_generic = TypeVariableId(10_302);
    let err_generic = TypeVariableId(10_303);

    environment.insert(
        "id".to_string(),
        TypeScheme {
            variables: vec![generic],
            body: Type::function(
                vec![Type::Variable(generic)],
                EffectSet::pure(),
                Type::Variable(generic),
            ),
        },
    );
    environment.insert(
        "add".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::pure(),
            Type::int(),
        )),
    );
    environment.insert(
        "eq_int".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::pure(),
            Type::bool(),
        )),
    );
    environment.insert(
        "eq".to_string(),
        TypeScheme {
            variables: vec![eq_generic],
            body: Type::function(
                vec![Type::Variable(eq_generic), Type::Variable(eq_generic)],
                EffectSet::pure(),
                Type::bool(),
            ),
        },
    );
    environment.insert(
        "not".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::bool()],
            EffectSet::pure(),
            Type::bool(),
        )),
    );
    environment.insert(
        "nat_succ".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::nat()],
            EffectSet::pure(),
            Type::nat(),
        )),
    );
    environment.insert(
        "ratio_from_ints".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::pure(),
            Type::rational(),
        )),
    );
    environment.insert(
        "some".to_string(),
        TypeScheme {
            variables: vec![option_generic],
            body: Type::function(
                vec![Type::Variable(option_generic)],
                EffectSet::pure(),
                Type::option(Type::Variable(option_generic)),
            ),
        },
    );
    environment.insert(
        "none_int".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::option(Type::int()))),
    );
    environment.insert(
        "ok".to_string(),
        TypeScheme {
            variables: vec![ok_generic, ok_error_generic],
            body: Type::function(
                vec![Type::Variable(ok_generic)],
                EffectSet::pure(),
                Type::result(Type::Variable(ok_generic), Type::Variable(ok_error_generic)),
            ),
        },
    );
    environment.insert(
        "err".to_string(),
        TypeScheme {
            variables: vec![generic, err_generic],
            body: Type::function(
                vec![Type::Variable(err_generic)],
                EffectSet::pure(),
                Type::result(Type::Variable(generic), Type::Variable(err_generic)),
            ),
        },
    );
    environment.insert(
        "io_error".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::error("IoFailure"))),
    );
    environment.insert(
        "config_error".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::error("ConfigMissing"))),
    );
    environment.insert(
        "divide_by_zero_error".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::error("DivideByZero"))),
    );
    environment.insert(
        "capture_trace".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::stack_trace())),
    );
    environment.insert(
        "fallible_add".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::pure(),
            Type::result(Type::int(), Type::error("IoFailure")),
        )),
    );
    environment.insert(
        "fallible_divide".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::pure(),
            Type::result(Type::int(), Type::error("DivideByZero")),
        )),
    );
    environment.insert(
        "lookup_flag".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int()],
            EffectSet::pure(),
            Type::option(Type::int()),
        )),
    );
    environment.insert(
        "panic".to_string(),
        TypeScheme::mono(Type::function(Vec::new(), EffectSet::pure(), Type::unit())),
    );
    environment.insert(
        "spawn".to_string(),
        TypeScheme {
            variables: vec![task_generic],
            body: Type::function(
                vec![Type::Variable(task_generic)],
                EffectSet::pure(),
                Type::task(Type::Variable(task_generic)),
            ),
        },
    );
    environment.insert(
        "join".to_string(),
        TypeScheme {
            variables: vec![task_generic],
            body: Type::function(
                vec![Type::task(Type::Variable(task_generic))],
                EffectSet::pure(),
                Type::Variable(task_generic),
            ),
        },
    );
    environment.insert(
        "select".to_string(),
        TypeScheme {
            variables: vec![task_generic],
            body: Type::function(
                vec![
                    Type::task(Type::Variable(task_generic)),
                    Type::task(Type::Variable(task_generic)),
                ],
                EffectSet::pure(),
                Type::Variable(task_generic),
            ),
        },
    );
    environment.insert(
        "channel_int".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::pure(),
            Type::channel(Type::int()),
        )),
    );
    environment.insert(
        "send_int".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::channel(Type::int()), Type::int()],
            EffectSet::pure(),
            Type::unit(),
        )),
    );
    environment.insert(
        "recv_int".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::channel(Type::int())],
            EffectSet::pure(),
            Type::int(),
        )),
    );
    environment.insert(
        "print_int".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int()],
            EffectSet::single(EffectAtom::Io),
            Type::unit(),
        )),
    );
    environment.insert(
        "print".to_string(),
        TypeScheme {
            variables: vec![print_generic],
            body: Type::function(
                vec![Type::Variable(print_generic)],
                EffectSet::single(EffectAtom::Io),
                Type::unit(),
            ),
        },
    );
    environment.insert(
        "read_clock".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Time),
            Type::int(),
        )),
    );
    environment.insert(
        "entropy_u64".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Entropy),
            Type::int(),
        )),
    );
    environment.insert(
        "touch_state".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int()],
            EffectSet::single_linear(EffectAtom::State),
            Type::int(),
        )),
    );
    environment.insert(
        "prove_eq_int".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int(), Type::int()],
            EffectSet::single(EffectAtom::Proof),
            Type::bool(),
        )),
    );
    environment.insert(
        "open_file".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Io),
            Type::file(),
        )),
    );
    environment.insert(
        "close_file".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::file()],
            EffectSet::single(EffectAtom::Io),
            Type::unit(),
        )),
    );
    environment.insert(
        "open_socket".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Io),
            Type::socket(),
        )),
    );
    environment.insert(
        "close_socket".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::socket()],
            EffectSet::single(EffectAtom::Io),
            Type::unit(),
        )),
    );
    environment.insert(
        "grant_capability".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::int()],
            EffectSet::single(EffectAtom::Proof),
            Type::capability(),
        )),
    );
    environment.insert(
        "consume_capability".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::capability()],
            EffectSet::single_linear(EffectAtom::State),
            Type::unit(),
        )),
    );

    environment.insert(
        "observation_evidence".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Proof),
            Type::evidence(EvidenceKind::Observation),
        )),
    );
    environment.insert(
        "proof_evidence".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Proof),
            Type::evidence(EvidenceKind::Proof),
        )),
    );
    environment.insert(
        "necessity_evidence".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Proof),
            Type::evidence(EvidenceKind::Necessity),
        )),
    );
    environment.insert(
        "possibility_evidence".to_string(),
        TypeScheme::mono(Type::function(
            Vec::new(),
            EffectSet::single(EffectAtom::Proof),
            Type::evidence(EvidenceKind::Possibility),
        )),
    );
    environment.insert(
        "assume_unknown".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![Type::Variable(modal_generic)],
                EffectSet::pure(),
                Type::unknown(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "promote_unknown_to_hypothesis".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![
                    Type::unknown(Type::Variable(modal_generic)),
                    Type::evidence(EvidenceKind::Observation),
                ],
                EffectSet::single(EffectAtom::Proof),
                Type::hypothesis(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "promote_hypothesis_to_fact".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![
                    Type::hypothesis(Type::Variable(modal_generic)),
                    Type::evidence(EvidenceKind::Proof),
                ],
                EffectSet::single(EffectAtom::Proof),
                Type::fact(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "promote_fact_to_necessary".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![
                    Type::fact(Type::Variable(modal_generic)),
                    Type::evidence(EvidenceKind::Necessity),
                ],
                EffectSet::single(EffectAtom::Proof),
                Type::necessary(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "promote_unknown_to_possible".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![
                    Type::unknown(Type::Variable(modal_generic)),
                    Type::evidence(EvidenceKind::Possibility),
                ],
                EffectSet::single(EffectAtom::Proof),
                Type::possible(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "promote_possible_to_fact".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![
                    Type::possible(Type::Variable(modal_generic)),
                    Type::evidence(EvidenceKind::Proof),
                ],
                EffectSet::single(EffectAtom::Proof),
                Type::fact(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "weaken_necessary_to_fact".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![Type::necessary(Type::Variable(modal_generic))],
                EffectSet::pure(),
                Type::fact(Type::Variable(modal_generic)),
            ),
        },
    );
    environment.insert(
        "reveal_fact".to_string(),
        TypeScheme {
            variables: vec![modal_generic],
            body: Type::function(
                vec![Type::fact(Type::Variable(modal_generic))],
                EffectSet::pure(),
                Type::Variable(modal_generic),
            ),
        },
    );
    environment.insert(
        "always".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::bool()],
            EffectSet::pure(),
            Type::temporal(Type::bool()),
        )),
    );
    environment.insert(
        "eventually".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::bool()],
            EffectSet::pure(),
            Type::temporal(Type::bool()),
        )),
    );
    environment.insert(
        "until".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::temporal(Type::bool()), Type::temporal(Type::bool())],
            EffectSet::pure(),
            Type::temporal(Type::bool()),
        )),
    );
    environment.insert(
        "since".to_string(),
        TypeScheme::mono(Type::function(
            vec![Type::temporal(Type::bool()), Type::temporal(Type::bool())],
            EffectSet::pure(),
            Type::temporal(Type::bool()),
        )),
    );

    environment
}

/// Returns callable metadata derived from the shared builtin type environment.
pub(crate) fn builtin_callable_environment() -> BTreeMap<String, CallableSignature> {
    builtin_type_environment()
        .into_iter()
        .map(|(name, scheme)| {
            let callable = match scheme.body {
                Type::Function(function) => CallableSignature {
                    arity: function.parameters.len(),
                    effects: function.effects,
                },
                _ => CallableSignature {
                    arity: 0,
                    effects: EffectSet::pure(),
                },
            };
            (name, callable)
        })
        .collect()
}

/// Returns the deterministic Stage 0 builtin ownership contracts.
pub(crate) fn builtin_linear_environment() -> BTreeMap<String, LinearCallableSignature> {
    use LinearBuiltinBehavior::{Consume, Forward, None, Produce};

    BTreeMap::from([
        (
            "id".to_string(),
            LinearCallableSignature {
                arity: 1,
                behavior: Forward { index: 0 },
            },
        ),
        (
            "add".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "eq_int".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "eq".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "not".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "nat_succ".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "ratio_from_ints".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "some".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "none_int".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "ok".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "err".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "io_error".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "config_error".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "divide_by_zero_error".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "capture_trace".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "fallible_add".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "fallible_divide".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "lookup_flag".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "panic".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "spawn".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "join".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "select".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "channel_int".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "send_int".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "recv_int".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "print_int".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "print".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "read_clock".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "entropy_u64".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "touch_state".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "prove_eq_int".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "observation_evidence".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "proof_evidence".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "necessity_evidence".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "possibility_evidence".to_string(),
            LinearCallableSignature { arity: 0, behavior: None },
        ),
        (
            "assume_unknown".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "promote_unknown_to_hypothesis".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "promote_hypothesis_to_fact".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "promote_fact_to_necessary".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "promote_unknown_to_possible".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "promote_possible_to_fact".to_string(),
            LinearCallableSignature { arity: 2, behavior: None },
        ),
        (
            "weaken_necessary_to_fact".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "reveal_fact".to_string(),
            LinearCallableSignature { arity: 1, behavior: None },
        ),
        (
            "open_file".to_string(),
            LinearCallableSignature {
                arity: 0,
                behavior: Produce(ResourceType::File),
            },
        ),
        (
            "close_file".to_string(),
            LinearCallableSignature {
                arity: 1,
                behavior: Consume {
                    index: 0,
                    resource: ResourceType::File,
                },
            },
        ),
        (
            "open_socket".to_string(),
            LinearCallableSignature {
                arity: 0,
                behavior: Produce(ResourceType::Socket),
            },
        ),
        (
            "close_socket".to_string(),
            LinearCallableSignature {
                arity: 1,
                behavior: Consume {
                    index: 0,
                    resource: ResourceType::Socket,
                },
            },
        ),
        (
            "grant_capability".to_string(),
            LinearCallableSignature {
                arity: 1,
                behavior: Produce(ResourceType::Capability),
            },
        ),
        (
            "consume_capability".to_string(),
            LinearCallableSignature {
                arity: 1,
                behavior: Consume {
                    index: 0,
                    resource: ResourceType::Capability,
                },
            },
        ),
    ])
}

/// Returns the deterministic Stage 0 modal contracts.
pub(crate) fn builtin_modal_environment() -> BTreeMap<String, ModalCallableSignature> {
    use ModalBuiltinBehavior::{Eliminate, Introduce, Promote};

    BTreeMap::from([
        (
            "assume_unknown".to_string(),
            ModalCallableSignature {
                arity: 1,
                behavior: Introduce {
                    modality: ModalKind::Unknown,
                },
            },
        ),
        (
            "promote_unknown_to_hypothesis".to_string(),
            ModalCallableSignature {
                arity: 2,
                behavior: Promote {
                    from: ModalKind::Unknown,
                    to: ModalKind::Hypothesis,
                    evidence: EvidenceKind::Observation,
                },
            },
        ),
        (
            "promote_hypothesis_to_fact".to_string(),
            ModalCallableSignature {
                arity: 2,
                behavior: Promote {
                    from: ModalKind::Hypothesis,
                    to: ModalKind::Fact,
                    evidence: EvidenceKind::Proof,
                },
            },
        ),
        (
            "promote_fact_to_necessary".to_string(),
            ModalCallableSignature {
                arity: 2,
                behavior: Promote {
                    from: ModalKind::Fact,
                    to: ModalKind::Necessary,
                    evidence: EvidenceKind::Necessity,
                },
            },
        ),
        (
            "promote_unknown_to_possible".to_string(),
            ModalCallableSignature {
                arity: 2,
                behavior: Promote {
                    from: ModalKind::Unknown,
                    to: ModalKind::Possible,
                    evidence: EvidenceKind::Possibility,
                },
            },
        ),
        (
            "promote_possible_to_fact".to_string(),
            ModalCallableSignature {
                arity: 2,
                behavior: Promote {
                    from: ModalKind::Possible,
                    to: ModalKind::Fact,
                    evidence: EvidenceKind::Proof,
                },
            },
        ),
        (
            "weaken_necessary_to_fact".to_string(),
            ModalCallableSignature {
                arity: 1,
                behavior: Eliminate {
                    from: ModalKind::Necessary,
                },
            },
        ),
        (
            "reveal_fact".to_string(),
            ModalCallableSignature {
                arity: 1,
                behavior: Eliminate { from: ModalKind::Fact },
            },
        ),
    ])
}

/// Returns the deterministic Stage 0 temporal operator contracts.
pub(crate) fn builtin_temporal_environment() -> BTreeMap<String, TemporalCallableSignature> {
    use TemporalBuiltinBehavior::{Always, Eventually, Since, Until};

    BTreeMap::from([
        (
            "always".to_string(),
            TemporalCallableSignature {
                arity: 1,
                behavior: Always,
            },
        ),
        (
            "eventually".to_string(),
            TemporalCallableSignature {
                arity: 1,
                behavior: Eventually,
            },
        ),
        (
            "until".to_string(),
            TemporalCallableSignature {
                arity: 2,
                behavior: Until,
            },
        ),
        (
            "since".to_string(),
            TemporalCallableSignature {
                arity: 2,
                behavior: Since,
            },
        ),
    ])
}

/// Returns the canonical return type for a supported self-reference primitive name.
#[must_use]
pub(crate) fn self_reference_type(name: &str) -> Option<Type> {
    match name {
        "current_program" => Some(Type::Meta(MetaType::CurrentProgram)),
        "current_receipt" => Some(Type::Meta(MetaType::CurrentReceipt)),
        "ledger_state" => Some(Type::Meta(MetaType::LedgerState)),
        _ => None,
    }
}
