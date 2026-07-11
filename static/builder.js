// rawform builder.js — minimal form builder UI

const state = { elements: [] };

function slugify(str) {
  return str.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '');
}

function addElement(type) {
  const base = { id: crypto.randomUUID(), type, label: '', name: '', nameOverridden: false, required: false };
  switch (type) {
    case 'text':
    case 'textarea':
    case 'checkbox':
      state.elements.push({ ...base, placeholder: '' });
      break;
    case 'dropdown':
      state.elements.push({ ...base, options: [] });
      break;
    default: return;
  }
  render();
}

function removeElement(id) {
  state.elements = state.elements.filter(e => e.id !== id);
  render();
}

function moveElement(id, dir) {
  const i = state.elements.findIndex(e => e.id === id);
  const j = i + dir;
  if (j < 0 || j >= state.elements.length) return;
  [state.elements[i], state.elements[j]] = [state.elements[j], state.elements[i]];
  render();
}

function updateField(id, field, value) {
  const el = state.elements.find(e => e.id === id);
  if (!el) return;
  el[field] = value;
  if (field === 'label' && !el.nameOverridden) {
    el.name = slugify(value);
  }
  render();
}

function addOption(id) {
  const el = state.elements.find(e => e.id === id);
  if (!el || !el.options) return;
  el.options.push({ label: '', value: '', valueOverridden: false });
  render();
}

function removeOption(id, idx) {
  const el = state.elements.find(e => e.id === id);
  if (!el) return;
  el.options.splice(idx, 1);
  render();
}

function updateOption(id, idx, field, value) {
  const el = state.elements.find(e => e.id === id);
  if (!el) return;
  const opt = el.options[idx];
  opt[field] = value;
  if (field === 'label' && !opt.valueOverridden) {
    opt.value = slugify(value);
  }
  render();
}

function makeField(label, input) {
  const row = document.createElement('div');
  row.className = 'field-row';
  const lbl = document.createElement('label');
  lbl.textContent = label;
  row.appendChild(lbl);
  row.appendChild(input);
  return row;
}

function makeInput(type, value, onchange) {
  const inp = document.createElement('input');
  inp.type = type;
  if (type === 'checkbox') inp.checked = value;
  else inp.value = value;
  inp.addEventListener('change', e => onchange(type === 'checkbox' ? e.target.checked : e.target.value));
  inp.addEventListener('input', e => { if (type !== 'checkbox') onchange(e.target.value); });
  return inp;
}

function makeNameRow(el) {
  const row = document.createElement('div');
  row.className = 'field-row';
  const lbl = document.createElement('label');
  lbl.textContent = 'Field name';
  row.appendChild(lbl);

  const wrap = document.createElement('div');
  wrap.className = 'name-override';

  const inp = makeInput('text', el.name, v => {
    el.name = v;
    el.nameOverridden = v !== '' && v !== slugify(el.label);
  });
  inp.placeholder = 'derived from label';

  const resetBtn = document.createElement('button');
  resetBtn.textContent = '↺';
  resetBtn.title = 'Reset to derived value';
  resetBtn.addEventListener('click', () => {
    el.nameOverridden = false;
    el.name = slugify(el.label);
    render();
  });

  // Reset on blur if empty
  inp.addEventListener('blur', () => {
    if (inp.value === '') {
      el.nameOverridden = false;
      el.name = slugify(el.label);
      render();
    }
  });

  wrap.appendChild(inp);
  wrap.appendChild(resetBtn);
  row.appendChild(wrap);
  return row;
}

function makeOptionsBuilder(el) {
  const container = document.createElement('div');
  container.className = 'options-builder';

  const title = document.createElement('label');
  title.textContent = 'Options';
  container.appendChild(title);

  el.options.forEach((opt, idx) => {
    const row = document.createElement('div');
    row.className = 'option-row';

    const labelInp = makeInput('text', opt.label, v => updateOption(el.id, idx, 'label', v));
    labelInp.placeholder = 'Label';

    const valueInp = makeInput('text', opt.value, v => {
      el.options[idx].value = v;
      el.options[idx].valueOverridden = v !== '' && v !== slugify(el.options[idx].label);
    });
    valueInp.placeholder = 'value (derived)';
    valueInp.title = 'Override the submitted value';

    const resetBtn = document.createElement('button');
    resetBtn.textContent = '↺';
    resetBtn.className = 'remove-option';
    resetBtn.title = 'Reset value';
    resetBtn.addEventListener('click', () => {
      el.options[idx].valueOverridden = false;
      el.options[idx].value = slugify(el.options[idx].label);
      render();
    });
    valueInp.addEventListener('blur', () => {
      if (valueInp.value === '') {
        el.options[idx].valueOverridden = false;
        el.options[idx].value = slugify(el.options[idx].label);
        render();
      }
    });

    const removeBtn = document.createElement('button');
    removeBtn.textContent = '✕';
    removeBtn.className = 'remove-option';
    removeBtn.addEventListener('click', () => removeOption(el.id, idx));

    row.appendChild(labelInp);
    row.appendChild(valueInp);
    row.appendChild(resetBtn);
    row.appendChild(removeBtn);
    container.appendChild(row);
  });

  const addBtn = document.createElement('button');
  addBtn.textContent = '+ Add option';
  addBtn.className = 'add-option-btn';
  addBtn.addEventListener('click', () => addOption(el.id));
  container.appendChild(addBtn);

  return container;
}

function renderCard(el, idx) {
  const card = document.createElement('div');
  card.className = 'element-card';

  // Header
  const header = document.createElement('div');
  header.className = 'card-header';

  const orderBtns = document.createElement('div');
  orderBtns.className = 'order-btns';
  const upBtn = document.createElement('button');
  upBtn.textContent = '▲';
  upBtn.title = 'Move up';
  upBtn.addEventListener('click', () => moveElement(el.id, -1));
  const downBtn = document.createElement('button');
  downBtn.textContent = '▼';
  downBtn.title = 'Move down';
  downBtn.addEventListener('click', () => moveElement(el.id, 1));
  orderBtns.appendChild(upBtn);
  orderBtns.appendChild(downBtn);

  const typeLabel = document.createElement('span');
  typeLabel.className = 'type-label';
  typeLabel.textContent = el.type;

  const deleteBtn = document.createElement('button');
  deleteBtn.className = 'delete-btn';
  deleteBtn.textContent = '✕';
  deleteBtn.title = 'Delete element';
  deleteBtn.addEventListener('click', () => removeElement(el.id));

  header.appendChild(orderBtns);
  header.appendChild(typeLabel);
  header.appendChild(deleteBtn);
  card.appendChild(header);

  // Fields
  const fields = document.createElement('div');
  fields.className = 'fields';

  fields.appendChild(makeField('Label', makeInput('text', el.label, v => updateField(el.id, 'label', v))));
  fields.appendChild(makeNameRow(el));

  if (el.type === 'text' || el.type === 'textarea') {
    const phInp = makeInput('text', el.placeholder, v => updateField(el.id, 'placeholder', v));
    phInp.placeholder = 'Placeholder text';
    fields.appendChild(makeField('Placeholder', phInp));
  }

  if (el.type === 'dropdown') {
    fields.appendChild(makeOptionsBuilder(el));
  }

  const reqInp = makeInput('checkbox', el.required, v => updateField(el.id, 'required', v));
  fields.appendChild(makeField('Required', reqInp));

  card.appendChild(fields);
  return card;
}

function render() {
  const builder = document.getElementById('builder');
  builder.innerHTML = '';
  state.elements.forEach((el, idx) => builder.appendChild(renderCard(el, idx)));
}

function exportJSON() {
  const title = document.getElementById('form-title').value || null;
  const output = {
    title,
    elements: state.elements.map(({ id, nameOverridden, ...el }) => {
      if (el.options) {
        el.options = el.options.map(({ valueOverridden, ...opt }) => opt);
      }
      return el;
    }),
  };
  const pre = document.getElementById('output');
  pre.style.display = 'block';
  pre.textContent = JSON.stringify(output, null, 2);
}

document.getElementById('add-btn').addEventListener('click', () => {
  const sel = document.getElementById('element-type-select');
  if (sel.value) { addElement(sel.value); sel.value = ''; }
});

document.getElementById('export-btn').addEventListener('click', exportJSON);
