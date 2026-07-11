# rawform

> ⚠️ **Note:** This project was largely implemented with the help of AI. Expect rough edges, missing validations, and evolving APIs.

A minimalist, self-hostable form builder with a REST API. Forms are created and updated directly; there are no standalone definition endpoints.

## Features

- Create and manage forms via REST API
- Simple schema: one `forms` table with JSON `data`
- Admin access via per-form `admin_token`
- Submission via per-form `submit_token`
- Public endpoint to resolve `submit_token` + schema by `client_name + external_id`
- Optional webhook call on submit
- Minimal HTML/JS/CSS frontend (`/builder.html`, `/form.html`)

## Concepts

- **Clients**: applications (e.g. Activepieces) authenticated by API key
- **Forms**: entities with stable `client_name + external_id`, JSON `data`, `admin_token`, `submit_token`

## Quick Start

```bash
cp .env.example .env
# edit DATABASE_URL, HOST, PORT as needed

# create client (prints API key once)
cargo run -- client myapp

# create or upsert form
cargo run -- form myapp contact-form --api-key rawform_... --data '{"title":"Contact","elements":[]}'

# start server
cargo run
```

## CLI

```text
rawform [--database-url <URL>] [--host <HOST>] [--port <PORT>] [COMMAND]

Commands:
  client <name>
    Create a new API client. Prints API key once.

  form <client_name> <external_id> --api-key <key> [--data <json>] [--webhook-url <url>]
    Create/upsert a form instance. Prints admin_token, submit_token and URLs.

  forms list [--client <name>] [--client-id <id>] [--name <external_id>]
    List forms and optionally filter by client name, client id, or external id.
```

## API Overview

### Client-authenticated (`Authorization: Bearer <api_key>`)

| Method | Path | Description |
|---|---|---|
| `PUT` | `/api/v1/forms/:client_name/:external_id` | Create or update form (`data`, optional `webhook_url`) |
| `GET` | `/api/v1/forms/:client_name/:external_id` | Get form |
| `PATCH` | `/api/v1/forms/:client_name/:external_id` | Partial update (`data`, `webhook_url`, `is_active`) |
| `DELETE` | `/api/v1/forms/:client_name/:external_id` | Delete form |

### Admin token

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/admin/forms/:admin_token` | Get form |
| `PUT` | `/api/v1/admin/forms/:admin_token` | Replace form data |
| `PATCH` | `/api/v1/admin/forms/:admin_token` | Partial update |

### Public submit

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/submit/:client_name/:external_id/token` | Resolve submit token + schema |
| `GET` | `/api/v1/submit/:submit_token` | Get schema by submit token |
| `POST` | `/api/v1/submit/:submit_token` | Submit values |

## Security Notes

- `admin_token` and `submit_token` are path tokens; they can appear in access logs.
- Public `/submit/:client/:external_id/token` should be rate-limited in production.
- `builder.html` requires `?token=<admin_token>`.
- `form.html` can load by `?token=<submit_token>` or `?client=<name>&id=<external_id>`.

## License

MIT — see [LICENSE](./LICENSE).
