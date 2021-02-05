
use libc::c_char;
use std::path::PathBuf;
use pathfinder_resources::ResourceLoader;
use pathfinder_resources::fs::FilesystemResourceLoader;
use pathfinder_resources::embedded::EmbeddedResourceLoader;

use crate::{
    PFResourceLoaderRef, ResourceLoaderWrapper, to_rust_string
};

#[no_mangle]
pub unsafe extern "C" fn PFEmbeddedResourceLoaderCreate() -> PFResourceLoaderRef {
    let loader = Box::new(EmbeddedResourceLoader::new());
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

#[no_mangle]
pub unsafe extern "C" fn PFFilesystemResourceLoaderLocate() -> PFResourceLoaderRef {
    let loader = Box::new(FilesystemResourceLoader::locate());
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

#[no_mangle]
pub unsafe extern "C" fn PFFilesystemResourceLoaderFromPath(path: *const c_char) -> PFResourceLoaderRef {
    let string = to_rust_string(&path, 0);
    let directory = PathBuf::from(string);
    let loader = Box::new(FilesystemResourceLoader { directory });
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

#[no_mangle]
pub unsafe extern "C" fn PFResourceLoaderDestroy(loader: PFResourceLoaderRef) {
    drop(Box::from_raw(loader))
}