# Rust Clean API

A **Rust** REST API built with [Axum](https://github.com/tokio-rs/axum), following a **Clean Architecture** approach combined with **Domain-Driven Design (DDD)** principles, on top of a **MySQL** database. It ships as a ready-to-extend RBAC + audit + settings foundation for building new applications.

## ✨ Features

- **JWT authentication** — login, refresh token, logout, and logout-from-all-devices (`refresh_tokens` table tracks device & IP)
- **Session management** — list active sessions, revoke a specific session, revoke all other sessions
- **RBAC (Role-Based Access Control)** — Users, Roles, Permissions, plus their many-to-many relations (`user_roles`, `role_permissions`)
- **Authorization endpoint** — `GET /iam/authorization/me` to check the profile & access rights of the currently logged-in user
- **Audit logging** — fully wired: every write operation across domains records actor, action, entity, IP, user agent, and metadata; queryable via API
- **Dynamic menus** — hierarchical menu (`parent_id`) with menu ↔ permission relations, for building RBAC-driven sidebars
- **System settings & user settings** — key-value settings store, both global (`system_settings`) and per-user (`user_settings`), each with active/inactive toggling
- **Swagger / OpenAPI** — auto-generated API docs via `utoipa` + `utoipa-swagger-ui`
- **Ready-to-use middleware** — CORS, compression, request timeout, request ID propagation, tracing, JWT auth guard
- **Layered configuration** — `configs/app.toml` plus environment variables (`.env`) via `config`, `dotenvy`, and `envy`
- **Automatic database migration** on startup (see `bootstrap/migration.rs`)

## 🏗️ Architecture

The project is organized following **Clean Architecture** per domain (rather than per technical layer), so each domain is self-contained and easy to extend:

```
src/
├── main.rs                # Application entry point
├── lib.rs                 # Root module declarations
├── swagger.rs             # OpenAPI/Swagger configuration (all documented endpoints)
│
├── bootstrap/             # Application wiring (composition root)
│   ├── app.rs               # Startup orchestration (state → migration → server)
│   ├── dependency.rs         # Dependency injection (repository → service)
│   ├── state.rs              # AppState, Infrastructure, Services definitions
│   ├── router.rs             # Root router + global middleware + swagger UI
│   ├── server.rs              # HTTP server listener
│   └── migration.rs           # Runs migrations automatically on startup
│
├── config/                # Configuration loader (app.toml + .env)
│
├── domain/                # Each domain has its own DDD structure:
│   │                       #   dto/ · entity/ · repository/ · service.rs
│   │                       #   handler.rs · routes.rs · mod.rs
│   ├── auth/                # Login, refresh token, logout, logout-all
│   ├── session/              # List / revoke sessions (built on refresh_tokens)
│   ├── user/                 # User CRUD
│   ├── role/                 # Role CRUD (+ merges role_permission routes)
│   ├── permission/            # Permission CRUD
│   ├── role_permission/       # Role ↔ Permission relation (assign/list/revoke)
│   ├── user_role/             # User ↔ Role relation (assign/list/revoke)
│   ├── authorization/         # `/me` endpoint (profile & access rights)
│   ├── menus/                 # Hierarchical menu CRUD (+ merges menu_permission routes)
│   ├── menu_permissions/      # Menu ↔ Permission relation (assign/list/revoke)
│   ├── audit_log/             # Read-only query API over recorded audit entries
│   ├── system_settings/       # Global key-value settings (upsert/list/get/delete/toggle)
│   └── user_setting/          # Per-user key-value settings (same shape as system_settings)
│
├── infrastructure/        # Technical implementations (see status table below)
├── middleware/             # CORS, compression, timeout, request ID, tracing, JWT auth guard
├── helper/                 # Cross-domain helper functions (datetime, uuid, slug, string, ip, etc.)
├── common/                 # Shared types: response wrapper, error types, pagination, validator, extractors
└── routes/                 # Aggregates all domain routes under `/api/v1` + health check
```

**Request flow:** `routes` → `handler` → `service` (business logic) → `repository` (data access) → MySQL, with `dependency.rs` acting as the composition root that assembles every repository and service into `AppState`. **Every domain service is constructed with an injected `Arc<dyn AuditLogService>`**, so create/update/delete operations are audited automatically and consistently.

## 🧱 Database Schema

The migrations (`migrations/`) define the following tables, applied automatically in numeric order on startup:

| # | Table | Description |
|---|---|---|
| 001 | `users` | User data (soft delete via `deleted_at`) |
| 002 | `roles` | List of roles |
| 003 | `permissions` | List of permissions (`resource` + `action`) |
| 004 | `user_roles` | Many-to-many relation between users and roles |
| 005 | `role_permissions` | Many-to-many relation between roles and permissions |
| 006 | `refresh_tokens` | Per-device refresh tokens, backs both auth and session management |
| 009 | — | Alters `users` table (additional columns) |
| 010 | `audit_logs` | Actor, action, entity, status, IP, user agent, metadata, timestamp |
| 011 | `menus` | Hierarchical menu (supports `parent_id`) |
| 012 | `menu_permissions` | Relation between menus and permissions |
| 013 | `system_settings` | Global key-value settings |
| 014 | `user_settings` | Per-user key-value settings |

## 🚀 Tech Stack

| Category | Library | Status |
|---|---|---|
| Async runtime | `tokio` | ✅ in use |
| Web framework | `axum`, `tower`, `tower-http` | ✅ in use |
| Database | `sqlx` (MySQL, runtime-tokio-rustls) | ✅ in use |
| Auth | `jsonwebtoken`, `argon2` | ✅ in use |
| Validation | `validator` | ✅ in use |
| API documentation | `utoipa`, `utoipa-swagger-ui` | ✅ in use |
| Configuration | `config`, `dotenvy`, `envy` | ✅ in use |
| Logging | `tracing`, `tracing-subscriber`, `tracing-appender` | ✅ in use |
| Error handling | `thiserror`, `anyhow` | ✅ in use |
| Cache | `redis` | ⚠️ client code exists (`infrastructure/cache`), **not connected** in `dependency.rs` (commented out) |
| Object storage | local disk / S3 / MinIO (`infrastructure/storage`) | ⚠️ provider code exists, **not wired** into any domain yet |
| Others | `uuid`, `chrono`, `chrono-tz`, `reqwest`, `sha2`, `hex`, `once_cell`, `regex`, `async-trait` | ✅ in use |

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition, use the latest stable toolchain)
- MySQL 8.x
- (Optional) Redis — client code is present but not yet activated by the app

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

3. **Adjust the application config** (optional) in `configs/app.toml` — port, timeouts, CORS, JWT expiry, swagger toggle, etc.

4. **Prepare the MySQL database** matching the name in `.env` (`MYSQL_DATABASE`). Migrations run automatically on startup (see `bootstrap/migration.rs`).

5. **Run the application**
   ```bash
   cargo run
   ```
   Or use the scripts in `scripts/` (`build.sh`, `start.sh`, `test.sh`) as needed.

6. **Access the API documentation**
   Swagger UI is available at `http://localhost:8080/` (enable via `[swagger] enabled = true` in `configs/app.toml`).

## 📡 Main Endpoints

All endpoints are served under the `/api/v1` prefix. Everything except `/auth/*` requires a valid JWT (Bearer token).

**Auth** (`/api/v1/auth`) — public
- `POST /login`
- `POST /refresh`
- `POST /logout`
- `POST /logout-all`

**Sessions** (`/api/v1/sessions`)
- `GET /` — list active sessions (devices) for the current user
- `DELETE /{id}` — revoke a specific session
- `POST /revoke-others` — revoke all sessions except the current one

**Users** (`/api/v1/users`)
- `POST /`, `GET /`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`

**IAM** (`/api/v1/iam/...`)
- **Roles** (`/iam/roles`) — `POST /`, `GET /`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`
  - `POST /{role_id}/permissions` — assign permission to role
  - `GET /{role_id}/permissions` — list role's permissions
  - `DELETE /{role_id}/permissions/{permission_id}` — revoke permission from role
- **Permissions** (`/iam/permissions`) — `POST /`, `GET /`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`
- **Menus** (`/iam/menus`) — `POST /`, `GET /`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`
  - `POST /{menu_id}/permissions` — assign permission to menu
  - `GET /{menu_id}/permissions` — list menu's permissions
  - `DELETE /{menu_id}/permissions/{permission_id}` — revoke permission from menu
- **Authorization** (`/iam/authorization`) — `GET /me`
- **User ↔ Role** (`/iam/users`)
  - `POST /{user_id}/roles` — assign role to user
  - `GET /{user_id}/roles` — list user's roles
  - `DELETE /{user_id}/roles/{role_id}` — revoke role from user

**Audit Logs** (`/api/v1/audit-logs`) — read-only
- `GET /` — list/query audit log entries
- `GET /{id}` — get a single audit log entry

**System Settings** (`/api/v1/system-settings`)
- `PUT /` — upsert a setting, `GET /` — list, `GET /{key}` — get by key, `DELETE /{key}`, `PATCH /{key}/active` — toggle active

**User Settings** (`/api/v1/user-settings`)
- Same shape as System Settings, scoped to the current user

> ℹ️ Note: `role_permission` and `menu_permission` don't have their own top-level path — they're merged directly into `role`'s and `menus`' routers respectively, so their endpoints live under `/iam/roles/...` and `/iam/menus/...`.

## 🧩 Domains present but not yet wired into any HTTP route or app state

- **`infrastructure/cache`** (Redis) — client code exists, connection commented out in `dependency.rs`
- **`infrastructure/storage`** (local/S3/MinIO uploader) — implemented, but no domain currently calls it (no file upload feature yet)
- **`infrastructure/email`, `infrastructure/sms`, `infrastructure/notification`, `infrastructure/queue`, `infrastructure/oauth`, `infrastructure/websocket`** — empty placeholder files only, no implementation yet
- **`middleware/rate_limit.rs`, `middleware/role.rs`, `middleware/audit.rs`, `middleware/recovery.rs`** — empty placeholder files, not yet implemented or layered into the router
- **`infrastructure/external`** (payment, maps, OCR, WhatsApp clients) — implemented as generic HTTP client wrappers, not yet called by any domain

These are good next-step candidates depending on what the next application needs (file upload, notifications, rate limiting, panic recovery, etc.).

## 🗂️ Other Folders

- `configs/` — application configuration files (`app.toml`)
- `migrations/` — SQL migration files, numbered sequentially
- `scripts/` — helper scripts for build/start/test

## 🤝 Contributing

This project is under active development. Contributions, suggestions, and architecture discussions are welcome via Issues or Pull Requests.

## 📄 License

Not yet specified — feel free to add a license as needed (e.g. MIT or Apache-2.0).
