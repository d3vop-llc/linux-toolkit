use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=scripts/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_scripts.rs");
    let mut file = fs::File::create(&dest_path).unwrap();

    // Start the embedded scripts module
    writeln!(file, "// Auto-generated embedded scripts").unwrap();
    writeln!(file, "").unwrap();
    writeln!(
        file,
        "pub fn get_embedded_scripts() -> std::collections::HashMap<&'static str, &'static [u8]> {{"
    ).unwrap();
    writeln!(file, "    let mut scripts = std::collections::HashMap::new();").unwrap();

    // Embed the scripts directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let scripts_dir = Path::new(&manifest_dir).join("scripts");

    if scripts_dir.exists() {
        embed_scripts_recursively(&scripts_dir, &scripts_dir, &mut file);
    }

    writeln!(file, "    scripts").unwrap();
    writeln!(file, "}}").unwrap();

    println!("cargo:rustc-env=EMBEDDED_SCRIPTS_PATH={}", dest_path.display());
}

fn embed_scripts_recursively(base_dir: &Path, current_dir: &Path, file: &mut fs::File) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    embed_scripts_recursively(base_dir, &path, file);
                } else if path.is_file() {
                    // Get relative path from base scripts directory
                    let relative_path = path.strip_prefix(base_dir).unwrap();
                    let relative_str = relative_path.to_string_lossy().replace('\\', "/");

                    // Convert Windows path to Unix-style path for the include_bytes! macro
                    let include_path = if cfg!(target_os = "windows") {
                        path.to_string_lossy().replace('\\', "/")
                    } else {
                        path.to_string_lossy().to_string()
                    };

                    // Embed the file content using raw string to avoid escape issues
                    // Cast to &[u8] to ensure consistent type
                    writeln!(
                        file,
                        "    scripts.insert(\"{}\", include_bytes!(r\"{}\") as &'static [u8]);",
                        relative_str,
                        include_path
                    ).unwrap();

                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }
}
