# Air Quality Monitoring System Backend

This is the backend server for the Air Quality Monitoring System. It provides API endpoints for storing and retrieving air quality data.

## Features

- RESTful API for air quality data
- API key authentication for secure data submission
- Data validation to ensure data integrity
- Rate limiting to prevent abuse
- Comprehensive logging for monitoring and debugging
- SQLite database for data storage
- CORS support for frontend integration
- Environment variable configuration
- Robust error handling

## Setup

1. Make sure you have Rust and Cargo installed
2. Run the server:
   ```
   cargo run
   ```
   (This works because we've set `default-run = "backend"` in Cargo.toml)

### Rate Limiting

To run the server with rate limiting (10 requests per minute):
```
cargo run --bin server_with_rate_limit
```

## Environment Variables

The following environment variables can be configured in the `.env` file:

- `DATABASE_URL`: Path to the SQLite database file
- `HOST`: Host address to bind the server to (default: 0.0.0.0)
- `PORT`: Port to listen on (default: 3000)
- `API_KEY`: Secret key for authenticating API requests
- `RUST_LOG`: Logging level configuration (default: info,tower_http=debug,axum=debug)


## API Endpoints

### GET /airquality

Retrieves all air quality records from the database.

**Response:**
```json
[
  {
    "timestamp": "2025-03-30 12:34:56",
    "longitude": -122.4194,
    "latitude": 37.7749,
    "temperature": 18.5,
    "pressure": 1012.3,
    "humidity": 60.2,
    "pm1_0": 5.1,
    "pm2_5": 10.2,
    "pm10": 20.5,
    "co2": 400.0,
    "co": 0.5,
    "o3": 0.03
  }
]
```

### POST /airquality

Creates a new air quality record in the database. Requires API key authentication.

**Headers:**
```
X-API-Key: your_secure_api_key_here
```

**Request Body:**
```json
{
  "timestamp": "2025-03-30 12:34:56",
  "longitude": -122.4194,
  "latitude": 37.7749,
  "temperature": 18.5,
  "pressure": 1012.3,
  "humidity": 60.2,
  "pm1_0": 5.1,
  "pm2_5": 10.2,
  "pm10": 20.5,
  "co2": 400.0,
  "co": 0.5,
  "o3": 0.03
}
```

**Response:**
```json
{
  "status": "success"
}
```

## Sending Data from GSM Module

To send data from your GSM module to the backend:

1. Configure your GSM module to connect to the internet using your mobile carrier's APN
2. Send an HTTP POST request to the `/airquality` endpoint with the following:
   - Content-Type: application/json
   - X-API-Key header with your API key
   - JSON body with the air quality data

Example AT commands for SIM808 module:

For HTTP:
```
AT+SAPBR=3,1,"CONTYPE","GPRS"
AT+SAPBR=3,1,"APN","your_carrier_apn"
AT+SAPBR=1,1
AT+HTTPINIT
AT+HTTPPARA="CID",1
AT+HTTPPARA="URL","http://your-server-ip:3000/airquality"
AT+HTTPPARA="CONTENT","application/json"
AT+HTTPPARA="USERDATA","X-API-Key: your_secure_api_key_here"
AT+HTTPDATA=200,10000
{"timestamp":"2025-03-30 12:34:56","temperature":18.5,"humidity":60.2,"pm1_0":5.1,"pm2_5":10.2,"pm10":20.5,"co2":400.0}
AT+HTTPACTION=1
AT+HTTPTERM
```

For HTTPS (with Cloudflare Tunnel):
```
AT+SAPBR=3,1,"CONTYPE","GPRS"
AT+SAPBR=3,1,"APN","your_carrier_apn"
AT+SAPBR=1,1
AT+HTTPINIT
AT+HTTPPARA="CID",1
AT+HTTPPARA="URL","https://air-quality.yourdomain.com/airquality"
AT+HTTPPARA="CONTENT","application/json"
AT+HTTPPARA="USERDATA","X-API-Key: your_secure_api_key_here"
AT+HTTPSSL=1
AT+HTTPDATA=200,10000
{"timestamp":"2025-03-30 12:34:56","temperature":18.5,"humidity":60.2,"pm1_0":5.1,"pm2_5":10.2,"pm10":20.5,"co2":400.0}
AT+HTTPACTION=1
AT+HTTPTERM
```

## Setting Up Cloudflare Tunnel

To expose your backend to the internet securely using Cloudflare Tunnel:

1. Create a Cloudflare account if you don't have one
2. Install the Cloudflare Tunnel client (cloudflared)
3. Authenticate with Cloudflare:
   ```
   cloudflared tunnel login
   ```
4. Create a tunnel:
   ```
   cloudflared tunnel create air-quality-backend
   ```
5. Configure your tunnel by creating a config file `config.yml`:
   ```yaml
   tunnel: <TUNNEL_ID>
   credentials-file: /path/to/.cloudflared/<TUNNEL_ID>.json

   ingress:
     - hostname: air-quality.yourdomain.com
       service: http://localhost:3000
     - service: http_status:404
   ```

   For HTTPS server:
   ```yaml
   tunnel: <TUNNEL_ID>
   credentials-file: /path/to/.cloudflared/<TUNNEL_ID>.json

   ingress:
     - hostname: air-quality.yourdomain.com
       service: https://localhost:3000
       originRequest:
         noTLSVerify: true  # Only for self-signed certificates
     - service: http_status:404
   ```

6. Route DNS to your tunnel:
   ```
   cloudflared tunnel route dns <TUNNEL_ID> air-quality.yourdomain.com
   ```
7. Start the tunnel:
   ```
   cloudflared tunnel run air-quality-backend
   ```

8. For production, consider using Cloudflare Zero Trust for additional security:
   ```
   cloudflared tunnel run --protocol http2 air-quality-backend
   ```

Once set up, your GSM module can send data to `https://air-quality.yourdomain.com/airquality` instead of using your server's IP address.

## Security Considerations

- Always use HTTPS when exposing your API to the internet
- Keep your API key secret and rotate it periodically
- Enable rate limiting to prevent abuse
- Monitor your server logs for suspicious activity
- Use proper SSL certificates in production (not self-signed)
- Consider using a reverse proxy like Nginx for additional security
- Regularly update dependencies to patch security vulnerabilities
- Back up your database regularly
