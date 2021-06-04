use std::collections::HashMap;
use legion::Entity;
use legion::systems::CommandBuffer;
use std::fmt::Debug;
use serde::*;

use crate::changes::SourceBuildMaps;

pub type EntityMap = HashMap<u64, Entity>;

pub fn create_entity_map() -> EntityMap {
    EntityMap::default()
}

pub type RelationshipMap = HashMap<Entity, Relationship>;

pub fn create_relationship_map() -> RelationshipMap {
    RelationshipMap::default()
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Root {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Group {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub previous_sibling: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutType {
    Horizontal,
    Vertical,
    Canvas,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutContent {
    pub layout_type: LayoutType
}

impl LayoutContent {
    pub fn vertical() -> Self {
        Self { layout_type: LayoutType::Vertical }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderType {
    Circle,
    Rectangle,
    Text
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable { 
    pub render_type: RenderType 
}

impl Renderable {
    pub fn circle() -> Self {
        Self { render_type: RenderType::Circle }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Radius {
    pub radius: u16
}

impl From<u16> for Radius {
    fn from(radius: u16) -> Self {
        Self { radius }
    }
} 

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokeWidth {
    pub width: u16
}

impl From<u16> for StrokeWidth {
    fn from(width: u16) -> Self {
        Self { width }
    }
}

pub trait EntityCreator {
    fn get_or_create<'a, T: Send + Sync + 'static>(&mut self, id: u64, creation_func: impl FnOnce() -> T, maps: &mut SourceBuildMaps<'a>) -> Entity;
    fn add_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>);
    fn remove_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>);
}

impl EntityCreator for CommandBuffer {
    fn get_or_create<'a, T: Send + Sync + 'static>(&mut self, id: u64, creation_func: impl FnOnce() -> T, maps: &mut SourceBuildMaps<'a>) -> Entity {
        match maps.entity_map.get(&id) {
            Some(entity) => *entity,
            None => {
                let relationship = Relationship::default();
                let entity = self.push((relationship, creation_func()));
                maps.entity_map.insert(id, entity);
                maps.relationship_map.insert(entity, relationship);
                entity
            }
        }
    }

    fn add_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>) {
        let child = *maps.entity_map.get(child_id).unwrap();
            
        let mut parent_relationship = *maps.relationship_map.get(&parent).unwrap();
        let mut child_relationship = *maps.relationship_map.get(&child).unwrap();

        if parent_relationship.last_child == None {
            parent_relationship.first_child = Some(child);
        } else {
            let previous_child = parent_relationship.last_child.unwrap();
            child_relationship.previous_sibling = Some(previous_child);
            let mut previous_child_relationship = *maps.relationship_map.get(&previous_child).unwrap();
            previous_child_relationship.next_sibling = Some(child);
            self.add_component(previous_child, previous_child_relationship);
            maps.relationship_map.insert(previous_child, previous_child_relationship);        
        }

        parent_relationship.last_child = Some(child);
        self.add_component(parent, parent_relationship);
        maps.relationship_map.insert(parent, parent_relationship);

        child_relationship.parent = Some(parent);
        self.add_component(child, child_relationship);
        maps.relationship_map.insert(child, child_relationship);
    }

    fn remove_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>) {
        let child = maps.entity_map.remove(child_id).unwrap();
        let child_relationship = maps.relationship_map.remove(&child).unwrap();
        let mut parent_relationship = *maps.relationship_map.get(&parent).unwrap();
    
        if let Some(previous_child) = child_relationship.previous_sibling {
            let mut previous_child_relationship = *maps.relationship_map.get(&previous_child).unwrap();
            previous_child_relationship.next_sibling = child_relationship.next_sibling;
            self.add_component(previous_child, previous_child_relationship);
            maps.relationship_map.insert(previous_child, previous_child_relationship);
        }

        if let Some(next_child) = child_relationship.next_sibling {
            let mut next_child_relationship = *maps.relationship_map.get(&next_child).unwrap();
            next_child_relationship.previous_sibling = child_relationship.previous_sibling;
            self.add_component(next_child, next_child_relationship);
            maps.relationship_map.insert(next_child, next_child_relationship);
        }   
        
        if parent_relationship.first_child.unwrap() == child {
            parent_relationship.first_child = child_relationship.next_sibling;
            self.add_component(parent, parent_relationship);
            maps.relationship_map.insert(parent, parent_relationship);
        }
        
        if parent_relationship.last_child.unwrap() == child {
            parent_relationship.last_child = child_relationship.previous_sibling;
            self.add_component(parent, parent_relationship);
            maps.relationship_map.insert(parent, parent_relationship);
        }
    }
}