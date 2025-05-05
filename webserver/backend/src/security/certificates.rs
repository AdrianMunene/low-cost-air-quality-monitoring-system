use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::env;
use std::fs;

/// Loads certificates from PEM files
pub fn load_certs(cert_path: &Path) -> Vec<CertificateDer<'static>> {
    let cert_file = File::open(cert_path)
        .unwrap_or_else(|_| panic!("Failed to open certificate file: {:?}", cert_path));
    let mut cert_reader = BufReader::new(cert_file);

    let certs_result: Result<Vec<_>, _> = certs(&mut cert_reader).collect();
    match certs_result {
        Ok(certs) => certs,
        Err(e) => panic!("Failed to parse certificate: {}", e),
    }
}

/// Loads private key from PEM file
pub fn load_private_key(key_path: &Path) -> PrivateKeyDer<'static> {
    let key_file = File::open(key_path)
        .unwrap_or_else(|_| panic!("Failed to open private key file: {:?}", key_path));
    let mut key_reader = BufReader::new(key_file);

    let keys_result: Result<Vec<_>, _> = pkcs8_private_keys(&mut key_reader).collect();
    let keys = match keys_result {
        Ok(keys) => keys,
        Err(e) => panic!("Failed to parse private key: {}", e),
    };

    if keys.is_empty() {
        panic!("No private keys found in file: {:?}", key_path);
    }

    PrivateKeyDer::Pkcs8(keys[0].clone_key())
}

/// Returns the paths to the certificate and key files
pub fn get_cert_paths() -> (PathBuf, PathBuf) {
    // Check environment variables first
    let cert_path = env::var("CERT_PATH").unwrap_or_else(|_| "certificates/cert.pem".to_string());
    let key_path = env::var("KEY_PATH").unwrap_or_else(|_| "certificates/key.pem".to_string());

    (PathBuf::from(cert_path), PathBuf::from(key_path))
}

/// Ensures the certificates directory exists
pub fn ensure_certs_dir() {
    let certificates_dir = PathBuf::from("certificates");
    if !certificates_dir.exists() {
        fs::create_dir_all(&certificates_dir).expect("Failed to create certificates directory");
    }
}

/// Checks if certificates exist, returns true if both cert and key files exist
pub fn certs_exist() -> bool {
    let (cert_path, key_path) = get_cert_paths();
    cert_path.exists() && key_path.exists()
}
