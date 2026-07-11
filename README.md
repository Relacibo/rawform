# rawform

> ⚠️ **Note:** This project was largely implemented with the help of AI. Expect rough edges, missing validations, and evolving APIs. Contributions welcome.

A minimalist, self-hostable form builder with a REST API. Designed to integrate with automation tools like [Activepieces](https://www.activepieces.com/).

## Features

- Create and manage forms via REST API
- Submit form responses via a per-form submit token
- Admin access to forms via a per-form admin token
- Minimal HTML/JS/CSS form builder UI
- SQLite (dev) or PostgreSQL (prod) backend

## Security Note

Admin and submit tokens are passed as URL path segments, which means they may appear in server access logs. Consider log sanitization or switching to header-based auth in production.

## Quick Start

```bash
cp .env.example .env
# edit .env as needed
cargo run
```

## API Overview

### Client-authenticated endpoints (Bearer API key)

| Method | Path | Description |
|--------|------|-------------|
| PUT | `/api/v1/forms/:client_name/:external_id` | Create or replace a form |
| GET | `/api/v1/forms/:client_name/:external_id` | Get form definition |
| PATCH | `/api/v1/forms/:client_name/:external_id` | Update form fields |

### Admin token endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/admin/forms/:admin_token` | Get form definition |
| PUT | `/api/v1/admin/forms/:admin_token` | Replace form definition |
| PATCH | `/api/v1/admin/forms/:admin_token` | Update form fields |

### Submit endpoint

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/submit/:submit_token` | Submit a form response |

## License

MIT — see [LICENSE](./LICENSE).
