// Smoke test: start the server over stdio, list its tools and call a few.
//   node test.mjs            (live API, falls back to bundled data)
//   NMN_OFFLINE=1 node test.mjs
import { Client } from '@modelcontextprotocol/sdk/client/index.js';
import { StdioClientTransport } from '@modelcontextprotocol/sdk/client/stdio.js';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';

const transport = new StdioClientTransport({ command: process.execPath, args: [fileURLToPath(new URL('./index.js', import.meta.url))], env: { ...process.env } });
const client = new Client({ name: 'nmn-research-mcp-test', version: '1.0.0' });
await client.connect(transport);

const { tools } = await client.listTools();
const names = tools.map((t) => t.name).sort();
console.log('tools:', names.join(', '));
assert.deepEqual(names, ['cheapest_per_gram', 'get_prices', 'get_stats', 'get_trial', 'get_trials']);

const parse = (r) => JSON.parse(r.content[0].text);
const cheap = parse(await client.callTool({ name: 'cheapest_per_gram', arguments: { format: 'capsule', limit: 3 } }));
console.log('cheapest_per_gram (capsule, 3):', cheap.meta.source, cheap.products.map((p) => `${p.brand} £${p.price_per_gram_gbp}/g`).join('; '));
assert.equal(cheap.products.length, 3);
assert.equal(cheap.meta.licence, 'CC BY 4.0');

const trial = parse(await client.callTool({ name: 'get_trial', arguments: { id: 'irie-2020' } }));
console.log('get_trial irie-2020:', trial.trial.title.slice(0, 60) + '...');
assert.equal(trial.trial.id, 'irie-2020');

const trials = parse(await client.callTool({ name: 'get_trials', arguments: { randomised_only: true } }));
console.log('get_trials randomised_only:', trials.count, 'of', trials.of);

const stats = parse(await client.callTool({ name: 'get_stats', arguments: { id: 'prices-median-per-gram' } }));
console.log('get_stats:', stats.statistics[0].statement);

const bad = await client.callTool({ name: 'get_trial', arguments: { id: 'nope' } });
assert.equal(bad.isError, true);

await client.close();
console.log('ok');
