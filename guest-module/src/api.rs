pub fn log(args: std::fmt::Arguments) {
    let msg = format!("{args}");
    unsafe {
        host_api::log_msg(msg.as_ptr(), msg.len());
    }
}

mod host_api {
    #[link(wasm_import_module = "wasm-test")]
    unsafe extern "C" {
        pub fn log_msg(msg: *const u8, msglen: usize);
    }
}
