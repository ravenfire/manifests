use std::fs;
// use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let mut generated_code = String::new();
    let output_file = Path::new("src/generated_manifests.rs");
    // let types = ["peripherals", "games", "specs"];
    // for group in types.iter() {

    for entry in WalkDir::new("examples").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().as_os_str().to_string_lossy();
        if !path.contains(".lock.toml") {
            continue;
        }

        let file_name = path.clone();
        // let file_name = file_name.to_string_lossy();
        let const_name = file_name.replace(".", "__dot__").replace("/", "__slash__");

        let const_declaration = format!(
            " 
            #[allow(non_upper_case_globals)]
            pub const {}: &str = include_str!(\"../{}\");",
            const_name, file_name
        );

        generated_code.push_str(&const_declaration);
        generated_code.push('\n');
    }

    let match_statement = generated_match_statement(&generated_code);

    let code = format!(
        r#"
        {}

        pub fn load(file_name: &str) -> Option<&'static str> {{
            {}
        }}"#,
        generated_code, match_statement
    );

    fs::write(output_file, code).expect("Failed to write generated code");
}

fn generated_match_statement(generated_code: &str) -> String {
    let mut match_cases = String::new();

    for line in generated_code.lines() {
        if !line.contains("pub") {
            continue;
        }

        let const_name = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .trim_end_matches(';')
            .trim_end_matches(':');
        let file_name = const_name.replace("__dot__", ".").replace("__slash__", "/");
        match_cases.push_str(&format!(r#""{}" => Some({}),"#, file_name, const_name));
        match_cases.push('\n');
    }

    format!(
        r#"match file_name {{
            {}
            _ => None,
        }}"#,
        match_cases
    )
}
