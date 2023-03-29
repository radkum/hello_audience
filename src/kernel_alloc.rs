//it must be defined in lib.rs
//#![feature(alloc_error_handler)]
#[allow(unused_imports)]
use alloc::alloc::handle_alloc_error;
use core::alloc::{GlobalAlloc, Layout};
use crate::{ExFreePool, ExAllocatePool, POOL_TYPE};

pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pool = ExAllocatePool(POOL_TYPE::NonPagedPool, layout.size());

        #[cfg(feature = "alloc_panic")]
        if pool.is_null() {
            handle_alloc_error(layout);
        }

        pool as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePool(ptr as _);
    }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[no_mangle]
extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}