import { createPiece, PieceAuth } from '@activepieces/pieces-framework';
import { rawformSubmissionTrigger } from './lib/triggers/rawform-submission.trigger';

export const rawformPiece = createPiece({
  displayName: 'rawform',
  description: 'Trigger flows from rawform submissions',
  auth: PieceAuth.None(),
  minimumSupportedRelease: '0.36.1',
  logoUrl: 'https://raw.githubusercontent.com/Relacibo/rawform/main/static/favicon.svg',
  authors: ['Relacibo'],
  actions: [],
  triggers: [rawformSubmissionTrigger],
});
