# Badge Forge Documentation

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [API Endpoints](#api-endpoints)
- [Queue System](#queue-system)
- [Data Models](#data-models)
- [Level System](./level_system.md)
- [Badge Management](./badge_management.md)
- [Security](#security)
- [Setup Instructions](#setup-instructions)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Overview

Badge Forge is a service that calculates and assigns badges and levels to users based on their activity and recipe interactions. It processes user data asynchronously through a queue system to update user badges and levels in MongoDB.

The system works by receiving requests to update a user's badges and level, queuing these requests, and processing them in the background, thus preventing performance bottlenecks during high traffic periods.

## Architecture

Badge Forge follows a microservice architecture with the following components:

1. **API Server**: Handles incoming requests and exposes endpoints for badge update operations
2. **Queue System**: Manages pending badge update requests
3. **Badge Processor**: Background worker that processes queued requests
4. **MongoDB Integration**: Stores and retrieves user and recipe data

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐     ┌────────────┐
│  API Server │────▶│ Queue System │────▶│ Badge Processor │────▶│  MongoDB   │
└─────────────┘     └──────────────┘     └─────────────────┘     └────────────┘
        │                                         │                    │
        └─────────────────────────────────────────┼────────────────────┘
                                                  │
                                         Calculates & Updates
                                            User Badges
```

## API Endpoints

### Badge Update Endpoint

```
POST /update
```

Endpoint for requesting a badge update for a user. Protected by API key authentication.

**Headers:**
- `X-API-Key`: Your API key for authentication

**Request Body:**
```json
{
  "user_id": "669b7be8f163ac944bc8a16e"
}
```

**Response:**
```json
{
  "status": "queued",
  "message": "Badge update request has been queued for processing",
  "user_id": "669b7be8f163ac944bc8a16e",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2025-06-12T17:45:53Z"
}
```

### Queue Status Endpoint

```
GET /queue/status
```

Returns the current state of the badge update queue.

**Response:**
```json
{
  "status": "ok",
  "pending_count": 3,
  "pending_requests": [
    {
      "user_id": "669b7be8f163ac944bc8a16e",
      "request_id": "550e8400-e29b-41d4-a716-446655440000",
      "created_at": "2025-06-12T17:45:53Z"
    },
    {
      "user_id": "669b7be8f163ac944bc8a16f",
      "request_id": "550e8400-e29b-41d4-a716-446655440001",
      "created_at": "2025-06-12T17:46:12Z"
    }
  ]
}
```

### Health Check Endpoint

```
GET /health
```

Simple health check endpoint to verify service availability.

**Response:**
```json
{
  "status": "ok"
}
```

### Version Endpoint

```
GET /version
```

Returns the current version of the Badge Forge API.

**Response:**
```json
{
  "version": "0.1.0"
}
```

## Queue System

Badge Forge uses an in-memory queue system to manage badge update requests. Each request is uniquely identified by:

- `request_id`: A UUID that uniquely identifies each request
- `created_at`: Timestamp when the request was created
- `user_id`: The MongoDB ObjectID of the user to be updated

The queue system ensures:
- No duplicate processing of requests
- Proper tracking of pending requests
- Asynchronous processing to avoid blocking API responses

## Data Models

### LevelRequest

Represents a request to update a user's badges and level:

```rust
struct LevelRequest {
    user_id: String,      // MongoDB ObjectID of the user
    request_id: String,   // UUID of the request
    created_at: DateTime<Utc>, // When the request was created
}
```

### User

The user model in MongoDB:

```rust
struct User {
    _id: ObjectId,        // MongoDB ObjectID
    name: Option<String>, // User's name (optional)
    email: Option<String>, // User's email (optional)
    level: i32,          // Current user level
    badges: Vec<String>,  // Array of badge identifiers
}
```

### Recipe

The recipe model in MongoDB:

```rust
struct Recipe {
    _id: ObjectId,        // MongoDB ObjectID
    user_id: ObjectId,    // Creator of the recipe
    num_likes: i32,       // Number of likes the recipe has
    created_at: DateTime<Utc>, // When the recipe was created
}
```

## Security

Badge Forge implements API key authentication for the badge update endpoint. The API key is configured via environment variables and must be provided in the `X-API-Key` header for all update requests.

```
X-API-Key: your_api_key_here
```

## Setup Instructions

### Prerequisites

- Rust 1.56 or higher
- MongoDB 4.4 or higher
- Docker (optional)

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/jorbushvale/badge-forge.git
cd badge-forge
```

2. Build the project:
```bash
cargo build --release
```

3. Run the service:
```bash
./target/release/badge_forge
```

### Docker Deployment

```bash
# Build Docker image
docker build -t badge-forge .

# Run container
docker run -d -p 4000:4000 \
  -e MONGODB_URI="mongodb://mongo:27017" \
  -e DB_NAME="badgeforge" \
  -e API_KEY="your_secure_key_here" \
  --name badge-forge \
  badge-forge
```

## Configuration

Badge Forge is configured through environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `MONGODB_URI` | MongoDB connection string | `mongodb://localhost:27017` |
| `DB_NAME` | MongoDB database name | `badgeforge` |
| `API_KEY` | API Key for authentication | `default_key` |

You can use a `.env` file for local development.

## Troubleshooting
### Common Issues

**Error: User not found**
- Ensure the user exists in the MongoDB database
- Check that the user ID is in the correct format (valid MongoDB ObjectID)

**Error: Failed to parse ObjectId**
- The user_id provided is not a valid MongoDB ObjectID format
- Expected format: 24 character hex string (e.g. `669b7be8f163ac944bc8a16e`)

**Error: Invalid type in date field**
- Recipe documents in MongoDB might have inconsistent date formats
- Ensure all recipes have valid date formats

**401 Unauthorized**
- Check that you're providing the correct API key in the `X-API-Key` header

For more assistance, check the application logs which contain detailed error information.
