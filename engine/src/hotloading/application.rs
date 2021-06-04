
use core::time;
use std::thread;
use crate::hotloading::*;
use crate::application_state::*;
use crate::application::Application;

pub struct HotLoadableApplication<TState: State> {
    app_library: HotReloadableLibrary,
    state: *mut HotLoadableApplicationState<TState>
}

impl<TState: State> HotLoadableApplication<TState> {
    pub fn new(library_folder: &str) -> Self {
        let app_library = HotReloadableLibrary::new(library_folder, "app");
        let state = HotLoadableApplication::create_state(&app_library);
        Self {
            app_library,
            state 
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.update_state() {
                break;
            }
    
            self.reload_app_library_if_changed();
            
            thread::sleep(time::Duration::from_millis(200));
        }
    
        self.shutdown();
    } 

    fn create_state(library: &HotReloadableLibrary) -> *mut HotLoadableApplicationState<TState> {
        library.load_symbol::<fn() -> *mut HotLoadableApplicationState<TState>>("initialise")()
    }

    fn update_state(&self) -> bool {
        self.app_library.load_symbol::<fn(*mut HotLoadableApplicationState<TState>) -> bool>("update")(self.state)
    }

    fn shutdown(&self) {
        self.app_library.load_symbol::<fn(*mut HotLoadableApplicationState<TState>)>("shutdown")(self.state)
    }

    fn unload(&self) {
        self.app_library.load_symbol::<fn(*mut HotLoadableApplicationState<TState>)>("unload")(self.state)
    }

    fn reload(&self) {
        self.app_library.load_symbol::<fn(*mut HotLoadableApplicationState<TState>)>("reload")(self.state)
    }

    fn reload_app_library_if_changed(&mut self) {
        if !self.app_library.has_changed() {
            return;
        }

        self.unload();
        self.app_library.reload();
        self.reload();
    }
}

#[repr(C)]
pub struct HotLoadableApplicationState<TState: State> {
    pub state: TState,
    pub application: Application<TState>
}