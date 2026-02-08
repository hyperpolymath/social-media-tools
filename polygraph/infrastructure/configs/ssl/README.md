# SSL Certificate Configuration

## Production Setup

For production environments, you should use Let's Encrypt or another trusted certificate authority.

### Option 1: Let's Encrypt with Certbot

```bash
# Install certbot
sudo apt-get install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot certonly --webroot \
  -w /var/www/certbot \
  -d yourdomain.com \
  -d www.yourdomain.com

# Copy certificates to this directory
sudo cp /etc/letsencrypt/live/yourdomain.com/fullchain.pem ./cert.pem
sudo cp /etc/letsencrypt/live/yourdomain.com/privkey.pem ./key.pem
```

### Option 2: Commercial Certificate

Place your certificate files in this directory:
- `cert.pem` - Your SSL certificate (or fullchain)
- `key.pem` - Your private key

### Development Setup

For local development, you can generate self-signed certificates:

```bash
cd infrastructure/configs/ssl

# Generate self-signed certificate (valid for 365 days)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout key.pem \
  -out cert.pem \
  -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Set proper permissions
chmod 600 key.pem
chmod 644 cert.pem
```

## Certificate Rotation

Certificates should be rotated before expiration. Let's Encrypt certificates expire after 90 days.

### Automated Renewal (Let's Encrypt)

Add to crontab:
```bash
0 0 * * * certbot renew --quiet && cp /etc/letsencrypt/live/yourdomain.com/*.pem /path/to/ssl/ && podman-compose restart nginx
```

## Security Best Practices

1. **Private Key Protection**
   - Never commit `key.pem` to version control
   - Set restrictive file permissions (600)
   - Use HSM for production keys if possible

2. **Certificate Monitoring**
   - Monitor certificate expiration
   - Set up alerts 30 days before expiration
   - Use certificate transparency monitoring

3. **Strong Ciphers**
   - The nginx.conf is configured with modern cipher suites
   - TLS 1.2 and 1.3 only
   - HSTS enabled with 1-year max-age

4. **OCSP Stapling**
   - Enabled in nginx configuration
   - Improves performance and privacy

## Troubleshooting

### Certificate Verification

```bash
# Check certificate
openssl x509 -in cert.pem -text -noout

# Verify certificate and key match
openssl x509 -noout -modulus -in cert.pem | openssl md5
openssl rsa -noout -modulus -in key.pem | openssl md5
```

### Test SSL Configuration

```bash
# Using openssl
openssl s_client -connect localhost:443 -servername yourdomain.com

# Using curl
curl -vI https://localhost

# Online tools (production only)
# - SSL Labs: https://www.ssllabs.com/ssltest/
# - Mozilla Observatory: https://observatory.mozilla.org/
```

## Files in this Directory

- `README.md` - This file
- `cert.pem` - SSL certificate (gitignored)
- `key.pem` - Private key (gitignored)
- `.gitkeep` - Keep directory in git

**IMPORTANT**: Never commit actual certificate or key files to version control!
