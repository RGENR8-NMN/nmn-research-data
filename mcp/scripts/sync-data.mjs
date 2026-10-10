// Refresh the bundled fallback copy from the live API (run before publishing).
import { writeFileSync } from 'node:fs';
const API = process.env.NMN_API_BASE || 'https://rgenr8.com/data/api/v1';
for (const name of ['trials', 'prices', 'stats']) {
  const res = await fetch(`${API}/${name}.json`);
  if (!res.ok) throw new Error(`${name}: HTTP ${res.status}`);
  const body = await res.json();
  writeFileSync(new URL(`../data/${name}.json`, import.meta.url), JSON.stringify(body, null, 2) + '\n');
  console.log(name, body.count, 'rows, checked', body.meta.checked);
}
