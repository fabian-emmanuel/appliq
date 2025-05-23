# 📌 AppliQ

A seamless web application that helps you efficiently track all your applications, including statuses, types, and relevant metadata.

Built with ❤️ using **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.

---

## 📚 Table of Contents

- [✨ Features](#-features)
- [🛠️ Tech Stack](#️-tech-stack)
- [🚀 Getting Started](#-getting-started)
    - [Prerequisites](#prerequisites)
    - [🔧 Setup](#-setup)
- [📘 API Documentation](#-api-documentation)
- [🧪 Sample Endpoints](#-sample-endpoints)
- [📂 Project Structure](#-project-structure)
- [🗄️ Database Schema](#️-database-schema)
- [🤝 Contributing](#-contributing)
- [👨‍💻 Author](#-author)

---

## ✨ Features

- 📝 Register applications with metadata (company, position, source, etc.)
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
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/appliq
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
├── handlers/           # HTTP request handlers, interfacing with services.
├── services/           # Business logic layer, orchestrates operations.
├── repositories/       # Data access layer, interacts with the database.
├── models/             # Database table representations and other data structures.
├── enums/              # Application-specific enumerations (e.g., roles, statuses).
├── errors/             # Custom error types for the application.
├── payloads/           # Structs for API request bodies and response payloads.
├── utils/              # Utility functions and modules (e.g., JWT handling, API response formatting).
├── middlewares/        # Axum middleware for request processing (e.g., authentication).
├── configs/            # Application configuration (e.g., routes, API documentation setup).
├── config.rs           # Deprecated or specific app configuration (review if still used alongside `configs/`).
└── main.rs             # Application entry point, sets up router and services.

db/
└── migrations/         # SQL files for database schema migrations, managed by SQLx CLI.

Other important files:
├── .env                # Local environment variables (DATABASE_URL, JWT_SECRET). **Should not be committed.**
├── .env.example        # Example environment file. Copy to .env and fill in your values.
├── Cargo.toml          # Rust project manifest, defines dependencies and project metadata.
├── Dockerfile          # Instructions for building the application's Docker image.
└── compose.yml         # Docker Compose file for running the application and its services (e.g., PostgreSQL).
```

---

## 🗄️ Database Schema

The application uses **PostgreSQL** as its database. The schema is managed via SQLx migrations located in the `db/migrations/` directory.

Key tables include:

-   **`users`**: Stores user information, credentials, and roles.
-   **`applications`**: Contains details about applications, such as company/institution, position, and application website.
-   **`application_statuses`**: Tracks the status history for each application (e.g., Applied, Interviewing, Offer, Rejected). Each entry links to an application and records a specific status at a point in time, optionally including details like test or interview types.
-   **`tokens`**: Stores tokens used for purposes like password resets or email verification.

For detailed table structures, relationships, and schema evolution, please refer to the SQL migration files in `db/migrations/`.

---

## 🤝 Contributing

Contributions are welcome! If you'd like to contribute to AppliQ, please follow these guidelines:

1.  **Fork the repository** on GitHub.
2.  **Create a new branch** for your feature or bug fix:
    ```bash
    git checkout -b feature/your-feature-name
    ```
    or
    ```bash
    git checkout -b fix/your-bug-fix-name
    ```
3.  **Follow the existing code style**. Ensure your code is well-formatted and commented where necessary.
4.  **Write tests** for any new functionality or bug fixes to ensure robustness.
5.  **Commit your changes** with clear and descriptive commit messages.
6.  **Push your branch** to your forked repository:
    ```bash
    git push origin feature/your-feature-name
    ```
7.  **Submit a pull request** to the main AppliQ repository. Provide a clear description of the changes and any relevant context.

We appreciate your help in making AppliQ better!

---

## 👨‍💻 Author

**Fabian Emmanuel**\
Email: [emmanuel.fabian.dev@gmail.com](emmanuel.fabian.dev@gmail.com)\
Website: [yourwebsite.com](https://yourwebsite.com)\
Twitter: [@yourhandle](https://twitter.com/yourhandle)
