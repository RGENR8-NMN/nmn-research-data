#!/usr/bin/env node
// nmn-research-mcp: a small MCP server (stdio) for the RGENR8 Longevity Data Lab.
// It reads the public, read-only API at https://rgenr8.com/data/api/v1/ and falls
// back to the JSON bundled in ./data if the network is unavailable.
//
// Data: CC BY 4.0, attribution "RGENR8 Longevity Data Lab, https://rgenr8.com/research".
// The trials data records what studies measured and what their authors reported.
// It is not health advice, and NMN has no authorised health claims in the UK.
import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';

const HERE = dirname(fileURLToPath(import.meta.url));
const API = (process.env.NMN_API_BASE || 'https://rgenr8.com/data/api/v1').replace(/\/$/, '');
const OFFLINE = process.env.NMN_OFFLINE === '1';
const TIMEOUT_MS = Number(process.env.NMN_TIMEOUT_MS || 8000);
const VERSION = JSON.parse(readFileSync(join(HERE, 'package.json'), 'utf8')).version;

// ---------------------------------------------------------------- data access
const cache = new Map();
function bundled(name) {
  return JSON.parse(readFileSync(join(HERE, 'data', `${name}.json`), 'utf8'));
}
async function load(name) {
  if (cache.has(name)) return cache.get(name);
  let body = null, source = 'live';
  if (!OFFLINE) {
    try {
      const ctrl = new AbortController();
      const timer = setTimeout(() => ctrl.abort(), TIMEOUT_MS);
      const res = await fetch(`${API}/${name}.json`, { signal: ctrl.signal, headers: { accept: 'application/json', 'user-agent': `nmn-research-mcp/${VERSION}` } });
      clearTimeout(timer);
      if (res.ok) body = await res.json();
    } catch { /* fall through to the bundled copy */ }
  }
  if (!body || !Array.isArray(body.data)) { body = bundled(name); source = 'bundled'; }
  const out = { ...body, _source: source };
  cache.set(name, out);
  return out;
}

const metaLine = (d) => ({
  source: d._source === 'live' ? `${API}` : 'bundled copy (network unavailable)',
  checked: d.meta?.checked,
  licence: d.meta?.licence,
  attribution: d.meta?.attribution,
  doi: d.meta?.doi,
});
const reply = (payload) => ({ content: [{ type: 'text', text: JSON.stringify(payload, null, 2) }] });
const fail = (text) => ({ isError: true, content: [{ type: 'text', text }] });

function arms(t) {
  return String(t.daily_dose_mg).replace(/\(.*?\)/g, '').split(';').map((s) => parseFloat(s)).filter(Number.isFinite);
}
const trialSummary = (t) => ({
  id: t.id, title: t.title, first_author: t.first_author, year: t.year, country: t.country, design: t.design,
  randomised: t.randomised, placebo_controlled: t.placebo_controlled, n_participants: t.n_participants,
  daily_dose_mg: t.daily_dose_mg, duration_text: t.duration_text, page_url: t.page_url,
});
const priceSummary = (r) => ({
  rank: r.rank_price_per_gram, brand: r.brand, product: r.product, format: r.format,
  price_gbp: r.price_gbp, nmn_grams_per_pack: r.nmn_grams_per_pack, price_per_gram_gbp: r.price_per_gram_gbp,
  mg_nmn_per_capsule: r.mg_nmn_per_capsule, coa_category: r.coa_category, date_checked: r.date_checked,
});

// ---------------------------------------------------------------- server
const server = new McpServer({ name: 'nmn-research-mcp', version: VERSION });
const FORMATS = ['capsule', 'powder', 'sublingual', 'liposomal'];

server.registerTool('get_trials', {
  title: 'List human trials of oral NMN',
  description: 'Published human trials of oral NMN from the RGENR8 NMN Human Trials Dataset (verified against PubMed or DOI records). Filter by year, country, dose or design. Records what was measured and what authors reported; not health advice.',
  inputSchema: {
    year: z.number().int().optional().describe('Publication year, e.g. 2023'),
    country: z.string().optional().describe('Country where the trial ran, e.g. Japan'),
    dose_mg: z.number().positive().optional().describe('Only trials with this exact daily dose in at least one arm'),
    min_dose_mg: z.number().positive().optional().describe('Only trials whose highest daily dose is at least this'),
    max_dose_mg: z.number().positive().optional().describe('Only trials whose highest daily dose is at most this'),
    randomised_only: z.boolean().optional().describe('Only randomised trials'),
    placebo_only: z.boolean().optional().describe('Only placebo-controlled trials'),
  },
}, async (a) => {
  const d = await load('trials');
  let rows = d.data;
  if (a.year) rows = rows.filter((t) => t.year === a.year);
  if (a.country) rows = rows.filter((t) => t.country.toLowerCase() === a.country.toLowerCase());
  if (a.dose_mg) rows = rows.filter((t) => arms(t).includes(a.dose_mg));
  if (a.min_dose_mg) rows = rows.filter((t) => t.dose_max_mg >= a.min_dose_mg);
  if (a.max_dose_mg) rows = rows.filter((t) => t.dose_max_mg <= a.max_dose_mg);
  if (a.randomised_only) rows = rows.filter((t) => t.randomised === 'Yes');
  if (a.placebo_only) rows = rows.filter((t) => t.placebo_controlled === 'Yes');
  return reply({ meta: metaLine(d), count: rows.length, of: d.data.length, trials: rows.map(trialSummary) });
});

server.registerTool('get_trial', {
  title: 'Get one NMN trial',
  description: 'Full record for one trial by id (for example "irie-2020"): design, participants, dose, duration, outcomes measured, the authors\' reported result, funding and registration.',
  inputSchema: { id: z.string().describe('Trial id, e.g. irie-2020. Use get_trials to list ids.') },
}, async ({ id }) => {
  const d = await load('trials');
  const t = d.data.find((x) => x.id === id.trim().toLowerCase());
  if (!t) return fail(`No trial with id "${id}". Known ids: ${d.data.map((x) => x.id).join(', ')}`);
  return reply({ meta: metaLine(d), trial: t });
});

server.registerTool('get_prices', {
  title: 'UK NMN Price Index',
  description: 'Products in the UK NMN Price Index 2026 with standard price, grams of NMN per pack and price per gram, sorted cheapest per gram first. Testing and certificate details are as each brand states them.',
  inputSchema: {
    brand: z.string().optional().describe('Brand name or slug, e.g. "RGENR8"'),
    format: z.enum(FORMATS).optional().describe('capsule, powder, sublingual or liposomal'),
    max_price_per_gram: z.number().positive().optional().describe('Only products at or below this price per gram (GBP)'),
    limit: z.number().int().positive().max(100).optional().describe('Maximum rows (default all)'),
  },
}, async (a) => {
  const d = await load('prices');
  let rows = d.data;
  if (a.brand) { const b = a.brand.toLowerCase(); rows = rows.filter((r) => r.brand.toLowerCase() === b || r.brand_slug === b); }
  if (a.format) rows = rows.filter((r) => r.format === a.format);
  if (a.max_price_per_gram) rows = rows.filter((r) => r.price_per_gram_gbp !== null && r.price_per_gram_gbp <= a.max_price_per_gram);
  if (a.limit) rows = rows.slice(0, a.limit);
  return reply({ meta: metaLine(d), median_price_per_gram_gbp: d.median_price_per_gram_gbp, count: rows.length, products: rows.map(priceSummary) });
});

server.registerTool('cheapest_per_gram', {
  title: 'Cheapest NMN per gram',
  description: 'The lowest standard prices per gram of NMN in the UK NMN Price Index, optionally for one format or capsule strength. Prices are standard one-off prices on the date checked.',
  inputSchema: {
    format: z.enum(FORMATS).optional().describe('capsule, powder, sublingual or liposomal'),
    mg_per_capsule: z.number().positive().optional().describe('Only capsules holding this much NMN each, e.g. 500'),
    limit: z.number().int().positive().max(50).optional().describe('How many to return (default 5)'),
  },
}, async (a) => {
  const d = await load('prices');
  let rows = d.data.filter((r) => r.price_per_gram_gbp !== null);
  if (a.format) rows = rows.filter((r) => r.format === a.format);
  if (a.mg_per_capsule) rows = rows.filter((r) => r.mg_nmn_per_capsule === a.mg_per_capsule);
  rows = [...rows].sort((x, y) => x.price_per_gram_gbp - y.price_per_gram_gbp).slice(0, a.limit || 5);
  return reply({ meta: metaLine(d), note: 'Ranked by standard price per gram of NMN. Equal prices share a rank.', products: rows.map(priceSummary) });
});

server.registerTool('get_stats', {
  title: 'NMN statistics 2026 (UK)',
  description: 'Citable summary statistics computed from the trials dataset and the UK price index, each with its method and a one-line citation. No trial outcome tallies and no health claims.',
  inputSchema: {
    group: z.enum(['trials', 'prices']).optional().describe('Only trial or only price statistics'),
    id: z.string().optional().describe('One statistic by id, e.g. prices-median-per-gram'),
  },
}, async (a) => {
  const d = await load('stats');
  let rows = d.data;
  if (a.group) rows = rows.filter((s) => s.group === a.group);
  if (a.id) rows = rows.filter((s) => s.id === a.id);
  if (a.id && !rows.length) return fail(`No statistic with id "${a.id}". Known ids: ${d.data.map((s) => s.id).join(', ')}`);
  return reply({ meta: metaLine(d), count: rows.length, statistics: rows.map(({ id, display, statement, citation, method, url }) => ({ id, display, statement, citation, method, url })) });
});

await server.connect(new StdioServerTransport());
