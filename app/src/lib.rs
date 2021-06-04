mod root;
use std::alloc::Layout;
use app_state::*;
use engine::{hotloading::*, application::*};
use root::*;

#[no_mangle]
pub extern "C" fn initialise<'a>() -> *mut HotLoadableApplicationState<TestState> {
    let application_state  = HotLoadableApplicationState {
        state: TestState::default(),
        application: initialise_application(TestState::default())
    };

    Box::into_raw(Box::new(application_state))
}

#[no_mangle]
pub unsafe extern "C" fn update(application_state: *mut HotLoadableApplicationState<TestState>) -> bool {
    if application_state.is_null() {
        panic!("[ FATAL ] game_update: game state is null!");
    }

    let _application_state = &mut *application_state;
    _application_state.state =_application_state.application.run_once();
    true
}

#[no_mangle]
pub unsafe extern "C" fn shutdown(application_state: *mut HotLoadableApplicationState<TestState>) {
    std::ptr::drop_in_place(application_state);
    std::alloc::dealloc(application_state as *mut u8, Layout::new::<HotLoadableApplicationState<TestState>>());
}

#[no_mangle]
pub unsafe extern "C" fn unload(_application_state: *mut HotLoadableApplicationState<TestState>) {
}

#[no_mangle]
pub unsafe extern "C" fn reload(application_state: *mut HotLoadableApplicationState<TestState>) {
    println!("reloading app");

    if application_state.is_null() {
        panic!("[ FATAL ] game_update: game state is null!");
    }

    let _application_state = &mut *application_state;
    _application_state.application = initialise_application(_application_state.state.clone());
}

fn initialise_application(state: TestState) -> Application<TestState> {
    Application::initialise(|| app_root(), state)
}
