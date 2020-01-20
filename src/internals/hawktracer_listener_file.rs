use super::hawktracer_listener::*;
#[allow(unused_imports)]
use std::path::PathBuf;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct HawktracerListenerFile {
}

impl<'a> HawktracerListener<'a> for HawktracerListenerFile {}

impl HawktracerListenerFile {
    pub fn new(file_path: PathBuf, buffer_size: usize) -> HawktracerListenerFile {
        let string_path = file_path.into_os_string().into_string().unwrap();
        let file_path = std::ffi::CString::new(string_path).unwrap();
        unsafe {
            ht_file_dump_listener_register(
                ht_global_timeline_get(),
                file_path.as_ptr(),
                buffer_size,
                std::ptr::null_mut() as _,
            );
        };

        // TODO: For backward compatibility only. Remove on next API change
        HawktracerListenerFile { }
    }
}

// TODO: For backward compatibility only. Remove on next API change
impl Drop for HawktracerListenerFile {
    fn drop(&mut self) {
    }
}