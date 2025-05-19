use std::fs;

fn read_file_as_plain_text(path: String) -> Option<String> {
    let contents = fs::read_to_string(path);

    match contents {
        Ok(cont) => { Some(cont) }
        Err(err) => {
            println!("{err}");
            None
        }
    }
}

pub fn read_shader(path: &str) -> Option<String> {
    read_file_as_plain_text("./assets/shaders".to_string() + path + ".glsl")
}
