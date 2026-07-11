import { HttpMethod, httpClient } from '@activepieces/pieces-common';

type RawformMode = 'test' | 'prod';

type RawformConfig = {
  baseUrl: string;
  apiKey: string;
};

type UpsertBody = {
  data: Record<string, unknown>;
  webhook_url?: string;
};

type PatchBody = {
  is_active?: boolean;
  webhook_url?: string | null;
};

export type RawformFormResponse = {
  id: number;
  external_id: string;
  data: Record<string, unknown>;
  is_active: boolean;
  webhook_url: string | null;
  admin_token?: string;
  submit_token?: string;
  created_at: string;
  updated_at: string;
};

export function getRawformConfig(): RawformConfig {
  const baseUrl = process.env.RAWFORM_BASE_URL?.trim();
  const apiKey = process.env.RAWFORM_API_KEY?.trim();

  if (!baseUrl) {
    throw new Error('Missing RAWFORM_BASE_URL environment variable.');
  }
  if (!apiKey) {
    throw new Error('Missing RAWFORM_API_KEY environment variable.');
  }

  return {
    baseUrl: baseUrl.replace(/\/+$/, ''),
    apiKey,
  };
}

function formApiUrl(baseUrl: string, client: string, externalId: string): string {
  return `${baseUrl}/api/v1/forms/${encodeURIComponent(client)}/${encodeURIComponent(externalId)}`;
}

function buildHeaders(apiKey: string): Record<string, string> {
  return {
    Authorization: `Bearer ${apiKey}`,
    'Content-Type': 'application/json',
  };
}

export async function putForm(
  config: RawformConfig,
  client: string,
  externalId: string,
  body: UpsertBody,
): Promise<RawformFormResponse> {
  const response = await httpClient.sendRequest<RawformFormResponse>({
    method: HttpMethod.PUT,
    url: formApiUrl(config.baseUrl, client, externalId),
    headers: buildHeaders(config.apiKey),
    body,
  });
  return response.body;
}

export async function patchForm(
  config: RawformConfig,
  client: string,
  externalId: string,
  body: PatchBody,
): Promise<RawformFormResponse> {
  const response = await httpClient.sendRequest<RawformFormResponse>({
    method: HttpMethod.PATCH,
    url: formApiUrl(config.baseUrl, client, externalId),
    headers: buildHeaders(config.apiKey),
    body,
  });
  return response.body;
}

export async function getForm(
  config: RawformConfig,
  client: string,
  externalId: string,
): Promise<RawformFormResponse> {
  const response = await httpClient.sendRequest<RawformFormResponse>({
    method: HttpMethod.GET,
    url: formApiUrl(config.baseUrl, client, externalId),
    headers: buildHeaders(config.apiKey),
  });
  return response.body;
}

export async function deleteForm(config: RawformConfig, client: string, externalId: string): Promise<void> {
  await httpClient.sendRequest({
    method: HttpMethod.DELETE,
    url: formApiUrl(config.baseUrl, client, externalId),
    headers: buildHeaders(config.apiKey),
  });
}

export function resolveManagedExternalId(mode: RawformMode, prodExternalId: string, testExternalId: string): string {
  return mode === 'prod' ? prodExternalId : testExternalId;
}

export function buildFormUrl(baseUrl: string, client: string, externalId: string): string {
  return `${baseUrl}/form.html?client=${encodeURIComponent(client)}&id=${encodeURIComponent(externalId)}`;
}

export function buildEditorUrl(baseUrl: string, adminToken?: string): string | null {
  if (!adminToken) {
    return null;
  }
  return `${baseUrl}/builder.html?token=${encodeURIComponent(adminToken)}`;
}

