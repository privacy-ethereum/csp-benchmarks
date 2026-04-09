use std::ffi::c_void;

#[allow(dead_code)]
unsafe extern "C" {
    pub fn zolt_thread_pool_create() -> *mut c_void;
    pub fn zolt_thread_pool_destroy(handle: *mut c_void);

    pub fn zolt_load_elf(
        tp_handle: *mut c_void,
        path_ptr: *const u8,
        path_len: usize,
    ) -> *mut c_void;
    pub fn zolt_loaded_elf_size(handle: *mut c_void) -> usize;
    pub fn zolt_loaded_elf_destroy(handle: *mut c_void);

    pub fn zolt_prove(elf_handle: *mut c_void) -> *mut c_void;

    pub fn zolt_proof_result_size(handle: *mut c_void) -> usize;
    pub fn zolt_proof_result_destroy(handle: *mut c_void);
}
