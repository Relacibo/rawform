# rawform — Concept & TODO

> This project was largely vibecoded. The following is an evolving concept and task list.

## Concept

rawform is a minimalist, self-hostable form builder. **Clients** (e.g. Activepieces, n8n, custom scripts) authenticate via API key and can create/manage **forms**. Each form has:

- A **definition** (`data` JSON): ordered list of typed elements
- An **admin_token**: for full read+write access to the form definition without client credentials
- A **submit_token**: for form renderers/users to submit responses
- An optional **webhook_url**: called on each submission

### Auth model

| Who | How | What |
|-----|-----|-------|
| Client app | `Bearer <api_key>` header | Create/read/update their own forms |
| Form admin | `admin_token` in URL path | Read/update a single form |
| End user | `submit_token` in URL path | Submit a response to a form |

> **Security note:** tokens in URL paths appear in server logs. Document this clearly and optionally support header-based token auth in the future.

### Route structure

```
PUT    /api/v1/forms/:client_name/:external_id   → create or replace form (client auth)
GET    /api/v1/forms/:client_name/:external_id   → get form definition (client auth)
PATCH  /api/v1/forms/:client_name/:external_id   → partial update (client auth)

GET    /api/v1/admin/forms/:admin_token           → get form (admin token)
PUT    /api/v1/admin/forms/:admin_token           → replace form data (admin token)
PATCH  /api/v1/admin/forms/:admin_token           → partial update (admin token)

POST   /api/v1/submit/:submit_token               → submit form response
```

### Form element types

Each element has at minimum: `type`, `label`, `name` (slugified from label, overridable), `required`.

| Type | Extra fields |
|------|-------------|
| `text` | `placeholder` |
| `textarea` | `placeholder` |
| `dropdown` | `options: [{label, value}]` — value derived from label, overridable |
| `checkbox` | — |

---

## TODO

### Backend

- [ ] **DB: RETURNING on upsert** — SQLite <3.35 doesn't support RETURNING; add runtime check or use separate SELECT after INSERT
- [ ] **Token preservation on upsert** — PUT `/forms/:client/:external_id` currently overwrites `admin_token`/`submit_token` on conflict; fix to keep existing tokens
- [ ] **PATCH endpoint implementation** — `api/forms.rs` and `api/admin.rs` have TODO stubs; implement partial updates (data, webhook_url, is_active)
- [ ] **PATCH: merge strategy** — decide: replace `data` field entirely or deep-merge elements array
- [ ] **Webhook_url update** — `admin::put_form` currently ignores `webhook_url` from body
- [ ] **Submissions table** — design and create migration: `(id, form_id, values JSON, submitted_at)`
- [ ] **Webhook firing** — on submit, POST the submission payload to `form.webhook_url` if set; add retry logic / fire-and-forget
- [ ] **Submit validation** — validate submitted field names and types against form definition
- [ ] **Client management API** — endpoints to create/list/rotate-key/deactivate clients (probably under `/api/v1/admin/clients/`, protected by a master API key or env var)
- [ ] **Master API key** — a server-level secret in `.env` to protect client management endpoints
- [ ] **is_active on update** — `forms::update_data` doesn't touch `is_active`; add `set_active` db function
- [ ] **sqlx migrations** — consider switching from manual SQL execution to `sqlx migrate` with `.sqlx/` folder for better migration tracking
- [ ] **PostgreSQL support** — migrations use SQLite syntax (`datetime('now')`); create Postgres variants or use `sqlx migrate` with database-agnostic syntax
- [ ] **Error on duplicate client name** — return 409 conflict with helpful message instead of DB error
- [ ] **API key generation helper** — CLI command or endpoint to generate a new client + API key
- [ ] **Rate limiting** — add per-IP or per-token rate limiting on submit endpoint
- [ ] **CORS configuration** — currently permissive; lock down in production

### Frontend

- [ ] **Save/load from API** — connect builder to backend; add auth input fields (client name + API key)
- [ ] **Load existing form** — fetch form by admin_token and populate builder state
- [ ] **Preview mode** — render the form as it would appear to an end user (in a modal or side panel)
- [ ] **More element types** — number, date, email, radio group, file upload
- [ ] **Validation rules** — per-element min/max, pattern, custom error messages
- [ ] **Conditional logic** — show/hide elements based on other field values (stretch goal)
- [ ] **Drag-and-drop reorder** — replace up/down buttons with drag handles
- [ ] **Form title / description field**
- [ ] **Responsive design** — test and improve on mobile
- [ ] **Accessible markup** — add proper ARIA labels, keyboard navigation

### Ops / Infra

- [ ] **Docker / docker-compose** — add Dockerfile and compose for easy self-hosting
- [ ] **Health check endpoint** — `GET /health`
- [ ] **Structured logging** — add request IDs, log submission events
- [ ] **Config validation** — fail fast on startup if required env vars are missing or malformed
- [ ] **CI** — GitHub Actions: `cargo check`, `cargo test`, `cargo clippy`
- [ ] **Tests** — integration tests for each API endpoint using `axum::test`

### Documentation

- [ ] **Standalone definition endpoint** — `POST /api/v1/definitions/:client_name` creates a definition independently and returns `definition_id`. `PUT /forms/:client/:external_id` should accept either `data` (auto-creates definition) OR `definition_id` (uses existing). `PATCH /forms/:client/:external_id` should accept `definition_id` to reassign.
- [ ] **Projects table** — `projects(id, client_id, name, admin_token, created_at)`. `form_definitions` gets `project_id` (nullable FK) and `tags` (JSON array, e.g. `["prod", "draft"]`). Endpoints: `POST/GET /api/v1/projects/:client_name`, `GET /api/v1/projects/:project_admin_token/definitions?tag=prod`. Enables listing all prod definitions of a project across forms.
- [x] **Activepieces integration example** — npm-ready piece scaffold in `integrations/activepieces` with one webhook trigger and test/prod lifecycle behavior
- [ ] **Admin UI** — consider a simple admin page listing all forms for a client (accessible by api key)
