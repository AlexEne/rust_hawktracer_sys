include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct ScopedTracepoint;

impl ScopedTracepoint {
    pub fn start_trace(name: *mut i8) {
        unsafe {
            ht_feature_callstack_start_string(ht_global_timeline_get(), name);
        }
    }
}

impl Drop for ScopedTracepoint {
    fn drop(&mut self) {
        unsafe {
            ht_feature_callstack_stop(ht_global_timeline_get());
        }
    }
}
