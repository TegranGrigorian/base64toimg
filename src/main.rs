pub mod base64_lib;

use base64_lib::{example, Convert};
fn main() {
    Convert::convert_from_base64(&Convert::new(String::from("output_path.png")), &example::Base64Lib::output_base64());    
}
