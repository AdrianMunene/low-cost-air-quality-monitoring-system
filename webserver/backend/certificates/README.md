# HTTPS Certificates

This directory contains the HTTPS certificates used by the backend and frontend servers.

## Certificate Files

When generated, this directory will contain:
- `cert.pem`: The self-signed certificate
- `key.pem`: The private key
- `openssl.cnf`: OpenSSL configuration file used during certificate generation

## Generating Certificates

The certificates are not included in the Git repository for security reasons. You need to generate them locally:

```bash
# Navigate to the backend directory
cd webserver/backend

# Run the certificate generation tool
cargo run --bin generate_certs
```

## Requirements

- OpenSSL must be installed on your system
- The certificate generation requires the `openssl` command to be available in your PATH

## Certificate Details

The generated certificates are self-signed and configured for:
- Common Name (CN): localhost
- Alternative Names: localhost, 127.0.0.1
- Validity: 365 days

## Security Note

These certificates are intended for development purposes only. In a production environment, you should use properly signed certificates from a trusted certificate authority.
