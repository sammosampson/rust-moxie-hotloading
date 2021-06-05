mod root;
use app_state::TestState;
use engine::application::Application;
use engine::*;
use crate::root::app_root;

zod_hotload_client!(
    [initialise_application] 
    [TestState]
);

fn initialise_application(state: TestState) -> Application<TestState> {
    Application::initialise(|| app_root(), state)
}