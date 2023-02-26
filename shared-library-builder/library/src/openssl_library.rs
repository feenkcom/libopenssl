use shared_library_builder::{
    CompiledLibraryName, GitLocation, Library, LibraryCompilationContext, LibraryDependencies,
    LibraryLocation, LibraryOptions, LibraryTarget,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LibraryArtefact {
    Crypto,
    Ssl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSSLLibrary {
    source_location: LibraryLocation,
    release_location: Option<LibraryLocation>,
    options: LibraryOptions,
    artefact: LibraryArtefact,
}

impl Default for OpenSSLLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenSSLLibrary {
    pub fn new() -> Self {
        Self {
            source_location: LibraryLocation::Git(
                GitLocation::github("syrel", "openssl")
                    .branch("OpenSSL_1_1_1-stable-Windows-pkgconfig"),
            ),
            release_location: None,
            options: Default::default(),
            artefact: LibraryArtefact::Crypto,
        }
    }

    pub fn be_ssl(mut self) -> Self {
        self.artefact = LibraryArtefact::Ssl;
        self
    }

    pub fn be_crypto(mut self) -> Self {
        self.artefact = LibraryArtefact::Crypto;
        self
    }

    pub fn with_release_location(mut self, release_location: Option<LibraryLocation>) -> Self {
        self.release_location = release_location;
        self
    }

    pub fn compiler(&self, options: &LibraryCompilationContext) -> &str {
        match options.target() {
            LibraryTarget::X8664appleDarwin => "darwin64-x86_64-cc",
            LibraryTarget::AArch64appleDarwin => "darwin64-arm64-cc",
            LibraryTarget::X8664pcWindowsMsvc => "VC-WIN64A",
            LibraryTarget::AArch64pcWindowsMsvc => "VC-WIN64-ARM",
            LibraryTarget::X8664UnknownlinuxGNU => "linux-x86_64-clang",
            LibraryTarget::AArch64UnknownlinuxGNU => "linux-aarch64",
            LibraryTarget::AArch64LinuxAndroid => "android-arm64",
        }
    }
}

#[typetag::serde]
impl Library for OpenSSLLibrary {
    fn location(&self) -> &LibraryLocation {
        &self.source_location
    }

    fn release_location(&self) -> &LibraryLocation {
        self.release_location
            .as_ref()
            .unwrap_or_else(|| &self.source_location)
    }

    fn name(&self) -> &str {
        match self.artefact {
            LibraryArtefact::Crypto => "crypto",
            LibraryArtefact::Ssl => "ssl",
        }
    }

    fn compiled_library_name(&self) -> CompiledLibraryName {
        match self.artefact {
            LibraryArtefact::Crypto => CompiledLibraryName::Matching("crypto".to_string()),
            LibraryArtefact::Ssl => CompiledLibraryName::Matching("ssl".to_string()),
        }
    }

    fn dependencies(&self) -> Option<&LibraryDependencies> {
        None
    }

    fn options(&self) -> &LibraryOptions {
        &self.options
    }

    fn options_mut(&mut self) -> &mut LibraryOptions {
        &mut self.options
    }

    fn force_compile(&self, options: &LibraryCompilationContext) -> Result<(), Box<dyn Error>> {
        let out_dir = self.native_library_prefix(options);
        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir)
                .unwrap_or_else(|_| panic!("Could not create {:?}", &out_dir));
        }

        let makefile_dir = options.build_root().join(self.name());
        if !makefile_dir.join("makefile").exists() {
            let mut command = Command::new("perl");
            command
                .current_dir(&makefile_dir)
                .arg(self.source_directory(options).join("Configure"))
                .arg(format!("--{}", options.profile()))
                .arg(format!(
                    "--prefix={}",
                    self.native_library_prefix(options).display()
                ))
                .arg(format!(
                    "--openssldir={}",
                    self.native_library_prefix(options).display()
                ))
                .arg(self.compiler(options))
                .arg("OPT_LEVEL=3");

            if self.is_static() {
                command.arg("no-shared");
            }
            if options.target().is_android() {
                command.arg(format!(
                    "-D__ANDROID_API__{}=",
                    options.android_target_api()
                ));
                configure_android_path(&mut command);
            }

            let configure = command.status().unwrap();

            if !configure.success() {
                panic!("Could not configure {}", self.name());
            }
        };

        let make = if options.is_windows() {
            let compiler = cc::Build::new()
                .opt_level(3)
                .target(options.target().to_string().as_str())
                .host(LibraryTarget::for_current_host().to_string().as_str())
                .debug(options.is_debug())
                .get_compiler();
            let build_tools_dir = compiler.path().parent().unwrap();
            let nmake = build_tools_dir.join("nmake.exe");
            if !nmake.exists() {
                panic!("Could not find nmake.exe in {}", build_tools_dir.display());
            };

            let filtered_env: HashMap<OsString, OsString> = compiler
                .env()
                .iter()
                .map(|(k, value)| (k.clone(), value.clone()))
                .collect();

            Command::new(&nmake)
                .current_dir(&makefile_dir)
                .envs(filtered_env)
                .arg("install_sw")
                .status()
                .unwrap()
        } else {
            let mut command = Command::new("make");
            command.current_dir(&makefile_dir).arg("install_sw");

            if options.target().is_android() {
                configure_android_path(&mut command);
            }

            command.status().unwrap()
        };

        if !make.success() {
            panic!("Could not compile {}", self.name());
        }
        Ok(())
    }

    fn compiled_library_directories(&self, context: &LibraryCompilationContext) -> Vec<PathBuf> {
        if context.is_unix() {
            let lib = self.native_library_prefix(context).join("lib");
            return vec![lib];
        }
        if context.is_windows() {
            let lib = self.native_library_prefix(context).join("bin");
            return vec![lib];
        }
        vec![]
    }

    fn ensure_requirements(&self, options: &LibraryCompilationContext) {
        which::which("perl").expect("Could not find `perl`");

        if options.is_unix() {
            which::which("make").expect("Could not find `make`");
        }
        if options.is_windows() {
            which::which("nasm").expect("Could not find `nasm`");
        }
    }

    fn native_library_prefix(&self, options: &LibraryCompilationContext) -> PathBuf {
        options.build_root().join(self.name()).join("build")
    }

    fn native_library_include_headers(&self, context: &LibraryCompilationContext) -> Vec<PathBuf> {
        let mut dirs = vec![];

        let directory = self.native_library_prefix(context).join("include");

        if directory.exists() {
            dirs.push(directory);
        }

        dirs
    }

    fn native_library_linker_libraries(&self, context: &LibraryCompilationContext) -> Vec<PathBuf> {
        let mut dirs = vec![];

        let directory = self.native_library_prefix(context).join("lib");

        if directory.exists() {
            dirs.push(directory);
        }

        dirs
    }

    fn pkg_config_directory(&self, context: &LibraryCompilationContext) -> Option<PathBuf> {
        let directory = self
            .native_library_prefix(context)
            .join("lib")
            .join("pkgconfig");

        if directory.exists() {
            return Some(directory);
        }

        None
    }

    fn clone_library(&self) -> Box<dyn Library> {
        Box::new(Clone::clone(self))
    }
}

impl From<OpenSSLLibrary> for Box<dyn Library> {
    fn from(library: OpenSSLLibrary) -> Self {
        Box::new(library)
    }
}

fn configure_android_path(command: &mut Command) {
    let ndk = ndk_build::ndk::Ndk::from_env().unwrap();

    let new_path = format!(
        "{}:{}",
        ndk.toolchain_dir().unwrap().join("bin").display(),
        std::env::var("PATH").expect("PATH must be set")
    );

    command.env("PATH", new_path);

    let ndk_root = std::env::var("ANDROID_NDK")
        .or_else(|_| std::env::var("NDK_HOME"))
        .expect("ANDROID_NDK or NDK_HOME must be defined");

    command.env("ANDROID_NDK_ROOT", ndk_root);
}
