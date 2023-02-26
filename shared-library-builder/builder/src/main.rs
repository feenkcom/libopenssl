use std::error::Error;
use std::path::Path;

use shared_library_builder::{with_target, Library, LibraryCompilationContext};

use libopenssl_library::{libcrypto, libssl};

fn main() -> Result<(), Box<dyn Error>> {
    let src_path = Path::new("target/src");
    if !src_path.exists() {
        std::fs::create_dir_all(&src_path)?;
    }

    with_target(|target| {
        let version: Option<String> = None;
        let crypto = libcrypto(version.clone());
        let ssl = libssl(version.clone());

        let context = LibraryCompilationContext::new(src_path, "target", target, false);
        let compiled_crypto = crypto.compile(&context)?;
        println!("Compiled {}", compiled_crypto.display());
        let compiled_ssl = ssl.compile(&context)?;
        println!("Compiled {}", compiled_ssl.display());
        Ok(())
    })
}
