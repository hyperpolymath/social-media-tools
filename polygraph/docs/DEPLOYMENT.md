# Deployment Guide

## Prerequisites

- Server with Linux (Ubuntu 22.04+ recommended)
- Podman or Docker installed
- Domain name configured (for production)
- SSL certificate (Let's Encrypt recommended)

## Production Deployment with Podman Compose

### 1. Server Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Podman
sudo apt install -y podman podman-compose

# Install other dependencies
sudo apt install -y git nginx certbot python3-certbot-nginx
```

### 2. Clone Repository

```bash
cd /opt
sudo git clone https://github.com/hyperpolymath/social-media-polygraph.git
cd social-media-polygraph
```

### 3. Configure Environment

```bash
# Copy and edit environment files
cd infrastructure/podman
cp .env.example .env

# Edit .env with production values
nano .env
```

**Important production settings:**

```env
# Generate strong secrets
SECRET_KEY=$(openssl rand -hex 32)
JWT_SECRET_KEY=$(openssl rand -hex 32)

# Set production URLs
API_URL=https://api.yourdomain.com
FRONTEND_URL=https://yourdomain.com

# Database passwords
ARANGO_ROOT_PASSWORD=$(openssl rand -base64 24)

# Disable debug
DEBUG=false
ENVIRONMENT=production
```

### 4. Start Services

```bash
# Start all services
podman-compose up -d

# Check status
podman-compose ps

# View logs
podman-compose logs -f
```

### 5. Configure Nginx Reverse Proxy

```bash
sudo nano /etc/nginx/sites-available/polygraph
```

```nginx
# Backend API
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# Frontend
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/polygraph /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Restart Nginx
sudo systemctl restart nginx
```

### 6. SSL with Let's Encrypt

```bash
# Obtain certificates
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
sudo certbot --nginx -d api.yourdomain.com

# Auto-renewal is set up automatically
sudo systemctl status certbot.timer
```

### 7. Firewall Configuration

```bash
# Allow HTTP, HTTPS, and SSH
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Enable firewall
sudo ufw enable
```

## Database Initialization

### ArangoDB

```bash
# Access ArangoDB web interface
# https://api.yourdomain.com:8529

# Or use CLI
podman exec -it polygraph-arangodb arangosh

# Create initial admin user, databases, etc.
```

### Backups

Create backup script:

```bash
sudo nano /opt/backup-polygraph.sh
```

```bash
#!/bin/bash
BACKUP_DIR="/opt/backups/polygraph"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p $BACKUP_DIR

# Backup ArangoDB
podman exec polygraph-arangodb arangodump \
  --output-directory /tmp/backup \
  --server.password changeme

podman cp polygraph-arangodb:/tmp/backup \
  $BACKUP_DIR/arango_$DATE

# Backup XTDB
podman exec polygraph-xtdb tar czf /tmp/xtdb_backup.tar.gz /var/xtdb
podman cp polygraph-xtdb:/tmp/xtdb_backup.tar.gz \
  $BACKUP_DIR/xtdb_$DATE.tar.gz

# Remove old backups (keep 30 days)
find $BACKUP_DIR -type f -mtime +30 -delete

echo "Backup completed: $DATE"
```

```bash
# Make executable
sudo chmod +x /opt/backup-polygraph.sh

# Add to crontab (daily at 2 AM)
echo "0 2 * * * /opt/backup-polygraph.sh" | sudo crontab -
```

## Monitoring

### System Monitoring

```bash
# Install monitoring tools
sudo apt install -y prometheus prometheus-node-exporter grafana

# Configure Prometheus
sudo nano /etc/prometheus/prometheus.yml
```

### Application Logs

```bash
# View all logs
podman-compose logs -f

# View specific service
podman-compose logs -f backend

# Export logs to file
podman-compose logs > /var/log/polygraph.log
```

### Health Checks

Create monitoring script:

```bash
#!/bin/bash
# /opt/health-check.sh

# Check API health
if curl -f http://localhost:8000/health > /dev/null 2>&1; then
    echo "API: OK"
else
    echo "API: FAILED"
    # Send alert
fi

# Check frontend
if curl -f http://localhost:3000 > /dev/null 2>&1; then
    echo "Frontend: OK"
else
    echo "Frontend: FAILED"
    # Send alert
fi
```

## Scaling

### Horizontal Scaling

For high traffic, run multiple backend instances:

```yaml
# In compose.yaml
services:
  backend:
    deploy:
      replicas: 3
```

Use load balancer (Nginx):

```nginx
upstream backend {
    least_conn;
    server localhost:8000;
    server localhost:8001;
    server localhost:8002;
}

server {
    location / {
        proxy_pass http://backend;
    }
}
```

### Database Scaling

- **ArangoDB**: Set up cluster mode
- **Dragonfly**: Configure replication
- **XTDB**: Use distributed deployment

## Security Checklist

- [ ] Change all default passwords
- [ ] Use strong secret keys
- [ ] Enable firewall
- [ ] Configure SSL/TLS
- [ ] Set up regular backups
- [ ] Enable fail2ban for SSH
- [ ] Configure rate limiting
- [ ] Set up monitoring and alerts
- [ ] Review and update dependencies regularly
- [ ] Enable audit logging
- [ ] Implement least-privilege access

## Troubleshooting

### Container won't start

```bash
# Check logs
podman logs polygraph-backend

# Check podman events
podman events

# Restart service
podman-compose restart backend
```

### Database connection issues

```bash
# Check database is running
podman ps | grep arango

# Test connection
podman exec -it polygraph-backend python -c "from app.db import arango_manager; arango_manager.connect()"
```

### Performance issues

```bash
# Check resource usage
podman stats

# Check disk space
df -h

# Check memory
free -h
```

## Updating

```bash
# Pull latest code
cd /opt/social-media-polygraph
sudo git pull

# Rebuild containers
cd infrastructure/podman
podman-compose build

# Restart with new images
podman-compose up -d

# Check logs
podman-compose logs -f
```

## Rollback

```bash
# Stop containers
podman-compose down

# Checkout previous version
git checkout <previous-commit>

# Rebuild and start
podman-compose build
podman-compose up -d
```
