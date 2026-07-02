# Folder Structure
``` sh
iam_platform
├── Cargo.lock
├── Cargo.toml
├── DESIGN.md
├── README.md
├── migrations
│   ├── 20260628091845_init-tables.sql
│   ├── 20260630035657_alter_org.sql
│   ├── 20260701144117_alter_sessions.sql
│   ├── 20260702135709_secondary-indexs.sql
│   └── 20260702143126_seed_permissions.sql
└── src
    ├── config
    │   ├── auth_config.rs
    │   ├── db_config.rs
    │   ├── response_config.rs
    │   └── server_config.rs
    ├── config.rs
    ├── handlers
    │   ├── api_key.rs
    │   ├── health.rs
    │   ├── organization.rs
    │   ├── role.rs
    │   ├── session.rs
    │   └── user.rs
    ├── handlers.rs
    ├── lib.rs
    ├── main.rs
    ├── middleware
    │   └── auth_middleware.rs
    ├── middleware.rs
    ├── models
    │   ├── api_key.rs
    │   ├── api_key_scope.rs
    │   ├── audit_logs.rs
    │   ├── member_role.rs
    │   ├── membership.rs
    │   ├── organization.rs
    │   ├── permission.rs
    │   ├── role.rs
    │   ├── role_permission.rs
    │   ├── session.rs
    │   └── user.rs
    ├── models.rs
    ├── repositories
    │   ├── api_key.rs
    │   ├── organization.rs
    │   ├── session.rs
    │   └── user.rs
    ├── repositories.rs
    ├── routes
    │   ├── main_router.rs
    │   ├── organization_router.rs
    │   └── user_router.rs
    ├── routes.rs
    ├── services
    │   └── user.rs
    └── services.rs

10 directories, 48 files
```

# This is the Database ER Diagram 
<img width="1000" src="https://github.com/user-attachments/assets/fba6d491-2ddc-44b2-9c3e-001b00648cc4" />
