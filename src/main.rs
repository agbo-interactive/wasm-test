const WASM_MODULE: &[u8] =
    include_bytes!("../guest-module/target/wasm32-unknown-unknown/debug/guest_module.wasm");

fn main() {
    // make the engine
    let engine = {
        let mut config = wasmtime::Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.debug_info(true);
        config.cranelift_opt_level(wasmtime::OptLevel::None);
        wasmtime::Engine::new(&config).unwrap()
    };

    // make a linkers
    let mut linker = wasmtime::Linker::new(&engine);
    linker
        .func_wrap(
            "wasm-test",
            "log_msg",
            |mut caller: wasmtime::Caller<()>,
             msg_ptr: u32,
             msg_len: u32|
             -> wasmtime::Result<()> {
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                let msg_buf = memory
                    .data(&caller)
                    .get(msg_ptr as usize..(msg_ptr + msg_len) as usize)
                    .unwrap();
                let msg = std::str::from_utf8(msg_buf).unwrap();
                println!("GUEST: {msg}");
                return Ok(());
            },
        )
        .unwrap();

    // load the module
    let module = wasmtime::Module::from_binary(&engine, WASM_MODULE).unwrap();
    /*let module_bytes = engine
        .precompile_module(WASM_MODULE)
        .expect("module compilation should succeed");
    let module = unsafe { wasmtime::Module::deserialize(&engine, &module_bytes) }
        .expect("we just got this from precompile_module!");*/

    // link the module
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let main_fn = instance
        .get_typed_func::<(), ()>(&mut store, "main")
        .unwrap();

    // run the instance
    println!("HOST: Main Starting");
    main_fn.call(&mut store, ()).unwrap();
    println!("HOST: Main Finished");
}
