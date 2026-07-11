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

# Create your first client and get its API key (shown once)
cargo run -- create-client myapp

# Start the server
cargo run
```

## API Overview

### Client-authenticated endpoints (`Authorization: Bearer <api_key>`)

| Method | Path | Description |
|--------|------|-------------|
| `PUT` | `/api/v1/forms/:client_name/:external_id` | Create or replace a form (new definition auto-created) |
| `GET` | `/api/v1/forms/:client_name/:external_id` | Get form with current definition |
| `PATCH` | `/api/v1/forms/:client_name/:external_id` | Partial update (data, webhook_url, is_active) |
| `DELETE` | `/api/v1/forms/:client_name/:external_id` | Delete the form instance |
| `DELETE` | `/api/v1/definitions/:client_name/:definition_id` | Delete a form definition (fails if still in use) |

### Admin token endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/admin/forms/:admin_token` | Get form with current definition |
| `PUT` | `/api/v1/admin/forms/:admin_token` | Replace form data (new definition auto-created) |
| `PATCH` | `/api/v1/admin/forms/:admin_token` | Partial update |

### Public submit endpoints (no auth)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/submit/:client_name/:external_id/token` | Resolve submit token + schema by readable IDs |
| `GET` | `/api/v1/submit/:submit_token` | Get form schema by submit token |
| `POST` | `/api/v1/submit/:submit_token` | Submit form values |

## Typical Integration Flow

```
1. Client creates/updates a form:
   PUT /api/v1/forms/myapp/contact-form
   Authorization: Bearer rawform_...
   { "data": { "title": "Contact", "elements": [...] }, "webhook_url": "https://..." }

2. Frontend resolves the form (no auth needed):
   GET /api/v1/submit/myapp/contact-form/token
   → { "submit_token": "...", "data": { ... } }

3. Frontend renders and submits:
   POST /api/v1/submit/<submit_token>
   { "values": { "name": "Alice", "message": "Hello" } }
```

## Security Notes

- Admin and submit tokens are passed as URL path segments and may appear in server access logs. Consider log sanitization in production.
- `submit_token` is a stable UUID. The public `/token` endpoint intentionally exposes it — rate-limit this endpoint in production to prevent enumeration.

## License

MIT — see [LICENSE](./LICENSE).
