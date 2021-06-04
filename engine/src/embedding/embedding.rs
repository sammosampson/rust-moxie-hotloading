use std::{fmt::Debug, vec};
use std::hash::Hash;
use legion::systems::CommandBuffer;
use moxie::Key;
use illicit::*;
use crate::changes::*;
use crate::components::*;
use super::nodes::*;

element! {
    <vertical_stack>
    [LayoutContent::vertical()]
}

element! {
    <circle>
    [Renderable::circle()]
    attributes {
        radius(u16)
        stroke_width(u16)
    }
}