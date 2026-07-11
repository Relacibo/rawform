# @rawform/activepieces-piece

Activepieces piece with one webhook trigger for `rawform`.

## Environment variables (Activepieces instance)

- `RAWFORM_BASE_URL` (example: `https://rawform.example.com`)
- `RAWFORM_API_KEY` (client API key from rawform)

The piece has **no auth** in the UI and reads both values from env.

## Trigger behavior

- **test mode**
  - `onEnable`: `PATCH /api/v1/forms/:client/:test_external_id` with `is_active=true`
  - `onDisable`: `PATCH ...` with `is_active=false`
- **prod mode**
  - `onEnable`: `PUT /api/v1/forms/:client/:prod_external_id` (create/upsert)
  - `onDisable`: `DELETE /api/v1/forms/:client/:prod_external_id`

When a form already exists in prod mode, `PUT` overwrites it by `client + external_id`.

## URL helpers

On enable, the trigger stores helper URLs and includes them in emitted events under `_rawform`:

- `prod_form_url` (copyable URL)
- `test_form_url` (link target)
- `editor_url` (link target, based on `admin_token`)

## Development

```bash
npm install
npm run build
```
