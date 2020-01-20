use super::hawktracer_listener::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl<'a> HawktracerListener<'a> for HawktracerListenerTCP {}

pub struct HawktracerListenerTCP {}

impl HawktracerListenerTCP {
    pub fn new(port: u32, buffer_size: usize) -> HawktracerListenerTCP {
        unsafe {
            ht_tcp_listener_register(
                ht_global_timeline_get(),
                port as i32,
                buffer_size,
                std::ptr::null_mut() as _,
            );
        };

        // TODO: For backward compatibility only. Remove on next API change
        HawktracerListenerTCP { }
    }
}

// TODO: For backward compatibility only. Remove on next API change
impl Drop for HawktracerListenerTCP {
    fn drop(&mut self) {
    }
}