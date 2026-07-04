# Rust Clean API

A **Rust** REST API built with [Axum](https://github.com/tokio-rs/axum), following a **Clean Architecture** approach combined with **Domain-Driven Design (DDD)** principles. The project implements **Auth, User, Role, Permission**, and **Authorization (RBAC)** modules on top of a **MySQL** database.

## ✨ Features

- **JWT authentication** — login, refresh token, logout, and logout-from-all-devices (`refresh_tokens` table tracks device & IP)
- **RBAC (Role-Based Access Control)** — manage Users, Roles, and Permissions, plus their many-to-many relations (`user_roles`, `role_permissions`)
- **Authorization endpoint** — `GET /authorize/me` to check the profile & access rights of the currently logged-in user
- **Audit logging** — table structure for tracking data changes (resource, action, old/new value, IP, user agent)
- **Menus & menu permissions** — table structure for dynamic, permission-based menus (useful for RBAC-driven sidebars)
- **Swagger / OpenAPI** — auto-generated API docs via `utoipa` + `utoipa-swagger-ui`
- **Ready-to-use middleware** — CORS, compression, request timeout, request ID, logging/tracing, auth guard, and permission guard
- **Layered configuration** — `configs/app.toml` plus environment variables (`.env`) via `config`, `dotenvy`, and `envy`
- **Automatic database migration** on startup (see `bootstrap/migration.rs`)

## 🏗️ Architecture

The project is organized following **Clean Architecture** per domain (rather than per technical layer), so each domain is self-contained and easy to extend:

```
src/
├── main.rs                # Application entry point
├── lib.rs                 # Root module declarations
├── swagger.rs             # OpenAPI/Swagger configuration
│
├── bootstrap/             # Application wiring (composition root)
│   ├── app.rs              # Startup orchestration (state → migration → server)
│   ├── dependency.rs       # Dependency injection (repository → service)
│   ├── state.rs            # AppState, Infrastructure, Services definitions
│   ├── router.rs           # Root router + swagger UI
│   ├── server.rs           # HTTP server listener
│   └── migration.rs        # Runs migrations automatically on startup
│
├── config/                # Configuration loader (app.toml + .env)
│
├── domain/                # Each domain has its own DDD structure:
│   │                       #   dto/ · entity/ · repository/ · service.rs
│   │                       #   handler.rs · routes.rs · mod.rs
│   ├── auth/               # Login, refresh token, logout
│   ├── user/               # User CRUD + assign/revoke role
│   ├── role/               # Role CRUD + assign/revoke permission
│   ├── permission/         # Permission CRUD
│   ├── role_permission/    # Role ↔ Permission relation
│   ├── user_role/          # User ↔ Role relation
│   └── authorization/      # `/me` endpoint (profile & access rights)
│
├── infrastructure/        # Technical implementations: DB connection, JWT service, etc.
├── middleware/             # CORS, compression, timeout, logging, auth & permission guards
├── helper/                 # Cross-domain helper functions
├── common/                 # Shared types/structs (response wrapper, error types, etc.)
└── routes/                 # Aggregates all domain routes under `/api/v1`
```

**Request flow:** `routes` → `handler` → `service` (business logic) → `repository` (data access) → MySQL, with `dependency.rs` acting as the composition root that assembles every repository and service into `AppState`.

## 🧱 Database Schema

The migrations (`migrations/`) define the following tables:

| Table | Description |
|---|---|
| `users` | User data (soft delete via `deleted_at`) |
| `roles` | List of roles |
| `permissions` | List of permissions (`resource` + `action`) |
| `user_roles` | Many-to-many relation between users and roles |
| `role_permissions` | Many-to-many relation between roles and permissions |
| `refresh_tokens` | Per-device refresh tokens, supports revocation |
| `audit_logs` | Log of data changes |
| `menus` | Hierarchical menu (supports `parent_id`) |
| `menu_permissions` | Relation between menus and permissions |

## 🚀 Tech Stack

| Category | Library |
|---|---|
| Async runtime | `tokio` |
| Web framework | `axum`, `tower`, `tower-http` |
| Database | `sqlx` (MySQL, runtime-tokio-rustls) |
| Cache | `redis` |
| Auth | `jsonwebtoken`, `argon2` |
| Validation | `validator` |
| API documentation | `utoipa`, `utoipa-swagger-ui` |
| Configuration | `config`, `dotenvy`, `envy` |
| Logging | `tracing`, `tracing-subscriber` |
| Error handling | `thiserror`, `anyhow` |
| Others | `uuid`, `chrono`, `reqwest`, `sha2`, `once_cell`, `regex` |

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition, use the latest stable toolchain)
- MySQL 8.x
- (Optional) Redis, if you want to enable caching

## ⚙️ Installation & Running

1. **Clone the repository**
   ```bash
   git clone https://github.com/hoerilahyar/rust-clean-ddd.git
   cd rust-clean-ddd
   ```

2. **Copy and configure environment variables**
   ```bash
   cp .env.example .env
   ```
   At minimum, set `MYSQL_HOST`, `MYSQL_PORT`, `MYSQL_DATABASE`, `MYSQL_USERNAME`, `MYSQL_PASSWORD`, and `JWT_SECRET`.

3. **Adjust the application config** (optional) in `configs/app.toml` — port, timeouts, CORS, JWT expiry, etc.

4. **Prepare the MySQL database** matching the name in `.env` (`MYSQL_DATABASE`). Migrations run automatically on startup (see `bootstrap/migration.rs`).

5. **Run the application**
   ```bash
   cargo run
   ```
   Or use the scripts available in the `scripts/` folder (`build.sh`, `start.sh`, `test.sh`) as needed.

6. **Access the API documentation**
   Swagger UI is available at `http://localhost:8080/` (enable it via `[swagger] enabled = true` in `configs/app.toml`).

## 📡 Main Endpoints

All endpoints are served under the `/api/v1` prefix.

**Auth** (`/api/v1/auth`)
- `POST /login`
- `POST /refresh`
- `POST /logout`
- `POST /logout-all`

**Users** (`/api/v1/users`)
- `POST /` — create user
- `GET /` — list users
- `GET /{id}` — get user detail
- `PUT /{id}` — update user
- `DELETE /{id}` — delete user
- `PUT /{user_id}/roles` — assign role to user
- `GET /{user_id}/roles` — list user's roles
- `DELETE /{user_id}/roles/{role_id}` — revoke role from user

**Roles** (`/api/v1/roles`)
- `POST /`, `GET /`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`
- `PUT /{role_id}/permissions` — assign permission to role
- `GET /{role_id}/permissions` — list role's permissions
- `DELETE /{role_id}/permissions/{permission_id}` — revoke permission from role

**Authorization** (`/api/v1/authorize`)
- `GET /me` — profile & access rights of the currently logged-in user

> The `permission` module is also available as a standalone domain (full CRUD) for further wiring into `routes/api.rs` as needed.

## 🗂️ Other Folders

- `configs/` — application configuration files (`app.toml`)
- `migrations/` — SQL migration files, numbered sequentially
- `scripts/` — helper scripts for build/start/test

## 🤝 Contributing

This project is under active development. Contributions, suggestions, and architecture discussions are welcome via Issues or Pull Requests.

## 📄 License

Not yet specified — feel free to add a license as needed (e.g. MIT or Apache-2.0).
