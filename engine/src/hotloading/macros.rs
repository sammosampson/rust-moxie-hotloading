#[macro_export]
macro_rules! zod_hotload_client {
    (
        [$initialisation:expr]
        [$state_ty:ty]
    ) => {        
        #[no_mangle]
        pub extern "C" fn initialise<'a>() -> *mut engine::hotloading::HotLoadableApplicationState<$state_ty> {
            let application_state  = engine::hotloading::HotLoadableApplicationState {
                state: <$state_ty>::default(),
                application: $initialisation(<$state_ty>::default())
            };

            Box::into_raw(Box::new(application_state))
        }

        #[no_mangle]
        pub unsafe extern "C" fn update(application_state: *mut engine::hotloading::HotLoadableApplicationState<$state_ty>) -> bool {
            if application_state.is_null() {
                panic!("[ FATAL ] game_update: game state is null!");
            }

            let _application_state = &mut *application_state;
            _application_state.state =_application_state.application.run_once();
            true
        }
        
        #[no_mangle]
        pub unsafe extern "C" fn shutdown(application_state: *mut engine::hotloading::HotLoadableApplicationState<$state_ty>) {
            std::ptr::drop_in_place(application_state);
            std::alloc::dealloc(application_state as *mut u8, std::alloc::Layout::new::<engine::hotloading::HotLoadableApplicationState<$state_ty>>());
        }

        #[no_mangle]
        pub unsafe extern "C" fn unload(_application_state: *mut engine::hotloading::HotLoadableApplicationState<$state_ty>) {
        }

        #[no_mangle]
        pub unsafe extern "C" fn reload(application_state: *mut engine::hotloading::HotLoadableApplicationState<$state_ty>) {
            println!("reloading app");

            if application_state.is_null() {
                panic!("[ FATAL ] game_update: game state is null!");
            }

            let _application_state = &mut *application_state;
            _application_state.application = $initialisation(_application_state.state.clone());
        }
    }
}