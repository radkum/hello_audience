# hello_audience

A minimal Windows kernel driver in Rust, written as a live example for the SFI conference.

It is deliberately tiny: no minifilter, no device object, no IOCTLs. It shows the pieces needed to
get `no_std` Rust code into the kernel and print from it:

- `DriverEntry` and `DriverUnload`
- `#![no_std]` with a `panic_handler` and an `alloc_error_handler`
- a global allocator over `ExAllocatePool` / `ExFreePool`
- `Vec` from `alloc`
- `kernel_println!` from `kernel-print`

On load it allocates and frees a pool block, pushes one element to a `Vec`, and prints
`Hello Audience!`.

## Layout

| Path | What it is |
|---|---|
| `src/lib.rs` | `DriverEntry` / `DriverUnload`, panic and allocation-error handlers |
| `src/driver_object.rs` | minimal local declarations for `DRIVER_OBJECT`, `UNICODE_STRING`, `NTSTATUS` |
| `src/ntoskrnl.rs` | `ExAllocatePool` / `ExFreePool` |
| `src/kernel_alloc.rs` | the global kernel allocator |
| `build.rs` | finds the WDK `km` import libraries in the registry |
| `Makefile.toml` | `cargo-make` tasks: build, rename, sign, copy to `output/` |
| `docs/` | structure notes from the talk |

## Build

Requirements: Windows, Visual Studio Build Tools, WDK 10, Rust nightly and `cargo-make`.

```
cargo make all
```

That builds, renames `hello_audience.dll` to `.sys`, signs it with a self-signed certificate and
copies it to `output\`. Without `cargo-make`, run `cargo build` and rename
`target\x86_64-pc-windows-msvc\debug\hello_audience.dll` to `hello_audience.sys` yourself.

## Load

It is a plain kernel driver, not a minifilter, so it is loaded as a service:

```
sc create hello_audience type= kernel binPath= C:\path\to\hello_audience.sys
sc start hello_audience
sc stop hello_audience
sc delete hello_audience
```

`Hello Audience!` goes through `kernel_print`, so read it with WinDbg or DebugView. The driver is
unsigned: enable test signing and use a test machine only.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
