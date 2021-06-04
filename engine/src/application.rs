
use std::marker::PhantomData;
use crate::components::*;
use crate::formatting::*;
use crate::embedding::*;
use crate::application_state::*;

pub struct Application<TState: State> {
    ecs_world: legion::World,
    ecs_resources: legion::Resources,
    ecs_schedule: legion::Schedule,
    _marker: PhantomData<TState>
}

impl<TState: State> Application<TState> {
    pub fn initialise (
        root_func: impl FnMut() -> RootNode<TState> + 'static,
        state: TState) -> Self {
            
        let ecs_world = legion::World::default();
        let mut ecs_resources = legion::Resources::default();
        ecs_resources.insert(create_moxie_runner(root_func, state));
        ecs_resources.insert(create_state_repository::<TState>());
        ecs_resources.insert(create_entity_map());
        ecs_resources.insert(create_relationship_map());

        let ecs_schedule = legion::Schedule::
            builder()
                .add_thread_local(run_moxie_system::<TState>())
                .flush()
                .add_thread_local_fn(log_world_view)
                .flush()
                .build();
            
        Self {
            ecs_world,
            ecs_resources,
            ecs_schedule,
            _marker: PhantomData::<TState>::default()
        } 
    }

    pub fn run_once(&mut self) -> TState {
        self.ecs_schedule.execute(&mut self.ecs_world, &mut self.ecs_resources);
        self.get_state()
    }

    pub fn get_state(&mut self) -> TState {
        let repository = &mut self.ecs_resources.get::<StateRepository<TState>>().unwrap();
        repository.get()
    }
}