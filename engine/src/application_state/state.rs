use std::fmt::Debug;

pub trait State: Debug + Clone + Copy + Default + PartialEq + 'static {}