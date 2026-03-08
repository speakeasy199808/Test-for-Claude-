//! Effect-set primitives for the Stage 0 LyraLang type kernel.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Atomic effects reserved by the Stage 0 kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectAtom {
    /// External input/output.
    Io,
    /// State mutation.
    State,
    /// Time-sensitive interaction.
    Time,
    /// Deterministic entropy interaction.
    Entropy,
    /// Proof construction or proof checking.
    Proof,
}

impl EffectAtom {
    /// Returns the canonical textual spelling of the effect atom.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Io => "io",
            Self::State => "state",
            Self::Time => "time",
            Self::Entropy => "entropy",
            Self::Proof => "proof",
        }
    }
}

/// A deterministic Stage 0 effect set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EffectSet {
    /// Persistent effect atoms in canonical sorted order.
    pub atoms: BTreeSet<EffectAtom>,
    /// Linear effect atoms in canonical sorted order.
    pub linear_atoms: BTreeSet<EffectAtom>,
}

impl EffectSet {
    /// Returns the canonical pure effect set.
    #[must_use]
    pub fn pure() -> Self {
        Self::default()
    }

    /// Returns a singleton persistent effect set.
    #[must_use]
    pub fn single(atom: EffectAtom) -> Self {
        let mut set = Self::pure();
        set.insert(atom);
        set
    }

    /// Returns a singleton linear effect set.
    #[must_use]
    pub fn single_linear(atom: EffectAtom) -> Self {
        let mut set = Self::pure();
        set.insert_linear(atom);
        set
    }

    /// Inserts a persistent atom into the set.
    pub fn insert(&mut self, atom: EffectAtom) {
        self.atoms.insert(atom);
    }

    /// Inserts a linear atom into the set.
    pub fn insert_linear(&mut self, atom: EffectAtom) {
        self.linear_atoms.insert(atom);
    }

    /// Returns `true` when the effect set is empty.
    #[must_use]
    pub fn is_pure(&self) -> bool {
        self.atoms.is_empty() && self.linear_atoms.is_empty()
    }

    /// Returns `true` when `self` is a sub-effect of `other`.
    #[must_use]
    pub fn is_sub_effect_of(&self, other: &Self) -> bool {
        self.atoms.is_subset(&other.atoms) && self.linear_atoms.is_subset(&other.linear_atoms)
    }

    /// Returns the obligations present in `self` but missing from `other`.
    #[must_use]
    pub fn missing_from(&self, other: &Self) -> Self {
        Self {
            atoms: self.atoms.difference(&other.atoms).copied().collect(),
            linear_atoms: self
                .linear_atoms
                .difference(&other.linear_atoms)
                .copied()
                .collect(),
        }
    }

    /// Returns the canonical entry strings for the set.
    #[must_use]
    pub fn canonical_entries(&self) -> Vec<String> {
        let mut persistent: Vec<String> = self
            .atoms
            .iter()
            .map(|atom| atom.as_str().to_string())
            .collect();
        persistent.sort();

        let mut linear: Vec<String> = self
            .linear_atoms
            .iter()
            .map(|atom| format!("{}!", atom.as_str()))
            .collect();
        linear.sort();

        persistent.extend(linear);
        persistent
    }

    /// Returns a canonical string for the effect set.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        if self.is_pure() {
            return "pure".to_string();
        }

        self.canonical_entries().join(",")
    }

    /// Returns the deterministic union of two effect sets.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut atoms = self.atoms.clone();
        atoms.extend(other.atoms.iter().copied());

        let mut linear_atoms = self.linear_atoms.clone();
        linear_atoms.extend(other.linear_atoms.iter().copied());

        Self { atoms, linear_atoms }
    }
}
