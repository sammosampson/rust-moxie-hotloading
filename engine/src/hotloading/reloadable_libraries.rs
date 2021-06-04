

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::{fs, time};

use notify::op::*;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher, RecommendedWatcher};

pub struct HotReloadableLibrary {
    original_path_string: String,
    original_path: PathBuf,
    loaded_path: PathBuf,
    library: Option<libloading::Library>,
    watch_event_receiver: Receiver<RawEvent>,
    _watcher: RecommendedWatcher,
}

impl<'lib> HotReloadableLibrary {
    pub fn new(folder: &str, file_name: &str) -> Self {
        let path_string = construct_library_path_string(folder, file_name);
        let path = to_canonicalized_path(&path_string);
        let (library, loaded_path) = copy_and_load_library(&path_string);
        let (watch_event_receiver, watcher) = watch_path(folder);

        Self {
            original_path_string: path_string,
            original_path: path,
            loaded_path: loaded_path,
            library: Some(library),
            watch_event_receiver,
            _watcher: watcher,
        }
    }

    pub fn load_symbol<Signature>(&self, symbol_name: &str) -> libloading::Symbol<Signature> {
        match self.library {
            Some(ref library) => unsafe {
                library
                    .get(symbol_name.as_bytes())
                    .expect(format!("Failed to find symbol '{:?}'", symbol_name).as_str())
            },
            None => panic!(),
        }
    }
    
    pub fn has_changed(&self) -> bool {
        for event in self.watch_event_receiver.try_iter() {
            if let RawEvent { path: Some(path), op: Ok(op), cookie: _} = event {
                let path = path.as_path().canonicalize().unwrap();
                if path != self.original_path {
                    continue;
                }

                if op == WRITE {
                    println!("!write!");
                    return true;
                }

                if op == CREATE {
                    println!("!create!");
                    return true;
                }

                if op == REMOVE {
                    println!("!remove!");
                    return true;
                }

                println!("no write, create or move");
            }
        }
        return false;
    }

    pub fn reload(&mut self) {
        self.library = None;

        let (library, path) = copy_and_load_library(&self.original_path_string);

        self.library = Some(library);
        self.loaded_path = path;
        //remove_file(&self.loaded_path);
    }
}

impl Drop for HotReloadableLibrary {
    fn drop(&mut self) {
        remove_file(&self.loaded_path);
    }
}

fn construct_library_path_string(folder: &str, file_name: &str) -> String {
    let path_string = format!("{}/{}.dll", folder, file_name);
    println!("{:?}", path_string);
    path_string
}

fn watch_path(folder: &str) -> (Receiver<RawEvent>, RecommendedWatcher) {
    let (watch_event_sender, watch_event_receiver) = channel();
    let mut watcher = raw_watcher(watch_event_sender).unwrap();
    watcher.watch(folder, RecursiveMode::NonRecursive).unwrap();
    (watch_event_receiver, watcher)
}

fn copy_and_load_library(lib_path: &String) -> (libloading::Library, PathBuf) {
    let unique_name = create_unique_file_name(lib_path);
    copy_file(lib_path, &unique_name);
    let unique_lib_path = to_canonicalized_path(&unique_name);
    (
        load_library(&unique_lib_path),
        unique_lib_path,
    )
}

fn create_unique_file_name(lib_path: &String) -> String{
    let timestamp = create_timestamp();
    let index = lib_path.rfind('.').unwrap();
    let (before, after) = lib_path.split_at(index);
    format!("{}-{}{}", before, timestamp, after)
}

fn create_timestamp() -> u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn copy_file(path: &String, file_name: &String) {
    fs
        ::copy(path,file_name)
        .expect("Failed to copy to unique path");
}

fn remove_file(path: &Path) {
    println!("{:?}", path);
    //fs::remove_file(path).unwrap();
}

fn load_library(path: &Path) -> libloading::Library {
    unsafe {
        libloading::Library
            ::new(path.as_os_str())
            .expect(format!("Failed to load library '{:?}'", path).as_str())
    }
}

fn to_canonicalized_path(path_string: &String) -> PathBuf {
    Path::new(path_string).canonicalize().unwrap()
}