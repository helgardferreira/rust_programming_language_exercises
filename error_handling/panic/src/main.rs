fn main() {
    // unrecoverable errors
    // manually calling the panic! macro
    /*panic!("crash and burn!");*/

    // panic macro being invoked from a library
    let v = vec![1, 2, 3];

    v[99];
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',
    /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
    */

    // with RUST_BACKTRACE=1
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',
    /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
    stack backtrace:
       0: backtrace::backtrace::libunwind::trace
                 at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
       1: backtrace::backtrace::trace_unsynchronized
                 at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
       2: std::sys_common::backtrace::_print_fmt
                 at src/libstd/sys_common/backtrace.rs:84
       3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
                 at src/libstd/sys_common/backtrace.rs:61
       4: core::fmt::ArgumentV1::show_usize
       5: std::io::Write::write_fmt
                 at src/libstd/io/mod.rs:1426
       6: std::sys_common::backtrace::_print
                 at src/libstd/sys_common/backtrace.rs:65
       7: std::sys_common::backtrace::print
                 at src/libstd/sys_common/backtrace.rs:50
       8: std::panicking::default_hook::{{closure}}
                 at src/libstd/panicking.rs:193
       9: std::panicking::default_hook
                 at src/libstd/panicking.rs:210
      10: std::panicking::rust_panic_with_hook
                 at src/libstd/panicking.rs:471
      11: rust_begin_unwind
                 at src/libstd/panicking.rs:375
      12: core::panicking::panic_fmt
                 at src/libcore/panicking.rs:84
      13: core::panicking::panic_bounds_check
                 at src/libcore/panicking.rs:62
      14: <usize as core::slice::SliceIndex<[T]>>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
      15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2657
      16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/liballoc/vec.rs:1871
      17: panic::main
                 at src/main.rs:8
      18: std::rt::lang_start::{{closure}}
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
      19: std::rt::lang_start_internal::{{closure}}
                 at src/libstd/rt.rs:52
      20: std::panicking::try::do_call
                 at src/libstd/panicking.rs:292
      21: __rust_maybe_catch_panic
                 at src/libpanic_unwind/lib.rs:78
      22: std::panicking::try
                 at src/libstd/panicking.rs:270
      23: std::panic::catch_unwind
                 at src/libstd/panic.rs:394
      24: std::rt::lang_start_internal
                 at src/libstd/rt.rs:51
      25: std::rt::lang_start
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
      26: panic::main
    */
}
