# nmn-research-mcp

A small [Model Context Protocol](https://modelcontextprotocol.io) server for the [RGENR8 Longevity Data Lab](https://rgenr8.com/research). It lets an AI assistant look up published human trials of oral NMN and UK NMN prices.

It reads the public, read-only Data API at `https://rgenr8.com/data/api/v1/` ([docs](https://rgenr8.com/research/api)). If the network is down it answers from a copy of the data bundled in `data/`, and says so in each reply.

## Tools

| Tool | What it returns |
|---|---|
| `get_trials` | Published human trials of oral NMN. Filters: `year`, `country`, `dose_mg`, `min_dose_mg`, `max_dose_mg`, `randomised_only`, `placebo_only`. |
| `get_trial` | The full record for one trial, by `id` (for example `irie-2020`). |
| `get_prices` | Products in the UK NMN Price Index, cheapest per gram first. Filters: `brand`, `format`, `max_price_per_gram`, `limit`. |
| `cheapest_per_gram` | The lowest standard prices per gram of NMN. Filters: `format`, `mg_per_capsule`, `limit` (default 5). |
| `get_stats` | Citable summary statistics with methods and citations. Filters: `group` (`trials` or `prices`), `id`. |

Every reply includes the date the data was checked, the licence and the attribution line.

## Install

Requires Node.js 18 or later.

With npx (once the package is on npm):

```json
{
  "mcpServers": {
    "nmn-research": {
      "command": "npx",
      "args": ["-y", "nmn-research-mcp"]
    }
  }
}
```

Add that block to your MCP client config. For Claude Desktop the file is `claude_desktop_config.json`; for Claude Code you can run:

```bash
claude mcp add nmn-research -- npx -y nmn-research-mcp
```

From source:

```bash
git clone https://github.com/RGENR8-NMN/nmn-research-data.git
cd nmn-research-data/mcp
npm install
npm test            # lists the tools and calls a few of them
```

Then point your client at `node /path/to/nmn-research-data/mcp/index.js`.

## Settings

| Variable | Default | Purpose |
|---|---|---|
| `NMN_API_BASE` | `https://rgenr8.com/data/api/v1` | API base URL |
| `NMN_OFFLINE` | unset | Set to `1` to use the bundled data and skip the network |
| `NMN_TIMEOUT_MS` | `8000` | Request timeout |

`npm run sync-data` refreshes the bundled copy from the live API.

## Data, licence and limits

The data is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Credit "RGENR8 Longevity Data Lab, https://rgenr8.com/research". Archived copy: [doi.org/10.5281/zenodo.23285128](https://doi.org/10.5281/zenodo.23285128). The server code is MIT licensed.

The trials data records what studies measured and what their authors reported. It is not health advice, and NMN has no authorised health claims in the UK. Prices are standard one-off prices on the date checked; testing and certificate details are as each brand states them.
