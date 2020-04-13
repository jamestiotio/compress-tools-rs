/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types)]

pub const ARCHIVE_EOF: i32 = 1;
pub const ARCHIVE_OK: i32 = 0;

pub const ARCHIVE_EXTRACT_OWNER: u32 = 1;
pub const ARCHIVE_EXTRACT_PERM: u32 = 2;
pub const ARCHIVE_EXTRACT_TIME: u32 = 4;
pub const ARCHIVE_EXTRACT_ACL: u32 = 32;
pub const ARCHIVE_EXTRACT_FFLAGS: u32 = 64;
pub const ARCHIVE_EXTRACT_XATTR: u32 = 128;
pub type __int64_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type la_int64_t = i64;
pub type la_ssize_t = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct archive {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct archive_entry {
    _unused: [u8; 0],
}
pub type archive_read_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut archive,
        _client_data: *mut ::std::os::raw::c_void,
        _buffer: *mut *const ::std::os::raw::c_void,
    ) -> la_ssize_t,
>;
pub type archive_open_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut archive,
        _client_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type archive_close_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut archive,
        _client_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn archive_read_new() -> *mut archive;
}
extern "C" {
    pub fn archive_read_support_filter_all(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_support_format_all(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_support_format_raw(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_open(
        arg1: *mut archive,
        _client_data: *mut ::std::os::raw::c_void,
        arg2: archive_open_callback,
        arg3: archive_read_callback,
        arg4: archive_close_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_next_header(
        arg1: *mut archive,
        arg2: *mut *mut archive_entry,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_data_block(
        a: *mut archive,
        buff: *mut *const ::std::os::raw::c_void,
        size: *mut usize,
        offset: *mut la_int64_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_close(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_read_free(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_header(
        arg1: *mut archive,
        arg2: *mut archive_entry,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_data_block(
        arg1: *mut archive,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
        arg4: la_int64_t,
    ) -> la_ssize_t;
}
extern "C" {
    pub fn archive_write_finish_entry(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_close(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_free(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_disk_new() -> *mut archive;
}
extern "C" {
    pub fn archive_write_disk_set_options(
        arg1: *mut archive,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_write_disk_set_standard_lookup(arg1: *mut archive) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn archive_error_string(arg1: *mut archive) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn archive_set_error(
        arg1: *mut archive,
        _err: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn archive_entry_free(arg1: *mut archive_entry);
}
extern "C" {
    pub fn archive_entry_hardlink(arg1: *mut archive_entry) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn archive_entry_pathname(arg1: *mut archive_entry) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn archive_entry_set_hardlink(
        arg1: *mut archive_entry,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn archive_entry_set_pathname(
        arg1: *mut archive_entry,
        arg2: *const ::std::os::raw::c_char,
    );
}