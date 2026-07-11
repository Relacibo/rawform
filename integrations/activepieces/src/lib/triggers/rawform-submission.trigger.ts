import {
  createTrigger,
  PieceAuth,
  Property,
  TriggerStrategy,
  DropdownOption,
} from '@activepieces/pieces-framework';
import {
  buildEditorUrl,
  buildFormUrl,
  deleteForm,
  getForm,
  getRawformConfig,
  patchForm,
  putForm,
  resolveManagedExternalId,
} from '../rawform-client';

type Mode = 'test' | 'prod';

type TriggerProps = {
  mode: Mode;
  client: string;
  prod_external_id: string;
  test_external_id: string;
  form_schema_json: string;
  show_urls: string;
};

type StoredLinks = {
  prod_form_url: string;
  test_form_url: string;
  editor_url: string | null;
  managed_external_id: string;
  managed_mode: Mode;
};

function isNotFoundError(error: unknown): boolean {
  if (!(error instanceof Error)) {
    return false;
  }
  const msg = error.message.toLowerCase();
  return msg.includes('404') || msg.includes('not found');
}

const modeOptions: DropdownOption<string>[] = [
  { label: 'Test mode (enable/disable)', value: 'test' },
  { label: 'Production mode (create/delete)', value: 'prod' },
];

const setupMarkdown = `
The piece is configured via environment variables on your Activepieces instance:

- \`RAWFORM_BASE_URL\`
- \`RAWFORM_API_KEY\`

**Behavior**
- **test** mode: onEnable activates an existing test form, onDisable deactivates it.
- **prod** mode: onEnable creates/upserts a production form, onDisable deletes it.

**Production Form URL (copyable)**
\`\`\`text
<RAWFORM_BASE_URL>/form.html?client=<client>&id=<prod_external_id>
\`\`\`

**Links**
- Test form: \`/form.html?client=<client>&id=<test_external_id>\`
- Editor: \`/builder.html?token=<admin_token>\` (token is resolved on enable)
`;

export const rawformSubmissionTrigger = createTrigger({
  name: 'rawform_submission',
  displayName: 'Rawform Submission',
  auth: PieceAuth.None(),
  description: 'Triggers when rawform forwards a form submission webhook.',
  aiMetadata: {
    description:
      'Fires for every rawform submission webhook payload. Supports a production mode that creates/deletes forms and a test mode that only toggles active state.',
  },
  type: TriggerStrategy.WEBHOOK,
  props: {
    mode: Property.StaticDropdown({
      displayName: 'Mode',
      required: true,
      options: {
        disabled: false,
        options: modeOptions,
      },
      defaultValue: 'test',
    }),
    client: Property.ShortText({
      displayName: 'Client',
      required: true,
      description: 'rawform client name',
    }),
    prod_external_id: Property.ShortText({
      displayName: 'Production External ID',
      required: true,
      description: 'Stable production external_id in rawform',
    }),
    test_external_id: Property.ShortText({
      displayName: 'Test External ID',
      required: true,
      description: 'Existing test external_id used in test mode',
    }),
    form_schema_json: Property.LongText({
      displayName: 'Form Schema JSON',
      required: true,
      description:
        'JSON used by onEnable in production mode (and as fallback in test mode if needed).',
      defaultValue: '{"title":"Form","elements":[]}',
    }),
    show_urls: Property.MarkDown({
      value: setupMarkdown,
    }),
  },
  sampleData: {
    event: 'form.submission',
    form_id: 1,
    external_id: 'contact-prod',
    values: {
      email: 'alice@example.com',
      message: 'Hi',
    },
    _rawform: {
      prod_form_url: 'https://rawform.example/form.html?client=myclient&id=contact-prod',
      test_form_url: 'https://rawform.example/form.html?client=myclient&id=contact-test',
      editor_url: 'https://rawform.example/builder.html?token=xxxxxxxx',
      managed_external_id: 'contact-prod',
      managed_mode: 'prod',
    },
  },
  async onEnable(context) {
    const config = getRawformConfig();
    const props = context.propsValue as unknown as TriggerProps;
    const mode = props.mode;
    const managedExternalId = resolveManagedExternalId(mode, props.prod_external_id, props.test_external_id);
    const schema = JSON.parse(props.form_schema_json) as Record<string, unknown>;

    let view;
    if (mode === 'prod') {
      view = await putForm(config, props.client, managedExternalId, {
        data: schema,
        webhook_url: context.webhookUrl,
      });
    } else {
      try {
        view = await patchForm(config, props.client, managedExternalId, {
          is_active: true,
          webhook_url: context.webhookUrl,
        });
      } catch (error) {
        if (!isNotFoundError(error)) {
          throw error;
        }
        view = await putForm(config, props.client, managedExternalId, {
          data: schema,
          webhook_url: context.webhookUrl,
        });
      }
    }

    const links: StoredLinks = {
      prod_form_url: buildFormUrl(config.baseUrl, props.client, props.prod_external_id),
      test_form_url: buildFormUrl(config.baseUrl, props.client, props.test_external_id),
      editor_url: buildEditorUrl(config.baseUrl, view.admin_token),
      managed_external_id: managedExternalId,
      managed_mode: mode,
    };

    await context.store.put('rawform.links', links);
  },
  async onDisable(context) {
    const config = getRawformConfig();
    const props = context.propsValue as unknown as TriggerProps;
    const mode = props.mode;
    const managedExternalId = resolveManagedExternalId(mode, props.prod_external_id, props.test_external_id);

    if (mode === 'prod') {
      await deleteForm(config, props.client, managedExternalId);
    } else {
      await patchForm(config, props.client, managedExternalId, {
        is_active: false,
      });
    }
  },
  async test(context) {
    const config = getRawformConfig();
    const props = context.propsValue as unknown as TriggerProps;
    const mode = props.mode;
    const managedExternalId = resolveManagedExternalId(mode, props.prod_external_id, props.test_external_id);
    const view = await getForm(config, props.client, managedExternalId);
    const links = (await context.store.get<StoredLinks>('rawform.links')) ?? {
      prod_form_url: buildFormUrl(config.baseUrl, props.client, props.prod_external_id),
      test_form_url: buildFormUrl(config.baseUrl, props.client, props.test_external_id),
      editor_url: buildEditorUrl(config.baseUrl, view.admin_token),
      managed_external_id: managedExternalId,
      managed_mode: mode,
    };
    return [
      {
        event: 'rawform.test',
        form_id: view.id,
        external_id: view.external_id,
        is_active: view.is_active,
        _rawform: links,
      },
    ];
  },
  async run(context) {
    const links = await context.store.get<StoredLinks>('rawform.links');
    const body = context.payload.body as Record<string, unknown>;
    return [
      {
        ...body,
        _rawform: links ?? null,
      },
    ];
  },
});
