use app_state::*;
use engine::hotloading::*;

fn main() {
    HotLoadableApplication::<TestState>::new("target/debug", "app").run();
}