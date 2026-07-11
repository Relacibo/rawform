# rawform

> ⚠️ **Note:** This project was largely implemented with the help of AI. Expect rough edges, missing validations, and evolving APIs. Contributions welcome.

A minimalist, self-hostable form builder with a REST API. Designed to integrate with automation tools like [Activepieces](https://www.activepieces.com/).

## Features

- Create and manage forms via REST API
- Versioned form definitions — updating a form creates a new definition; the live instance switches to it automatically
- Submit form responses via a stable per-form submit token
- Public endpoint to resolve `submit_token` + schema by `client_name` + `external_id` (no auth required)
- Admin access to a form via a per-form admin token
- On-submit webhook (fire-and-forget POST to a configurable URL)
- Minimal HTML/JS/CSS form builder UI
- SQLite (dev) or PostgreSQL (prod) backend

## Concepts

**Clients** are applications (e.g. an Activepieces flow) that authenticate via API key to create and manage forms.

**Form definitions** contain the JSON schema of a form (fields, types, labels). Every write that changes `data` creates a new definition record.

**Forms** (instances) are the live, addressable endpoints. Each form has a stable `client_name + external_id` pair, an `admin_token`, a `submit_token`, and a pointer to the current definition. Updating a form's data auto-publishes the new definition.

## Quick Start

```bash
cp .env.example .env
# edit DATABASE_URL, HOST, PORT as needed

# Create a client and get its API key (shown once)
cargo run -- client myapp

# Create a form (empty schema by default)
cargo run -- form myapp contact-form --api-key rawform_...

# Or: create a standalone definition first, then assign it
cargo run -- definition myapp --api-key rawform_... --data '{"title":"Contact","elements":[]}'
cargo run -- form myapp contact-form --api-key rawform_... --definition-id 1

# Start the server
cargo run
```

## CLI Reference

```
rawform [--database-url <URL>] [--host <HOST>] [--port <PORT>] [COMMAND]

Commands:
  client <name>
    Create a new API client. Prints API key once.

  definition <client_name> --api-key <key> [--data <json>]
    Create a standalone form definition. Returns definition_id.

  form <client_name> <external_id> --api-key <key>
       [--data <json> | --definition-id <id>] [--webhook-url <url>]
    Create a form instance. Prints admin_token, submit_token and URLs.
```

## API Overview

### Client-authenticated endpoints (`Authorization: Bearer <api_key>`)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/definitions/:client_name` | Create a standalone definition, returns `definition_id` |
| `DELETE` | `/api/v1/definitions/:client_name/:definition_id` | Delete a definition (fails if still assigned to a form) |
| `PUT` | `/api/v1/forms/:client_name/:external_id` | Create or update a form — body: `{data}` or `{definition_id}` |
| `GET` | `/api/v1/forms/:client_name/:external_id` | Get form with current definition |
| `PATCH` | `/api/v1/forms/:client_name/:external_id` | Partial update: `data`, `definition_id`, `webhook_url`, `is_active` |
| `DELETE` | `/api/v1/forms/:client_name/:external_id` | Delete the form instance |

### Admin token endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/admin/forms/:admin_token` | Get form with current definition |
| `PUT` | `/api/v1/admin/forms/:admin_token` | Replace form data (creates new definition) |
| `PATCH` | `/api/v1/admin/forms/:admin_token` | Partial update: `data`, `webhook_url`, `is_active` |

### Public submit endpoints (no auth)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/submit/:client_name/:external_id/token` | Resolve submit token + schema by readable IDs |
| `GET` | `/api/v1/submit/:submit_token` | Get form schema by submit token |
| `POST` | `/api/v1/submit/:submit_token` | Submit form values |

## Typical Integration Flow

```
# 1. Create a definition
POST /api/v1/definitions/myapp
Authorization: Bearer rawform_...
{ "data": { "title": "Contact", "elements": [...] } }
→ { "id": 5, ... }

# 2. Create/update a form pointing to that definition
PUT /api/v1/forms/myapp/contact-form
{ "definition_id": 5, "webhook_url": "https://..." }
→ { "admin_token": "...", "submit_token": "...", ... }

# Publish: reassign production form to a new definition
PATCH /api/v1/forms/myapp/contact-form
{ "definition_id": 7 }

# 3. Frontend resolves the form (no auth needed)
GET /api/v1/submit/myapp/contact-form/token
→ { "submit_token": "...", "data": { ... } }

# 4. User submits
POST /api/v1/submit/<submit_token>
{ "values": { "name": "Alice", "message": "Hello" } }
```

## Security Notes

- Admin and submit tokens are passed as URL path segments and may appear in server access logs. Consider log sanitization in production.
- `submit_token` is a stable UUID. The public `/token` endpoint intentionally exposes it — rate-limit this endpoint in production to prevent enumeration.

## License

MIT — see [LICENSE](./LICENSE).
