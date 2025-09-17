use crate::*;

pub fn main() {
    let x = 29283;
    let testing: String = "this is a test string".into();
    api::log(format_args!("Hello World: {testing}. x = {x}"));
}
