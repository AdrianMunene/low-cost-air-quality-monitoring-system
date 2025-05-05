use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use backend::security::certificates::ensure_certs_dir;

fn main() {
    println!("Generating self-signed certificates for HTTPS...");

    // Create certificates directory if it doesn't exist
    ensure_certs_dir();
    let certificates_dir = Path::new("certificates");

    // Generate OpenSSL configuration file
    let openssl_config = r#"
[req]
default_bits = 2048
prompt = no
default_md = sha256
distinguished_name = dn
x509_extensions = v3_req

[dn]
C = US
ST = State
L = City
O = Organization
OU = OrganizationalUnit
CN = localhost

[v3_req]
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
    "#;

    let config_path = certificates_dir.join("openssl.cnf");
    let mut config_file = File::create(&config_path).expect("Failed to create OpenSSL config file");
    config_file.write_all(openssl_config.as_bytes()).expect("Failed to write OpenSSL config");

    // Generate private key
    let key_path = certificates_dir.join("key.pem");
    let status = Command::new("openssl")
        .args(["genrsa", "-out"])
        .arg(&key_path)
        .arg("2048")
        .status()
        .expect("Failed to execute OpenSSL command for key generation");

    if !status.success() {
        panic!("Failed to generate private key");
    }

    // Generate certificate
    let cert_path = certificates_dir.join("cert.pem");
    let status = Command::new("openssl")
        .args(["req", "-new", "-x509", "-key"])
        .arg(&key_path)
        .args(["-out"])
        .arg(&cert_path)
        .args(["-days", "365", "-config"])
        .arg(&config_path)
        .status()
        .expect("Failed to execute OpenSSL command for certificate generation");

    if !status.success() {
        panic!("Failed to generate certificate");
    }

    println!("Self-signed certificates generated successfully!");
    println!("Certificate: {}", cert_path.display());
    println!("Private key: {}", key_path.display());
}
