// rawform form.js — render and submit a form for end users

const params = new URLSearchParams(location.search);
let submitToken = params.get('token');
const clientName = params.get('client');
const externalId = params.get('id');

function showError(msg) {
  document.getElementById('form-loading').hidden = true;
  const el = document.getElementById('form-error');
  el.textContent = msg;
  el.hidden = false;
}

function renderForm(data) {
  document.getElementById('form-loading').hidden = true;
  document.getElementById('form-title-display').textContent = data.title ?? '';

  const container = document.getElementById('form-fields');
  (data.elements ?? []).forEach(el => {
    const group = document.createElement('div');
    group.className = 'preview-group';

    if (el.type === 'checkbox') {
      const lbl = document.createElement('label');
      lbl.className = 'preview-checkbox-label';
      const inp = document.createElement('input');
      inp.type = 'checkbox';
      inp.name = el.name;
      if (el.required) inp.required = true;
      lbl.appendChild(inp);
      lbl.append(' ' + (el.label || el.name));
      group.appendChild(lbl);
    } else {
      const lbl = document.createElement('label');
      lbl.textContent = el.label || el.name;
      if (el.required) {
        const r = document.createElement('span');
        r.textContent = ' *'; r.className = 'preview-required';
        lbl.appendChild(r);
      }
      group.appendChild(lbl);

      let inp;
      if (el.type === 'textarea') {
        inp = document.createElement('textarea');
        inp.placeholder = el.placeholder ?? '';
      } else if (el.type === 'dropdown') {
        inp = document.createElement('select');
        const empty = document.createElement('option');
        empty.value = ''; empty.textContent = '— Select —';
        inp.appendChild(empty);
        (el.options ?? []).forEach(o => {
          const opt = document.createElement('option');
          opt.value = o.value; opt.textContent = o.label;
          inp.appendChild(opt);
        });
      } else {
        inp = document.createElement('input');
        inp.type = 'text';
        inp.placeholder = el.placeholder ?? '';
      }
      inp.name = el.name;
      if (el.required) inp.required = true;
      group.appendChild(inp);
    }
    container.appendChild(group);
  });

  document.getElementById('form-container').hidden = false;
}

async function initPage() {
  try {
    if (!submitToken && clientName && externalId) {
      // Resolve via public token endpoint
      const res = await fetch(`/api/v1/submit/${encodeURIComponent(clientName)}/${encodeURIComponent(externalId)}/token`);
      if (!res.ok) { showError('Form not found.'); return; }
      const json = await res.json();
      submitToken = json.submit_token;
      renderForm(json.data);
      return;
    }
    if (!submitToken) { showError('No form token in URL.'); return; }
    const res = await fetch(`/api/v1/submit/${encodeURIComponent(submitToken)}`);
    if (!res.ok) { showError('Form not found.'); return; }
    const json = await res.json();
    renderForm(json.data);
  } catch {
    showError('Network error.');
  }
}

document.getElementById('submit-form').addEventListener('submit', async () => {
  if (!submitToken) return;
  const form = document.getElementById('submit-form');
  const values = {};
  new FormData(form).forEach((v, k) => { values[k] = v; });

  const btn = document.getElementById('submit-btn');
  btn.disabled = true;
  btn.textContent = 'Submitting…';

  try {
    const res = await fetch(`/api/v1/submit/${encodeURIComponent(submitToken)}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ values }),
    });
    if (res.ok) {
      form.hidden = true;
      document.getElementById('submit-success').hidden = false;
    } else {
      const json = await res.json();
      btn.disabled = false;
      btn.textContent = 'Submit';
      alert(json.error ?? 'Submission failed.');
    }
  } catch {
    btn.disabled = false;
    btn.textContent = 'Submit';
    alert('Network error.');
  }
});

initPage();
