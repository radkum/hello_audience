#![allow(non_camel_case_types)]
use core::ffi::c_void;

pub type PVOID = *mut c_void;
pub type SIZE_T = usize;

#[repr(C)]
pub enum POOL_TYPE {
    NonPagedPool,
    //NonPagedPoolExecute,
    //PagedPool,
}

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAllocatePool(PoolType: POOL_TYPE, NumberOfBytes: SIZE_T) -> PVOID;
    pub fn ExFreePool(Pool: PVOID);
}