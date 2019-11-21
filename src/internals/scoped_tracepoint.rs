include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct ScopedTracepoint;

impl ScopedTracepoint {
    pub fn start_trace(name: *mut i8) {
        unsafe {
            ht_feature_callstack_start_string(ht_global_timeline_get(), name as _);
        }
    }
    
    pub fn start_trace_id(id: u64) {
        unsafe {
            ht_feature_callstack_start_int(ht_global_timeline_get(), id);
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

pub fn add_cached_mapping(name: *mut i8) -> u64 {
    unsafe {
        ht_feature_cached_string_add_mapping(ht_global_timeline_get(), name as _) as u64
    }
}