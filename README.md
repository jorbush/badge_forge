# Badge Forge 🎖️

Badge Forge is a high-performance microservice that calculates and assigns achievement badges and experience levels to users on the [Jorbites](https://github.com/jorbush/jorbites) platform. It uses an asynchronous queue system to efficiently process badge updates without blocking the main application flow.

![logo](/docs/assets/badge_forge_logo.png)

## 🌟 Features

- **Asynchronous Processing**: Uses a non-blocking queue system to handle badge updates
- **User Levels**: Automatically calculates user levels based on recipe count and engagement metrics
- **Achievement Badges**: Assigns badges for various milestones and accomplishments
- **API Security**: Secured endpoints with API key authentication
- **Queue Monitoring**: Real-time visibility into the badge update queue
- **MongoDB Integration**: Seamless integration with MongoDB for data storage

## 🏗️ Architecture

Badge Forge follows a modular microservice architecture:

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐     ┌────────────┐
│  API Server │────▶│ Queue System │────▶│ Badge Processor │────▶│  MongoDB   │
└─────────────┘     └──────────────┘     └─────────────────┘     └────────────┘
```

1. **API Server**: Handles incoming requests and authentication
2. **Queue System**: Manages and prioritizes badge update requests
3. **Badge Processor**: Calculates levels and assigns badges based on user activity
4. **MongoDB**: Stores user data, recipes, and badge information


## 📋 Requirements

- Docker or Rust.

## 🚀 Quick Start

### Environment Variables

Create a `.env` file in the project root with the following:

```
MONGODB_URI=mongodb://localhost:27017
DB_NAME=badgeforge
API_KEY=your_secure_key_here
```

### Using Rust

```bash
make run
```

### Using Docker

```bash
make docker
```

## 🧪 Development

### Testing

```bash
cargo test
```

### Formatting and Linting

```bash
# Format code
cargo fmt

# Lint code
cargo clippy
```

## 🔌 API Endpoints

### Update User Badges
```
POST /update
Header: X-API-Key: your_api_key_here
Body: { "user_id": "5f8d0d55b54764421b7156d3" }
```

### Queue Status
```
GET /queue/status
```

### Health Check
```
GET /health
```

### Version Info
```
GET /version
```


## 📄 Documentation

Comprehensive documentation is available in the [docs](./docs/README.md) directory, including:

- Complete API reference
- Detailed architecture diagrams
- Badge calculation algorithms
- MongoDB schema details
- Deployment guides
