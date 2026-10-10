# nmn-data

[![crates.io](https://img.shields.io/crates/v/nmn-data.svg)](https://crates.io/crates/nmn-data)
[![docs.rs](https://docs.rs/nmn-data/badge.svg)](https://docs.rs/nmn-data)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23285128.svg)](https://doi.org/10.5281/zenodo.23285128)

Typed, embedded open datasets on NMN (nicotinamide mononucleotide) for Rust, from the [RGENR8 Longevity Data Lab](https://rgenr8.com/research).

* **Published human trials of oral NMN**: 25 trials, 884 participants (2020 to 2026). Canonical page: https://rgenr8.com/research/nmn-human-trials
* **UK NMN Price Index 2026**: 51 product listings, 17 brands, with price per gram. Methodology: https://rgenr8.com/research/nmn-price-index

The data is compiled into the crate, so there is no network access or file I/O at runtime.

## Usage

```toml
[dependencies]
nmn-data = "1"
```

```rust
// Every trial, typed.
let trials = nmn_data::human_trials();

// Filters.
let rcts = nmn_data::randomised_trials();
let from_2023 = nmn_data::trials_by_year(2023);
let at_250mg = nmn_data::trials_by_dose(250.0); // tested 250 mg/day in at least one arm

// UK price index.
let cheapest = nmn_data::cheapest_per_gram(5);

// Price normalisation helpers: 60 x 500 mg capsules for 39.99 GBP.
let per_gram = nmn_data::price_per_gram(39.99, 500.0, 60.0);      // about 1.33
let per_day = nmn_data::cost_per_day(39.99, 500.0, 60.0, 500.0);  // about 0.67
```

This is a record of what studies measured and what their authors reported. It is not health advice, and NMN has no authorised health claims in Great Britain.

## Licence and citation

Code: MIT (`LICENSE-MIT`). Data: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) (`LICENSE-DATA`).

Attribution: **RGENR8 Longevity Data Lab, https://rgenr8.com/research**

> RGENR8 Longevity Data Lab (2026). NMN Research Data: human trials and UK price index (v1.0.0). https://doi.org/10.5281/zenodo.23285128

Source repository (Python, Rust and Go): https://github.com/RGENR8-NMN/nmn-research-data
