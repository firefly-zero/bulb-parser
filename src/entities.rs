use crate::*;
use alloc::{boxed::Box, vec::Vec};

pub struct Entity<'a, T> {
    /// The human-readable entity ID as defined in the file.
    id: &'a str,
    /// Row number where the entity is first referenced.
    first_ref: usize,
    /// Entity value. If None, the entity was referenced but definition not found yet.
    value: Option<T>,
}

pub struct Entities<'a, T> {
    items: Vec<Entity<'a, T>>,
}

impl<'a, T> Entities<'a, T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn reference(&mut self, id: &'a str, row: usize) -> usize {
        for (i, entity) in self.items.iter().enumerate() {
            if entity.id == id {
                return i;
            }
        }
        let entity = Entity {
            id,
            first_ref: row,
            value: None,
        };
        self.items.push(entity);
        self.items.len() - 1
    }

    pub fn is_defined(&self, id: &str) -> bool {
        for entity in &self.items {
            if entity.value.is_some() && entity.id == id {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity<T>> {
        self.items.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn finalize(self, kind: ErrKind) -> Result<Box<[T]>, Err> {
        let mut result: Vec<T> = Vec::new();
        for entity in self.items {
            let Some(val) = entity.value else {
                return Err(Err::new(kind, entity.first_ref));
            };
            result.push(val);
        }
        Ok(result.into_boxed_slice())
    }
}
