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
        stroke_colour(Colour)
        content(String)
    }
}