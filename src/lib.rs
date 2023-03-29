#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]

mod driver_object;
use driver_object::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};

pub mod ntoskrnl;
use crate::ntoskrnl::{ExAllocatePool, ExFreePool, POOL_TYPE, PVOID};

mod kernel_alloc;

#[global_allocator]
static GLOBAL: kernel_alloc::KernelAlloc = kernel_alloc::KernelAlloc;

extern crate alloc;
use alloc::vec;

#[no_mangle]
pub unsafe extern "system"  fn DriverEntry(
    driver_object: &mut DRIVER_OBJECT,
    _reg_path: *const UNICODE_STRING,
) -> NTSTATUS {
    driver_object.DriverUnload = Some(DriverUnload);

    let ptr = ExAllocatePool(POOL_TYPE::NonPagedPool, 0x10);
    ExFreePool(ptr as PVOID);

    let mut vector = vec![];
    vector.push(3);

    kernel_print::kernel_println!("Hello Audience!");
    STATUS_SUCCESS

}

extern "system" fn DriverUnload(_driver: &mut DRIVER_OBJECT) {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}



