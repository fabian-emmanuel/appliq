# 📌 AppliQ

A seamless web application that helps you efficiently track all your job applications, including statuses, types, and relevant metadata.

Built with ❤️ using **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.

---

## ✨ Features

- 📝 Register job applications with metadata (company, position, source, etc.)
- 📊 Track application statuses (e.g., Applied, Interview, Offer)
- 🔐 JWT-based Authentication
- 🔍 Filter applications by status, date range, and search keyword
- 📑 Auto-generated OpenAPI documentation with `utoipa`
- 🚀 Ready for deployment

---

## 🛠️ Tech Stack

| Layer      | Tech                     |
|------------|--------------------------|
| Language   | Rust                     |
| Framework  | Axum                     |
| ORM        | SQLx (Async, PostgreSQL) |
| Auth       | JWT                      |
| Docs       | utoipa + Swagger UI      |
| Database   | PostgreSQL               |
| Migrations | SQLx CLI                 |

---

## 🚀 Getting Started

### Prerequisites

- Rust (nightly or stable)
- PostgreSQL (e.g., via Docker)
- SQLx CLI:
  ```sh
  cargo install sqlx-cli --no-default-features --features postgres
  ```

### 🔧 Setup

1. **Clone the repo**

   ```bash
   git clone https://github.com/your-username/job-application-tracker.git
   cd appliq
   ```

2. **Create `.env` file**

   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/job_tracker
   JWT_SECRET=your_jwt_secret
   ```

3. **Run migrations**

   ```bash
   sqlx database create
   sqlx migrate run
   ```

4. **Start the server**

   ```bash
   cargo run
   ```

---

## 📘 API Documentation

Once the server is running, visit:

```
http://localhost:3000/api-docs
```

This shows the full OpenAPI docs powered by `utoipa`.

---

## 🧪 Sample Endpoints

| Method | Path                   | Description                   |
|--------|------------------------|-------------------------------|
| POST   | `/auth/login`          | Authenticate user             |
| POST   | `/user/register`       | Register a new user           |
| GET    | `/user/me`             | Fetch user data               |
| POST   | `/applications`        | Register a new application    |
| POST   | `/applications/status` | Add a new status              |
| GET    | `/applications`        | Filter and fetch applications |

---

## 📂 Project Structure

```
src/
├── handlers/           # HTTP handlers
├── services/           # Business logic
├── repositories/       # Database layer
├── models/             # Structs & DB models
├── middlewares/        # Auth and guards
├── config.rs           # App configuration
└── main.rs             # App entrypoint
```

---

## 👨‍💻 Author

**Fabian Emmanuel**\
Email: [emmanuel.fabian.dev@gmail.com](emmanuel.fabian.dev@gmail.com)\
Website: [yourwebsite.com](https://yourwebsite.com)\
Twitter: [@yourhandle](https://twitter.com/yourhandle)
