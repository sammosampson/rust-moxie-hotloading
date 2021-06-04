use std::sync::atomic::*;
use dyn_cache::local::SharedLocalCache;
use moxie::runtime::Revision;
use super::revisions::Previous;

static ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

pub fn generate_node_id() -> u64 {
    moxie::once(|| ID_GENERATOR.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone)]
pub struct Node {
    pub id: u64,
    pub group_children: Vec<u64>,
    pub is_group: bool
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: generate_node_id(),
            group_children: vec!(),
            is_group: false
        }
    }

    pub fn group_children(&mut self, children: Vec<u64>) {
        self.group_children = children;
        self.is_group = true;
    }

    pub fn build(self) -> Node {
        self
    }
}

pub trait NodeBuilder {
    fn build(&self) -> Node;
}

#[derive(Default, Debug)]
pub struct NodeBuildCache(SharedLocalCache);

impl NodeBuildCache {
    pub fn get_previous_node_revision<T: Clone + 'static>(&self, node_id: u64, default_value: T) -> T {
        self.storage().cache(&node_id, &Revision::previous(), |_| default_value)
    }

    pub fn cache_current_node_revision<T: Clone + 'static>(&self, node_id: u64, to_cache: T) -> T {
        self.storage().cache(&node_id, &Revision::current(), |_| to_cache)
    }

    pub fn clear_up(&self) {
        self.storage().gc()
    }

    fn storage(&self) -> &SharedLocalCache{
        &self.0
    }
}

impl NodeBuilder for Vec<Node> {
    fn build(&self) -> Node {
        let children: Vec::<u64> = self.iter().map(|child| child.id).collect();
        moxie::cache(
            &children,
           |_| {    
                let mut node = Node::new();
                node.group_children(children.clone());
                node
            }
        )
    }
}