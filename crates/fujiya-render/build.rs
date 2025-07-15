use std::process::Command;
use std::fs::read_dir;

fn main() {

    let dirs = read_dir(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders").unwrap();
    for file_name in dirs {
        match file_name {
            Ok(name) => {

                let original_name = name.file_name().to_str().unwrap().to_string();
                if !original_name.contains('.') { continue; }

                let format = original_name.split(".").collect::<Vec<&str>>()[1];
                let name = original_name.split(".").collect::<Vec<&str>>()[0];
                let out_name = format!("{}-{}.spv", name, format);

                if format == "frag" || format == "vert" {
                    let res = Command::new("glslc")
                        .arg(format!("./shaders/{}", original_name))
                        .arg("-o")
                        .arg(format!("./shaders/spv/{}", out_name))
                        .spawn()
                        .unwrap()
                        .wait();


                    if res.is_err() {
                        panic!("WARN: {}", original_name)
                    }
                }
            },

            _ => {}
        }
    }
}