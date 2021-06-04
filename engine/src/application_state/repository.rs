use super::state::State;

pub fn create_state_repository<TState: State>() -> StateRepository<TState> {
    StateRepository::default()
}


#[derive(Default)]
pub struct StateRepository<TState: State>(TState);

impl<TState: State> StateRepository<TState> {
    pub fn get(&self) -> TState {
        self.0.clone()
    }
    
    pub fn set(&mut self, to_set: TState) {
        self.0 = to_set;
    }
}

trait StateSnapshotter {
    type TState: State;
}