// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-etc-shadow: etc shadow component model.
//!
//! Real implementation: shadows, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shadow {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtcShadowError {
    EmptyShadowName,
    DuplicateShadow(String),
    EmptyEventSubject,
}

impl fmt::Display for EtcShadowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShadowName => write!(f, "shadow name must not be empty"),
            Self::DuplicateShadow(id) => write!(f, "duplicate shadow {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EtcShadowError {}

pub struct EtcShadow {
    pub name: String,
    shadows: BTreeMap<String, Shadow>,
    events: BTreeMap<String, Event>,
}

impl EtcShadow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shadows: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_shadow(&mut self, item: Shadow) -> Result<(), EtcShadowError> {
        if item.name.trim().is_empty() {
            return Err(EtcShadowError::EmptyShadowName);
        }
        let id = item.id.clone();
        if self.shadows.contains_key(&id) {
            return Err(EtcShadowError::DuplicateShadow(id.clone()));
        }
        self.shadows.insert(id, item);
        Ok(())
    }

    pub fn shadow_count(&self) -> usize {
        self.shadows.len()
    }

    pub fn get_shadow(&self, id: &str) -> Option<&Shadow> {
        self.shadows.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EtcShadowError> {
        if event.subject.trim().is_empty() {
            return Err(EtcShadowError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shadow(id: &str, name: &str) -> Shadow {
        Shadow {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EtcShadow::new("EtcShadow");
        assert_eq!(m.name, "EtcShadow");
        assert_eq!(m.shadow_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_shadow() {
        let mut m = EtcShadow::new("EtcShadow");
        m.add_shadow(sample_shadow("a1", "Alpha")).unwrap();
        assert_eq!(m.shadow_count(), 1);
        assert!(m.get_shadow("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EtcShadow::new("EtcShadow");
        assert!(m.add_shadow(sample_shadow("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EtcShadow::new("EtcShadow");
        m.add_shadow(sample_shadow("x", "One")).unwrap();
        assert!(m.add_shadow(sample_shadow("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EtcShadow::new("EtcShadow");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
