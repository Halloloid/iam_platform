# IAM Platform Design

## 1. Purpose

`iam_platform` is a reusable Authentication and Authorization service for
applications that do not want to implement identity, organization management,
and role-based access control from scratch.

The platform provides:

- User registration and login.
- JWT access tokens and refresh-token rotation.
- Logout and device-based session management.
- Organizations and organization membership.
- Organization-scoped roles and permissions.
- Scoped API keys for service-to-service access.
- User and organization audit logs.

The service is implemented as an Axum HTTP API backed by PostgreSQL. SQLx
migrations create and evolve the database schema, while integration tests
exercise the public API.

## 2. Architectural layers

The platform is organized into three functional authorization layers:

### Layer 1: Authentication

Authentication identifies the caller and manages identity credentials:

- User registration.
- Password hashing with bcrypt.
- Login and JWT access-token creation.
- Refresh-token rotation.
- Logout.
- Device-based session listing and revocation.
- API-key validation.

### Layer 2: Organizations

Organizations isolate tenants or companies within the same platform:

- Create and list organizations.
- View and update an organization.
- Add and remove members.
- Inspect organization membership.
- Assign and remove member roles.

### Layer 3: RBAC

Role-based access control determines what an authenticated actor may do:

- Global permissions are seeded in the database.
- Organizations own their roles.
- Roles receive permissions.
- Organization members receive roles.
- User requests are checked against membership and role permissions.
- API-key requests are checked against the key's organization and scopes.

## 3. Request architecture

Requests follow this path:

```text
HTTP request
    |
    v
Route composition
    |
    v
Authentication middleware
    |
    v
Handler: extraction and validation
    |
    v
Service: business rules and workflows
    |
    v
Repository: parameterized SQLx queries
    |
    v
PostgreSQL
```

Handlers should remain thin. HTTP extraction, request validation, and response
conversion belong in handlers. Business rules belong in services. SQL and
persistence operations belong in repositories.

## 4. Runtime components

- **Axum** provides routing, extractors, middleware, and HTTP responses.
- **Tokio** provides the asynchronous runtime.
- **SQLx/PostgreSQL** provide persistence and migrations.
- **JWT** access tokens authenticate users after login.
- **bcrypt** hashes and verifies passwords.
- **Tracing** provides request and application logging.
- **Tower HTTP** provides HTTP tracing middleware.

The application starts by loading environment variables, connecting to
PostgreSQL, applying migrations, creating the router, and starting the HTTP
server.

## 5. Authentication design

### User authentication

1. A client registers with an email, password, and name.
2. The password is hashed with bcrypt before storage.
3. On login, the password is verified.
4. A short-lived JWT access token is returned.
5. A random refresh token is stored with the user's session.
6. Refresh requests rotate the refresh token and issue a new access token.

User access tokens contain the user's UUID in the `sub` claim. Clients send
them using:

```text
Authorization: Bearer <jwt>
```

### API-key authentication

API keys use the `iam_` prefix. The middleware distinguishes API keys from JWTs,
validates the key against PostgreSQL, and attaches an `AuthContext::ApiKey`.
Raw API keys are returned only at creation time and must not be logged.

### Authentication context

The middleware inserts one of these contexts into the request extensions:

- `AuthContext::User`: verified JWT claims and user UUID.
- `AuthContext::ApiKey`: API-key ID, organization ID, and permission scopes.

`require_user` rejects API keys for endpoints that require a user identity.
`resolver_actor` validates organization ownership, membership permissions, and
API-key organization/scope permissions.

## 6. Session design

Sessions are associated with:

- User ID.
- Device string derived from the `User-Agent` header.
- IP address.
- Refresh token.
- Creation and expiration timestamps.
- Revocation state.

Repeated login from the same device updates the existing active session rather
than creating a duplicate. Refresh tokens are rotated during refresh.

Revoked and unexpired sessions remain visible in session history. Session
listing determines the current session from the latest session for the
requesting device, even when that latest session is revoked. This preserves
visibility after revoking the current session.

## 7. Authorization design

For organization-scoped requests:

1. The authentication middleware identifies the actor.
2. The handler obtains the organization ID from the route.
3. `resolver_actor` verifies the actor's organization access.
4. If a permission is required:
   - Users are checked through organization membership and assigned roles.
   - API keys are checked against their stored permission scopes.
5. The service performs the business operation.

A user or API key belonging to another organization must receive a forbidden
response rather than accessing the resource.

## 8. API surface

### Public routes

| Method | Path | Purpose |
|---|---|---|
| GET | `/health` | Health check |
| POST | `/auth/register` | Register a user |
| POST | `/auth/login` | Authenticate a user |
| POST | `/auth/refresh` | Rotate refresh token and issue access token |
| POST | `/auth/logout` | Revoke a refresh-token session |

### User and session routes

| Method | Path | Purpose |
|---|---|---|
| GET | `/user/me` | View the authenticated user's profile |
| PATCH | `/user/me` | Update the authenticated user's profile |
| GET | `/session` | List the user's active and revoked unexpired sessions |
| DELETE | `/session/{session_id}` | Revoke an owned session |

### Organization routes

| Method | Path | Purpose |
|---|---|---|
| POST | `/organization` | Create an organization |
| GET | `/organization` | List organizations available to the user |
| GET | `/organization/{id}` | View an organization |
| PATCH | `/organization/{id}` | Update an organization |

### Role and permission routes

| Method | Path | Purpose |
|---|---|---|
| POST | `/organization/{id}/role` | Create an organization role |
| GET | `/organization/{id}/role` | List organization roles |
| PATCH | `/organization/{id}/role/{roleid}` | Rename a role |
| DELETE | `/organization/{id}/role/{roleid}` | Delete a role |
| GET | `/permission` | List available permissions |
| POST | `/organization/{id}/role/{roleid}/permission` | Assign a permission |
| GET | `/organization/{id}/role/{roleid}/permission` | List role permissions |
| DELETE | `/organization/{id}/role/{roleid}/permission` | Remove a permission |

### Membership routes

| Method | Path | Purpose |
|---|---|---|
| POST | `/organization/{org_id}/member` | Add a member |
| GET | `/organization/{org_id}/member` | List members |
| DELETE | `/organization/{org_id}/member/{member_id}` | Remove a member |
| GET | `/organization/{org_id}/member/{member_id}/role` | List member roles |
| POST | `/organization/{org_id}/member/{member_id}/role` | Assign a role |
| DELETE | `/organization/{org_id}/member/{member_id}/role/{role_id}` | Remove a role |

### API-key routes

| Method | Path | Purpose |
|---|---|---|
| POST | `/organization/{org_id}/api_key` | Create a scoped API key |
| GET | `/organization/{org_id}/api_key` | List organization API keys |
| DELETE | `/organization/{org_id}/api_key/{api_id}` | Revoke an API key |

### Audit-log routes

| Method | Path | Purpose |
|---|---|---|
| GET | `/audit-logs` | List logs for the authenticated user |
| GET | `/organization/{org_id}/audit-logs` | List logs for an organization |

## 9. Data model

The primary database relationships are:

- `users` stores identities and password hashes.
- `organizations` stores tenant/company records.
- `membership` links users to organizations.
- `roles` belongs to an organization.
- `permissions` stores available permissions.
- `role_permissions` links roles to permissions.
- `member_roles` links organization members to roles.
- `sessions` stores device sessions and refresh tokens.
- `api_keys` stores hashed organization API keys.
- `api_keys_scopes` links API keys to permission scopes.
- `audit_logs` stores actor, action, resource, and timestamp information.

Foreign keys and composite keys enforce ownership and membership relationships.
New schema changes must be introduced as new ordered migrations.

### Indexes

- `organizations.created_at DESC` — supports cursor pagination on org listings
- `membership.user_id` — supports membership lookups per user
- `audit_logs.resource` — supports LIKE prefix queries for org-scoped logs
- `audit_logs.actor_id` — supports personal audit log queries
- `audit_logs.timestamp DESC` — supports cursor pagination on audit logs
- `sessions.user_id` — supports session listing per user
- `api_keys.key_hash` — supports O(1) key validation on every authenticated request

## 10. Error handling

The shared `AppError` type maps application failures to HTTP responses:

- `400 Bad Request`: malformed or invalid input.
- `401 Unauthorized`: missing or invalid authentication.
- `403 Forbidden`: authenticated actor lacks access.
- `404 Not Found`: resource does not exist or is not owned by the actor.
- `409 Conflict`: duplicate or conflicting resource.
- `500 Internal Server Error`: database and unexpected server failures.

New endpoints should reuse this mapping instead of defining custom error
formats.

## 11. Project structure

```text
iam_platform/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── DESIGN.md
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

The current repository contains migrations through
`20260805091002_3rd-new-secondary-index.sql` and integration tests for
authentication, organizations, roles, memberships, API keys, sessions, and
audit logs.

## 12. Database ER diagram

<img width="1000" alt="IAM platform database entity relationship diagram" src="https://github.com/user-attachments/assets/fba6d491-2ddc-44b2-9c3e-001b00648cc4" />

## 13. Design decisions

### Stateless JWT validation
Access tokens are validated by signature and expiry only — no database
lookup per request. Session revocation is enforced at refresh time: a
revoked session cannot obtain a new access token. This trades immediate
revocation for eliminating per-request database hits, which is the correct
tradeoff at this scale.

### Device-based session deduplication
Repeated login from the same device updates the existing session rather
than creating a new one. This prevents session table bloat from repeated
logins and keeps the session list meaningful to the user.

### Bootstrap transaction
Organization creation runs five operations in a single database
transaction: insert organization, insert membership, insert Owner role,
assign all system permissions to Owner, assign Owner role to creator.
Either all five succeed or none do — no partial organization state is
possible.

### Name-based Owner role protection
The Owner role is protected by name rather than a flag column. The
UNIQUE(name, org_id) constraint prevents a second "owner" role from being
created in the same organization. POST /role explicitly rejects the name
"owner". This avoids an extra migration while achieving the same guarantee.

### Soft deletes
Users, organizations, and API keys use is_deleted rather than hard
deletion. This preserves audit log integrity — audit records reference
actor IDs that must remain resolvable even after a resource is logically
removed.

### Cursor-based pagination
Organization and audit log listings use cursor-based pagination over
offset. Cursor pagination avoids the skipped-record problem under
concurrent inserts and eliminates full-table scans at large offsets. The
cursor encodes created_at as an opaque base64 string so clients treat it
as a token rather than a manipulable timestamp.

### Global permissions, org-scoped roles
Permissions are system-wide and seeded at migration time. Roles are
organization-owned and can be customized per org. This means the
permission vocabulary is stable and predictable while orgs retain full
control over how permissions are grouped into roles.

### Dual authentication context
The middleware produces either AuthContext::User or AuthContext::ApiKey.
User-only endpoints reject API keys explicitly via require_user. 
Org-scoped endpoints accept both via resolve_actor, which enforces RBAC
for users and scope checks for API keys through a single code path.


## 14. Tradeoffs

### Stateless tokens vs immediate revocation
Stateless JWT validation means a revoked session's access token remains
valid until expiry (up to 15 minutes). A Redis blocklist would enable
immediate revocation without per-request database hits. The current design
accepts the delay in exchange for simplicity.

### Name-based role protection vs flag column
Using the Owner name as the protection mechanism means protection depends
on application-level guards rather than a database constraint. An is_system
flag enforced at the DB level would be stronger. The name approach was
chosen because the UNIQUE constraint already prevents duplicate Owner roles
and avoids an additional migration.

### Single database vs caching layer
All permission checks query PostgreSQL directly. At higher request volumes,
a Redis cache for permission lookups would reduce database load
significantly. The current design is correct for this scale and the cache
layer could be added without changing the authorization model.

### Fetch-all vs pagination for small collections
Roles, sessions, and permissions use fetch-all rather than cursor
pagination. These collections are inherently bounded per organization or
user. Paginating them would add complexity with no practical benefit at
realistic data volumes.


