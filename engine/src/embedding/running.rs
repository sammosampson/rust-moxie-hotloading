
use legion::systems::CommandBuffer;
use moxie::*;
use moxie::runtime::Runtime as MoxieRuntime;
use illicit::*;
use crate::changes::*;
use crate::components::*;
use crate::embedding::*;
use crate::application_state::*;

pub fn create_moxie_runner<TState: State>(
    root_func: impl FnMut() -> RootNode<TState> + 'static,
    state: TState) -> MoxieRunner<TState> {
    MoxieRunner::<TState>::new(root_func, state)
}
pub struct MoxieRunner<TState: State> {
    root_func: Box::<dyn FnMut() -> RootNode<TState>>,
    runtime: MoxieRuntime
}

impl<TState: State> MoxieRunner<TState> {
    fn new(
        mut root_func: impl FnMut() -> RootNode<TState> + 'static,
        default_state: TState) -> Self {
        Self {
            root_func: Box::new(move || {
                Layer::new()
                    .offer(state(|| default_state.clone()).1)
                    .offer(state(|| NodeBuildCache::default()).1)
                    .offer(state(|| SourceBuildChangeState::default()).1)
                    .enter(|| topo::root(|| root_func()))
            }),
            runtime: MoxieRuntime::new()
        }
    }    

    fn run_once(&mut self) -> RootNode<TState> {
        self.runtime.run_once(&mut self.root_func)
    }   
}

#[legion::system(simple)]
#[read_component(Relationship)]
#[write_component(Relationship)]
pub fn run_moxie<TState: State>(
    command_buffer: &mut CommandBuffer,
    #[resource] moxie_runner: &mut MoxieRunner<TState>,
    #[resource] state_repository: &mut StateRepository<TState>, 
    #[resource] relationship_map: &mut RelationshipMap, 
    #[resource] entity_map: &mut EntityMap) {

    let frame_start = std::time::Instant::now();    
    let root_node = moxie_runner.run_once();
    println!("*******************");
    println!("frame time: {:?}", std::time::Instant::now() - frame_start);
    println!("*******************");

    let mut maps = SourceBuildMaps {
        entity_map,
        relationship_map
    };

    let (changes, state_snapshot) = root_node.changes();
    changes.apply(command_buffer, &mut maps);
    state_repository.set(state_snapshot);
}