
use libc::c_char;
use std::path::PathBuf;
use pathfinder_resources::ResourceLoader;
use pathfinder_resources::fs::FilesystemResourceLoader;
use pathfinder_resources::embedded::EmbeddedResourceLoader;

use crate::{
    PFResourceLoaderRef, ResourceLoaderWrapper, to_rust_string
};


/// Creates a loader that will attempt to use the resources embedded into the pathfinder binary.
/// Note that this will fail if pathfinder was not compiled with embedded resources.
#[no_mangle]
pub unsafe extern "C" fn PFEmbeddedResourceLoaderCreate() -> PFResourceLoaderRef {
    let loader = Box::new(EmbeddedResourceLoader::new());
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

/// Attempts to find the resources by traversing the filesystem relative to the current directory of the binary.
/// Will fail if the resource directory cannot be found.
#[no_mangle]
pub unsafe extern "C" fn PFFilesystemResourceLoaderLocate() -> PFResourceLoaderRef {
    let loader = Box::new(FilesystemResourceLoader::locate());
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

/// Creates a resource loader with the given path as the resources directory.
/// Will fail if the resources are not actually present in the directory.
#[no_mangle]
pub unsafe extern "C" fn PFFilesystemResourceLoaderFromPath(path: *const c_char) -> PFResourceLoaderRef {
    let string = to_rust_string(&path, 0);
    let directory = PathBuf::from(string);
    let loader = Box::new(FilesystemResourceLoader { directory });
    Box::into_raw(Box::new(ResourceLoaderWrapper(loader as Box<dyn ResourceLoader>)))
}

/// Destroys the resource loader.
#[no_mangle]
pub unsafe extern "C" fn PFResourceLoaderDestroy(loader: PFResourceLoaderRef) {
    drop(Box::from_raw(loader))
}