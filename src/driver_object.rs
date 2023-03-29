#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub const STATUS_SUCCESS: NTSTATUS = 0;
pub type NTSTATUS = i32;
pub type UNICODE_STRING = *const u16;
pub type PDRIVER_UNLOAD = Option<extern "system" fn(_self: &mut DRIVER_OBJECT)>;

#[repr(C)]
pub struct DRIVER_OBJECT {
    pub filler: [u8; 0x68],
    pub DriverUnload: PDRIVER_UNLOAD,
}

const _SIZE_CHECKER: [u8; 0x70] = [0; core::mem::size_of::<DRIVER_OBJECT>()];
