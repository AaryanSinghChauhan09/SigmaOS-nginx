// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-nginx: nginx component model.
//!
//! Real implementation: nginx_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NginxItem {
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
pub enum NginxError {
    EmptyNginxItemName,
    DuplicateNginxItem(String),
    EmptyEventSubject,
}

impl fmt::Display for NginxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNginxItemName => write!(f, "nginx_item name must not be empty"),
            Self::DuplicateNginxItem(id) => write!(f, "duplicate nginx_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NginxError {}

pub struct Nginx {
    pub name: String,
    nginx_items: BTreeMap<String, NginxItem>,
    events: BTreeMap<String, Event>,
}

impl Nginx {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nginx_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_nginx_item(&mut self, item: NginxItem) -> Result<(), NginxError> {
        if item.name.trim().is_empty() {
            return Err(NginxError::EmptyNginxItemName);
        }
        let id = item.id.clone();
        if self.nginx_items.contains_key(&id) {
            return Err(NginxError::DuplicateNginxItem(id.clone()));
        }
        self.nginx_items.insert(id, item);
        Ok(())
    }

    pub fn nginx_item_count(&self) -> usize {
        self.nginx_items.len()
    }

    pub fn get_nginx_item(&self, id: &str) -> Option<&NginxItem> {
        self.nginx_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NginxError> {
        if event.subject.trim().is_empty() {
            return Err(NginxError::EmptyEventSubject);
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

    fn sample_nginx_item(id: &str, name: &str) -> NginxItem {
        NginxItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Nginx::new("Nginx");
        assert_eq!(m.name, "Nginx");
        assert_eq!(m.nginx_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_nginx_item() {
        let mut m = Nginx::new("Nginx");
        m.add_nginx_item(sample_nginx_item("a1", "Alpha")).unwrap();
        assert_eq!(m.nginx_item_count(), 1);
        assert!(m.get_nginx_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Nginx::new("Nginx");
        assert!(m.add_nginx_item(sample_nginx_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Nginx::new("Nginx");
        m.add_nginx_item(sample_nginx_item("x", "One")).unwrap();
        assert!(m.add_nginx_item(sample_nginx_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Nginx::new("Nginx");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
