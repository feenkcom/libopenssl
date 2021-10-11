use libopenssl_library::{libcrypto, libssl};
use shared_library_builder::{Library, LibraryCompilationContext, LibraryTarget};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let version: Option<String> = None;
    let crypto = libcrypto(version.clone());
    let ssl = libssl(version.clone());

    let src_path = Path::new("target/src");
    if !src_path.exists() {
        std::fs::create_dir_all(&src_path)?;
    }

    let context = LibraryCompilationContext::new(
        src_path,
        "target",
        LibraryTarget::for_current_platform(),
        false,
    );
    let compiled_crypto = crypto.compile(&context)?;
    println!("Compiled {}", compiled_crypto.display());
    let compiled_ssl = ssl.compile(&context)?;
    println!("Compiled {}", compiled_ssl.display());
    Ok(())
}
