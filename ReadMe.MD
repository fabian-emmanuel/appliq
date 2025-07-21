# 📌 AppliQ - Application Tracking System

A seamless web application that helps you efficiently track all your applications, including statuses, types, and relevant metadata. Built with modern technologies for reliability, performance, and scalability.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

## 🚀 Quick Start with Docker

The fastest way to get AppliQ running locally is using Docker Compose. This method handles all dependencies and configuration automatically.

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) (version 20.10 or higher)
- [Docker Compose](https://docs.docker.com/compose/install/) (version 2.0 or higher)
- Git

### 🐳 Running with Docker Compose

1. **Clone the repository**
   ```bash
   git clone https://github.com/fabian-emmanuel/appliq.git
   cd appliq
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```

   Edit the `.env` file with your preferred settings:
   ```bash
   nano .env
   ```

3. **Start all services**
   ```bash
   docker-compose up -d
   ```

4. **Run database migrations**
   ```bash
   docker-compose --profile migration up migrate
   ```

5. **Access the application**
    - API: http://localhost:3000
    - API Documentation: http://localhost:3000/api-docs
    - Database: localhost:5432
    - RabbitMQ Management: http://localhost:15672 (guest/guest)

### 🛑 Stopping the Application

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (⚠️ This will delete all data)
docker-compose down -v
```

## 📋 Table of Contents

- [Features](#-features)
- [Architecture](#-architecture)
- [Development Setup](#-development-setup)
- [API Documentation](#-api-documentation)
- [Database Schema](#-database-schema)
- [Environment Configuration](#-environment-configuration)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

- **Application Management**: Register and track job applications with comprehensive metadata
- **Status Tracking**: Monitor application progress through various stages (Applied, Interview, Offer, etc.)
- **JWT Authentication**: Secure user authentication and authorization
- **Advanced Filtering**: Search and filter applications by status, date range, and keywords
- **RESTful API**: Well-documented REST API with OpenAPI/Swagger documentation
- **Real-time Updates**: WebSocket support for real-time application status updates
- **Email Notifications**: Automated email notifications for application updates
- **Data Export**: Export application data in various formats (CSV, JSON, PDF)

## 🏗️ Architecture

AppliQ follows a clean architecture pattern with clear separation of concerns:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Handlers      │    │   Services      │    │  Repositories   │
│  (HTTP Layer)   │───▶│ (Business Logic)│───▶│ (Data Access)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Middlewares   │    │     Utils       │    │   PostgreSQL    │
│ (Auth, CORS)    │    │ (JWT, Helpers)  │    │   Database      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Backend Framework** | Axum | High-performance async web framework |
| **Language** | Rust | Memory-safe systems programming |
| **Database** | PostgreSQL | Reliable relational database |
| **ORM** | SQLx | Async SQL toolkit with compile-time checks |
| **Authentication** | JWT | Stateless authentication tokens |
| **Message Queue** | RabbitMQ | Async task processing |
| **Documentation** | utoipa + Swagger UI | Interactive API documentation |
| **Containerization** | Docker + Docker Compose | Consistent development environment |

## 🛠️ Development Setup

### Option 1: Docker Development (Recommended)

This is the easiest way to get started and ensures consistency across different development environments.

#### Prerequisites
- Docker Desktop or Docker Engine + Docker Compose
- Git

#### Setup Steps

1. **Clone and navigate to the project**
   ```bash
   git clone https://github.com/fabian-emmanuel/appliq.git
   cd appliq
   ```

2. **Create environment file**
   ```bash
   cp .env.example .env
   ```

3. **Start development environment**
   ```bash
   # Start all services in development mode
   docker-compose up -d
   
   # View logs
   docker-compose logs -f app
   ```

4. **Run migrations**
   ```bash
   docker-compose --profile migration up migrate
   ```

5. **Verify setup**
   ```bash
   # Check if all services are running
   docker-compose ps
   
   # Test API endpoint
   curl http://localhost:3000/health
   ```

#### Development Workflow

```bash
# Rebuild application after code changes
docker-compose up --build app

# Run tests
docker-compose exec app cargo test

# Access database
docker-compose exec db psql -U postgres -d appliq

# View application logs
docker-compose logs -f app

# Stop development environment
docker-compose down
```

### Option 2: Native Development

For developers who prefer running services natively or need to debug at a lower level.

#### Prerequisites
- Rust (stable or nightly)
- PostgreSQL 13+
- SQLx CLI
- Git

#### Installation Steps

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install SQLx CLI**
   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

3. **Install and start PostgreSQL**
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install postgresql postgresql-contrib
   sudo systemctl start postgresql
   
   # macOS with Homebrew
   brew install postgresql
   brew services start postgresql
   
   # Create database
   sudo -u postgres createdb appliq
   ```

4. **Clone and setup project**
   ```bash
   git clone https://github.com/fabian-emmanuel/appliq.git
   cd appliq
   cp .env.example .env
   ```

5. **Configure environment**
   Edit `.env` file:
   ```env
   DATABASE_URL=postgres://postgres:password@localhost:5432/appliq
   JWT_SECRET=your_development_secret
   RUST_LOG=debug
   ```

6. **Run migrations and start application**
   ```bash
   sqlx database create
   sqlx migrate run
   cargo run
   ```

## 📚 API Documentation

AppliQ provides comprehensive API documentation through Swagger UI, automatically generated from the code using `utoipa`.

### Accessing Documentation

Once the application is running, visit:
- **Swagger UI**: http://localhost:3000/api-docs
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

### Authentication

Most endpoints require JWT authentication. To authenticate:

1. **Register a new user**
   ```bash
   curl -X POST http://localhost:3000/user/register \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "password": "securepassword",
       "first_name": "John",
       "last_name": "Doe"
     }'
   ```

2. **Login to get JWT token**
   ```bash
   curl -X POST http://localhost:3000/auth/login \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "password": "securepassword"
     }'
   ```

3. **Use token in subsequent requests**
   ```bash
   curl -X GET http://localhost:3000/user/me \
     -H "Authorization: Bearer YOUR_JWT_TOKEN"
   ```

### Core Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `POST` | `/user/register` | Register new user | ❌ |
| `POST` | `/auth/login` | User authentication | ❌ |
| `GET` | `/user/me` | Get current user info | ✅ |
| `POST` | `/applications` | Create new application | ✅ |
| `GET` | `/applications` | List applications with filters | ✅ |
| `GET` | `/applications/{id}` | Get specific application | ✅ |
| `PUT` | `/applications/{id}` | Update application | ✅ |
| `DELETE` | `/applications/{id}` | Delete application | ✅ |
| `POST` | `/applications/{id}/status` | Add status update | ✅ |
| `GET` | `/health` | Health check | ❌ |

### Example API Usage

#### Creating an Application
```bash
curl -X POST http://localhost:3000/applications \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "company": "Tech Corp",
    "position": "Senior Software Engineer",
    "location": "San Francisco, CA",
    "application_url": "https://techcorp.com/careers/123",
    "source": "LinkedIn",
    "notes": "Referred by John Smith"
  }'
```

#### Filtering Applications
```bash
# Get applications with specific status
curl "http://localhost:3000/applications?status=interview" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# Search applications by company
curl "http://localhost:3000/applications?search=google" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# Get applications within date range
curl "http://localhost:3000/applications?from_date=2024-01-01&to_date=2024-12-31" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## 🗄️ Database Schema

AppliQ uses PostgreSQL with a well-structured schema designed for scalability and data integrity.

### Core Tables

#### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    role VARCHAR(50) DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Applications Table
```sql
CREATE TABLE applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    company VARCHAR(255) NOT NULL,
    position VARCHAR(255) NOT NULL,
    location VARCHAR(255),
    application_url TEXT,
    source VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Application Statuses Table
```sql
CREATE TABLE application_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL,
    notes TEXT,
    interview_type VARCHAR(100),
    interview_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Tokens Table
```sql
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type VARCHAR(50) NOT NULL,
    token_value VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Database Migrations

Migrations are managed using SQLx CLI and are located in the `db/migrations/` directory. Each migration file follows the naming convention: `YYYYMMDDHHMMSS_description.sql`.

#### Running Migrations

```bash
# With Docker
docker-compose --profile migration up migrate

# Native development
sqlx migrate run

# Check migration status
sqlx migrate info
```

#### Creating New Migrations

```bash
# Create a new migration
sqlx migrate add create_new_table

# This creates: db/migrations/YYYYMMDDHHMMSS_create_new_table.sql
```

## ⚙️ Environment Configuration

AppliQ uses environment variables for configuration. All available options are documented in `.env.example`.

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost:5432/appliq` |
| `JWT_SECRET` | Secret key for JWT token signing | `your_super_secure_secret` |

### Optional Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level (error, warn, info, debug, trace) |
| `BIND_ADDRESS` | `0.0.0.0:3000` | Server bind address and port |
| `APP_ENV` | `development` | Application environment |
| `RABBITMQ_URL` | `amqp://guest:guest@localhost:5672/` | RabbitMQ connection string |

### Production Configuration

For production deployments, ensure you:

1. **Use strong secrets**
   ```env
   JWT_SECRET=$(openssl rand -base64 32)
   ```

2. **Configure secure database connection**
   ```env
   DATABASE_URL=postgres://username:password@db-host:5432/appliq?sslmode=require
   ```

3. **Set appropriate log level**
   ```env
   RUST_LOG=warn
   APP_ENV=production
   ```

4. **Configure email settings** (if using email features)
   ```env
   SMTP_HOST=smtp.gmail.com
   SMTP_PORT=587
   SMTP_USERNAME=your-app@domain.com
   SMTP_PASSWORD=your-app-password
   ```

## 🔧 Troubleshooting

### Common Issues and Solutions

#### Docker Issues

**Problem**: `docker-compose up` fails with port conflicts
```
Error: bind: address already in use
```

**Solution**: Check for conflicting services and stop them
```bash
# Check what's using the port
sudo lsof -i :3000
sudo lsof -i :5432

# Stop conflicting services or change ports in docker-compose.yml
```

**Problem**: Database connection refused
```
Error: connection refused (os error 111)
```

**Solution**: Ensure database is ready before starting app
```bash
# Wait for database to be ready
docker-compose up db
# Wait a few seconds, then start app
docker-compose up app
```

#### Migration Issues

**Problem**: Migration fails with "database does not exist"
```bash
# Create database first
docker-compose exec db createdb -U postgres appliq

# Then run migrations
docker-compose --profile migration up migrate
```

**Problem**: Migration version conflicts
```bash
# Check migration status
docker-compose exec app sqlx migrate info

# Reset migrations (⚠️ This will delete all data)
docker-compose exec app sqlx database reset
```

#### Application Issues

**Problem**: JWT token expired or invalid
```json
{"error": "Invalid token"}
```

**Solution**: Login again to get a new token
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password"}'
```

**Problem**: CORS errors in browser
```
Access to fetch at 'http://localhost:3000' from origin 'http://localhost:8080' has been blocked by CORS policy
```

**Solution**: The application includes CORS middleware. Ensure you're using the correct origin or check CORS configuration in the code.

### Performance Optimization

#### Database Performance
```sql
-- Add indexes for frequently queried columns
CREATE INDEX idx_applications_user_id ON applications(user_id);
CREATE INDEX idx_applications_company ON applications(company);
CREATE INDEX idx_application_statuses_application_id ON application_statuses(application_id);
CREATE INDEX idx_application_statuses_status ON application_statuses(status);
```

#### Application Performance
```bash
# Build optimized release version
docker-compose build --build-arg RUST_ENV=release app

# Monitor resource usage
docker stats appliq-app appliq-db
```

### Debugging

#### Enable Debug Logging
```env
RUST_LOG=debug
```

#### Access Application Logs
```bash
# View real-time logs
docker-compose logs -f app

# View database logs
docker-compose logs -f db

# View all logs
docker-compose logs -f
```

#### Database Debugging
```bash
# Connect to database
docker-compose exec db psql -U postgres -d appliq

# View table contents
\dt  -- List tables
SELECT * FROM users LIMIT 5;
SELECT * FROM applications LIMIT 5;
```

## 🤝 Contributing

We welcome contributions to AppliQ! Please follow these guidelines:

### Development Process

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Write tests** for new functionality
5. **Run the test suite**
   ```bash
   docker-compose exec app cargo test
   ```
6. **Submit a pull request**

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run Clippy for linting: `cargo clippy`
- Ensure all tests pass: `cargo test`
- Update documentation for new features

### Commit Messages

Use conventional commit format:
```
feat: add user profile management
fix: resolve database connection timeout
docs: update API documentation
test: add integration tests for auth
```

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Fabian Emmanuel**
- Email: emmanuel.fabian.dev@gmail.com
- GitHub: [@fabian-emmanuel](https://github.com/fabian-emmanuel)

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation generator
- [tokio](https://tokio.rs/) - Async runtime for Rust

---

**Happy coding! 🚀**

For questions, issues, or feature requests, please [open an issue](https://github.com/fabian-emmanuel/appliq/issues) on GitHub.

