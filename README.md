# IAM Platform

`iam_platform` is a reusable Authentication and Authorization service for
applications that need user identity, organizations, and role-based access
control without implementing those systems from scratch.

The service is an asynchronous Axum HTTP API backed by PostgreSQL. It uses
SQLx migrations for the database schema and integration tests for API behavior.

## Features

- User registration and login.
- Bcrypt password hashing.
- JWT access tokens.
- Refresh-token rotation.
- Logout and device-based session management.
- Organizations and organization membership.
- Organization-scoped roles and permissions.
- Scoped API keys for service-to-service access.
- User and organization audit logs.
- Permission and ownership checks for protected resources.

## Architecture

The platform has three functional layers:

1. **Authentication**: registration, login, JWTs, refresh tokens, logout,
   sessions, and API-key validation.
2. **Organizations**: organization creation, membership, and member roles.
3. **RBAC**: organization roles, permissions, role assignments, and access
   checks.

Requests follow this flow:

```text
route -> authentication middleware -> handler -> service -> repository -> PostgreSQL
```

See [`DESIGN.md`](DESIGN.md) for the detailed architecture, authorization
model, API surface, data model, and database diagram.

## Technology

- Rust edition 2024
- Axum 0.8
- Tokio
- SQLx 0.9 with PostgreSQL
- JSON Web Tokens
- bcrypt
- validator
- tracing and tower-http

## API overview

### Public endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/health` | Health check |
| POST | `/auth/register` | Register a user |
| POST | `/auth/login` | Authenticate a user |
| POST | `/auth/refresh` | Rotate the refresh token and issue an access token |
| POST | `/auth/logout` | Revoke a refresh-token session |

### User and session endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/user/me` | View the authenticated user's profile |
| PATCH | `/user/me` | Update the authenticated user's profile |
| GET | `/session` | List active and revoked unexpired sessions |
| DELETE | `/session/{session_id}` | Revoke an owned session |

### Organization, RBAC, and membership endpoints

| Resource | Endpoints |
|---|---|
| Organizations | `GET/POST /organization`, `GET/PATCH /organization/{id}` |
| Roles | `GET/POST /organization/{id}/role`, `PATCH/DELETE /organization/{id}/role/{roleid}` |
| Permissions | `GET /permission`, plus role-permission assignment endpoints |
| Members | Member listing, creation, removal, and role assignment under `/organization/{org_id}/member` |

### API keys and audit logs

| Resource | Endpoints |
|---|---|
| API keys | `GET/POST /organization/{org_id}/api_key`, `DELETE /organization/{org_id}/api_key/{api_id}` |
| Audit logs | `GET /audit-logs`, `GET /organization/{org_id}/audit-logs` |

All endpoints except the public endpoints require authentication. Users
authenticate with `Authorization: Bearer <jwt>`. API keys use the `iam_`
prefix and are restricted to their organization and configured permission
scopes.

## Requirements

- Rust toolchain with Cargo.
- PostgreSQL 16 or compatible PostgreSQL.
- Docker Compose is optional but recommended for local PostgreSQL.

## Configuration

Create a `.env` file based on `.env.example`:

```text
DATABASE_URL=postgres://...
JWT_SECRET=...
```

Never commit real credentials or production secrets.

## Running locally

Start PostgreSQL with Docker:

```powershell
docker compose up -d db
```

Run the application:

```powershell
cargo run
```

The application listens on port `3000`. When running the complete Docker
stack:

```powershell
docker compose up -d
```

The application connects to the PostgreSQL service named `db` inside Compose.
When running locally outside Docker, PostgreSQL is normally available on port
`5432`.

On startup, the application loads configuration, connects to PostgreSQL, runs
all pending SQLx migrations, and starts the HTTP server.

## Testing and validation

Run formatting and compilation checks:

```powershell
cargo fmt --check
cargo check
```

Run a specific integration test:

```powershell
cargo test --test session_test
```

Run the complete test suite:

```powershell
cargo test
```

Current integration test areas include authentication, organizations, roles,
memberships, API keys, sessions, and audit logs. The integration tests use
`sqlx::test`, so PostgreSQL and the required environment configuration must be
available.

For a failing test with a backtrace:

```powershell
$env:RUST_BACKTRACE = "1"
cargo test --test session_test test_name -- --nocapture
```

## Project structure

```text
iam_platform/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── DESIGN.md
├── AGENTS.md
├── Dockerfile
├── docker-compose.yml
├── migrations/
├── src/
│   ├── config/
│   ├── handlers/
│   ├── middleware/
│   ├── models/
│   ├── repositories/
│   ├── routes/
│   └── services/
└── tests/
    └── common/
```

The current migration set extends through
`20260805091002_3rd-new-secondary-index.sql`.

## Security notes

- Passwords are stored as bcrypt hashes.
- API keys are stored as hashes; the raw key is only returned at creation.
- Do not log passwords, JWTs, refresh tokens, raw API keys, or other secrets.
- Protected resources enforce authentication, organization ownership, and
  permission checks.
- API keys cannot access user-only endpoints such as profile and session
  management.

## Database ER diagram

<img width="1000" alt="IAM platform database entity relationship diagram" src="https://github.com/user-attachments/assets/fba6d491-2ddc-44b2-9c3e-001b00648cc4" />
