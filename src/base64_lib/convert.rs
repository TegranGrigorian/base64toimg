//this contains the base64 converter, it takes an input of a output path and a base64 string and outputs a image

use base64::decode; //depracted function but still works
use std::fs::File;
use std::io::Write;
pub struct Convert {
    output_path: String
}

impl Convert {
    pub fn new(output_path: String) -> Self {
        Self { output_path } //new output path
    }

    pub fn convert_from_base64(&self,base64:&str,) {
        match decode(base64) { //like case in C, match the decode output, either error or ok
            Ok(image) => { //if ok, lets output the image to a new path
                let mut file = File::create("output.png").expect("error outputting file");
                file.write_all(&image).expect("failed to write the data");
                println!("Image decoded, {}", self.output_path)
            } Err(e) => { // if error tell the user
                    println!("couldnt decode base 64");
            }
        }
    }
}