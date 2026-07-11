document.getElementById('open-builder-btn').addEventListener('click', () => {
  const token = document.getElementById('admin-token-input').value.trim();
  if (token) window.location.href = `/builder.html?token=${encodeURIComponent(token)}`;
});

document.getElementById('open-form-token-btn').addEventListener('click', () => {
  const token = document.getElementById('submit-token-input').value.trim();
  if (token) window.location.href = `/form.html?token=${encodeURIComponent(token)}`;
});

document.getElementById('open-form-id-btn').addEventListener('click', () => {
  const client = document.getElementById('client-input').value.trim();
  const id = document.getElementById('id-input').value.trim();
  if (client && id)
    window.location.href = `/form.html?client=${encodeURIComponent(client)}&id=${encodeURIComponent(id)}`;
});

// Allow Enter key in inputs
document.querySelectorAll('input').forEach(inp => {
  inp.addEventListener('keydown', e => {
    if (e.key === 'Enter') inp.nextElementSibling?.click();
  });
});
