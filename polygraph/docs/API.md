# API Documentation

## Base URL

```
http://localhost:8000/api/v1
```

## Authentication

The API supports two authentication methods:

### 1. JWT Bearer Token

```bash
# Login to get token
curl -X POST "http://localhost:8000/api/v1/auth/login" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=your_username&password=your_password"

# Use token in requests
curl -X GET "http://localhost:8000/api/v1/auth/me" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### 2. API Key

```bash
curl -X POST "http://localhost:8000/api/v1/claims/verify" \
  -H "X-API-Key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"text": "claim to verify"}'
```

## Endpoints

### Claims

#### Verify Claim

Verify a social media claim.

**Endpoint:** `POST /claims/verify`

**Request:**

```json
{
  "text": "The claim text to verify",
  "url": "https://example.com/post",
  "platform": "twitter",
  "author": "@username",
  "metadata": {}
}
```

**Response:**

```json
{
  "success": true,
  "claim_id": "abc123",
  "analysis": {
    "claim": {
      "id": "abc123",
      "text": "The claim text",
      "status": "verified",
      "created_at": "2024-01-15T12:00:00"
    },
    "verification": {
      "verdict": "mixed",
      "confidence": 0.75,
      "explanation": "Detailed explanation...",
      "sources": [],
      "fact_checks": [
        {
          "source": "FactChecker",
          "verdict": "mixed",
          "rating": 0.7,
          "url": "https://..."
        }
      ],
      "entities": ["Entity 1", "Entity 2"],
      "sentiment": {
        "polarity": 0.1,
        "subjectivity": 0.6,
        "classification": "neutral"
      },
      "credibility_score": 0.65
    },
    "temporal_history": []
  },
  "processing_time": 1.234
}
```

#### Get Claim

Get analysis for a previously verified claim.

**Endpoint:** `GET /claims/{claim_id}`

**Response:** Same as verification response analysis object.

#### List Claims

List all claims with pagination.

**Endpoint:** `GET /claims/`

**Query Parameters:**
- `skip` (int, default: 0): Number of records to skip
- `limit` (int, default: 100, max: 1000): Maximum records to return
- `status` (string, optional): Filter by status

**Response:**

```json
[
  {
    "id": "abc123",
    "text": "Claim text",
    "status": "verified",
    "created_at": "2024-01-15T12:00:00"
  }
]
```

#### Get Claim History

Get temporal history of claim verifications.

**Endpoint:** `GET /claims/{claim_id}/history`

**Response:**

```json
[
  {
    "verified_at": "2024-01-15T12:00:00",
    "verdict": "true",
    "confidence": 0.85,
    "credibility_score": 0.8
  }
]
```

### Authentication

#### Register

Create a new user account.

**Endpoint:** `POST /auth/register`

**Request:**

```json
{
  "email": "user@example.com",
  "username": "username",
  "password": "secure_password",
  "full_name": "Full Name"
}
```

**Response:**

```json
{
  "id": "user123",
  "email": "user@example.com",
  "username": "username",
  "is_active": true,
  "created_at": "2024-01-15T12:00:00"
}
```

#### Login

Authenticate and receive JWT tokens.

**Endpoint:** `POST /auth/login`

**Request (form data):**

```
username=user@example.com
password=secure_password
```

**Response:**

```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "bearer"
}
```

#### Get Current User

Get authenticated user information.

**Endpoint:** `GET /auth/me`

**Headers:** `Authorization: Bearer YOUR_TOKEN`

**Response:**

```json
{
  "id": "user123",
  "email": "user@example.com",
  "username": "username",
  "full_name": "Full Name",
  "is_active": true
}
```

#### Create API Key

Generate a new API key.

**Endpoint:** `POST /auth/api-keys`

**Headers:** `Authorization: Bearer YOUR_TOKEN`

**Request:**

```json
{
  "name": "My API Key",
  "rate_limit": 100
}
```

**Response:**

```json
{
  "api_key": {
    "id": "key123",
    "name": "My API Key",
    "is_active": true,
    "created_at": "2024-01-15T12:00:00",
    "rate_limit": 100
  },
  "key": "sk_abc123..."
}
```

**Note:** The actual key is only shown once!

## Rate Limiting

API endpoints are rate-limited:
- Default: 100 requests per minute
- Custom limits can be set per API key

Rate limit headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640000000
```

## Error Responses

All error responses follow this format:

```json
{
  "detail": "Error message",
  "status_code": 400
}
```

Common error codes:
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `429` - Too Many Requests
- `500` - Internal Server Error

## Webhooks (Coming Soon)

Subscribe to events:
- `claim.verified` - When a claim is verified
- `claim.updated` - When claim status changes
