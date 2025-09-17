pub mod api;
mod guest;

#[unsafe(no_mangle)]
unsafe extern "C" fn main() {
    guest::main();
}
