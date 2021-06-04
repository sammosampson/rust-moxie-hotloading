use std::marker::PhantomData;
use illicit::from_env;
use legion::systems::*;
use moxie::*;
use crate::{changes::*, components::*, application_state::*};
use super::nodes::*;

#[derive(Default)]
pub struct RootNode<TState: State> {
    id: u64,
    changes: SourceBuildChanges,
    state_snapshot: TState
}

impl<TState: State> RootNode<TState> {
    pub fn new() -> Self {
        Self {
            id: generate_node_id(),
            changes: SourceBuildChanges::default(),
            state_snapshot: TState::default()
        }
    }

    pub fn changes(self) -> (SourceBuildChanges, TState) {
        (self.changes, self.state_snapshot)
    }

    #[from_env(changes: &Key<SourceBuildChangeState>)]
    #[from_env(app_state: &Key<TState>)]
    fn collect_state(&mut self) {
        self.changes = changes.commit();
        println!("{:?}", app_state);
        self.state_snapshot = **app_state;
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct RootBuilder<TState: State> {
    children: Vec<u64>,
    _marker: PhantomData<TState>
}

impl<TState: State> RootBuilder<TState> {
    pub fn new() -> Self {
        Self {
            children: vec!(),
            _marker: PhantomData::<TState>::default()
        }
    }

    #[from_env(state: &Key<SourceBuildChangeState>)]
    #[from_env(cache: &Key<NodeBuildCache>)]
    pub fn build(&self) -> RootNode<TState> {
        let mut node = RootNode::<TState>::new();

        moxie::cache(
            self,
            |_| {    
                state.push_change(RootChange::between(node.id, self, &self.previous_version(node.id)));
                cache.cache_current_node_revision(node.id, self.clone());
            }
        );

        if state.has_changed() {
            node.collect_state();
        }

        cache.clear_up();

        node
    }
    
    #[from_env(cache: &Key<NodeBuildCache>)]
    fn previous_version(&self, node_id: u64) -> Self {
        cache.get_previous_node_revision(node_id, RootBuilder::default())
    }

    pub fn child(mut self, mut child: Node) -> Self {
        if child.is_group { 
            self.children.append(&mut child.group_children)
        } else {
            self.children.push(child.id);
        }
        self
    }
}


#[derive(Default, Debug, Clone)]
pub struct RootChange<TState: State> {
    node_id: u64,
    child_changes: NodeChanges::<u64>,
    _marker: PhantomData<TState>
}

impl<TState: State> RootChange<TState> {
    pub fn between(node_id: u64, current: &RootBuilder<TState>, previous: &RootBuilder<TState>) -> Self {
        Self {
            node_id,
            child_changes: NodeChanges::<u64>::between(&current.children, &previous.children),
            _marker: PhantomData::<TState>::default()
        }
    }
}

impl<TState: State> SourceBuildChange for RootChange<TState> {
    fn apply<'a>(&self, command_buffer: &mut CommandBuffer, maps: &mut SourceBuildMaps<'a>) {        
        let parent = command_buffer.get_or_create(self.node_id, || Root::default(), maps);
        self.child_changes.process_additions(&mut |child_id| command_buffer.add_child(parent, child_id, maps));    
        self.child_changes.process_removals(&mut |child_id| command_buffer.remove_child(parent, child_id, maps));
    }
}
