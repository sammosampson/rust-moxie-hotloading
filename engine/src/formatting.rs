use crate::components::*;
use legion::serialize::*;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl Pretty for legion::World {
    fn to_pretty(&mut self) {
        let mut registry = legion::Registry::<String>::default();
        registry.on_unknown(UnknownType::Ignore);
        registry.register::<Relationship>("Relationship".to_string());
        registry.register::<Root>("Root".to_string());
        registry.register::<Group>("Group".to_string());
        registry.register::<Radius>("Radius".to_string());
        registry.register::<StrokeWidth>("StrokeWidth".to_string());
        let json = serde_json::to_value(self.as_serializable(legion::passthrough(), &registry)).unwrap();
        println!("{:#}", json);
    }
}

pub fn log_world_view(world: &mut legion::World, _: &mut legion::Resources) {
    world.to_pretty();
}