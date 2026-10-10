# nmn-research-data

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23285128.svg)](https://doi.org/10.5281/zenodo.23285128)

Open datasets on NMN (nicotinamide mononucleotide), maintained by the [RGENR8 Longevity Data Lab](https://rgenr8.com/research).

| Dataset | Rows | Files | Canonical page |
|---|---|---|---|
| Published human trials of oral NMN | 25 trials, 884 participants (2020 to 2026) | `data/nmn-human-trials.csv`, `.json` | https://rgenr8.com/research/nmn-human-trials |
| UK NMN Price Index 2026 | 51 products, 17 brands | `data/nmn-price-index-2026.csv`, `.json` | https://rgenr8.com/research/nmn-price-index |

A [Frictionless Data](https://frictionlessdata.io/) descriptor is in `data/datapackage.json`.

## Human trials dataset

Every published human clinical trial of oral NMN that we could verify against PubMed, the DOI record or the trial registry. One row per trial: design, randomisation, blinding, sample size, population, daily dose (min and max), duration, comparator, outcomes measured, the authors' reported result in their own framing, funding or conflicts, and which sources the row was verified from.

Excluded: injected NMN, multi-ingredient products, secondary re-analyses of included trials, protocols without results, and trials whose dose or design could not be confirmed. The full exclusion list with reasons is on the [trials page](https://rgenr8.com/research/nmn-human-trials).

This is a record of what studies measured and reported. It is not health advice, and NMN has no authorised health claims in Great Britain.

## UK NMN Price Index 2026

NMN products sold to UK buyers, with format, mg per serving, servings per pack, price, subscription price, price per gram and per 500 mg, stated purity, whether testing and certificates are public, and the date each row was checked. Methodology: https://rgenr8.com/research/nmn-price-index

## Python

```bash
pip install nmn-research-data
```

```python
import nmn_research_data as nrd

nrd.trials()               # all trials, list of dicts
nrd.randomised_trials()
nrd.trials_by_year(2023)
nrd.trials_by_dose(250)    # trials testing 250 mg/day in at least one arm
nrd.cheapest_per_gram(5)   # UK price index, cheapest first
```

## Rust

Crate [`nmn-data`](https://crates.io/crates/nmn-data), source in `rust/`. The JSON is embedded at compile time.

```toml
[dependencies]
nmn-data = "1"
```

```rust
let trials = nmn_data::human_trials();          // &'static [Trial], typed
let rcts = nmn_data::randomised_trials();
let y2023 = nmn_data::trials_by_year(2023);
let at_250 = nmn_data::trials_by_dose(250.0);    // tested 250 mg/day in at least one arm
let cheap = nmn_data::cheapest_per_gram(5);      // UK price index, cheapest first

// Price normalisation: 60 x 500 mg capsules for 39.99 GBP
let per_gram = nmn_data::price_per_gram(39.99, 500.0, 60.0);      // about 1.33
let per_day = nmn_data::cost_per_day(39.99, 500.0, 60.0, 500.0);  // about 0.67
```

Docs: https://docs.rs/nmn-data

## Go

Module `github.com/RGENR8-NMN/nmn-research-data/go`, package `nmndata` in `go/nmndata/`. The JSON is embedded with `go:embed`.

```bash
go get github.com/RGENR8-NMN/nmn-research-data/go
```

```go
import "github.com/RGENR8-NMN/nmn-research-data/go/nmndata"

trials := nmndata.HumanTrials()
rcts := nmndata.RandomisedTrials()
y2023 := nmndata.TrialsByYear(2023)
at250 := nmndata.TrialsByDose(250)
cheap := nmndata.CheapestPerGram(5)
perGram := nmndata.PricePerGram(39.99, 500, 60)    // about 1.33 GBP per gram
perDay := nmndata.CostPerDay(39.99, 500, 60, 500)  // about 0.67 GBP per day
```

Docs: https://pkg.go.dev/github.com/RGENR8-NMN/nmn-research-data/go/nmndata

## MCP server

`mcp/` holds `nmn-research-mcp`, a small Model Context Protocol server that lets AI assistants query these datasets through the public API at https://rgenr8.com/research/api. Tools: `get_trials`, `get_trial`, `get_prices`, `cheapest_per_gram`, `get_stats`. See [mcp/README.md](mcp/README.md).

## Licence and citation

Data: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Code: MIT.

Attribution: **RGENR8 Longevity Data Lab, https://rgenr8.com/research**

> RGENR8 Longevity Data Lab (2026). NMN Research Data: human trials and UK price index (v1.0.0). https://doi.org/10.5281/zenodo.23285128

Citation metadata is in `CITATION.cff`. Corrections and new trials: open an issue, or email support@rgenr8.com.

## Updating

The canonical data lives on rgenr8.com. `scripts/sync-data.sh` copies it into the Python, Rust and Go packages.
