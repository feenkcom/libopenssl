mod openssl_library;

use crate::openssl_library::OpenSSLLibrary;
use shared_library_builder::{GitLocation, LibraryLocation};

pub fn libopenssl(binary_version: Option<impl Into<String>>) -> OpenSSLLibrary {
    OpenSSLLibrary::default().with_release_location(binary_version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libopenssl").tag(version))
    }))
}

pub fn libssl(binary_version: Option<impl Into<String>>) -> OpenSSLLibrary {
    libopenssl(binary_version).be_ssl()
}

pub fn libcrypto(binary_version: Option<impl Into<String>>) -> OpenSSLLibrary {
    libopenssl(binary_version).be_crypto()
}