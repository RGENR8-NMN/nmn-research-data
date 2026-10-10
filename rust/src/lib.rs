//! # nmn-data
//!
//! Typed, embedded open datasets on NMN (nicotinamide mononucleotide), maintained by the
//! [RGENR8 Longevity Data Lab](https://rgenr8.com/research).
//!
//! Two datasets ship inside the crate (embedded at compile time with `include_str!`, so
//! there is no network access and no file I/O at runtime):
//!
//! * **Published human trials of oral NMN**: 25 trials verified against PubMed, the DOI
//!   record or the trial registry. Canonical page:
//!   <https://rgenr8.com/research/nmn-human-trials>
//! * **UK NMN Price Index 2026**: 51 product listings sold to UK buyers, with price per
//!   gram. Canonical page: <https://rgenr8.com/research/nmn-price-index>
//!
//! Archived release: DOI [10.5281/zenodo.23285128](https://doi.org/10.5281/zenodo.23285128).
//!
//! This crate is a record of what studies measured and what their authors reported. It is
//! not health advice, and NMN has no authorised health claims in Great Britain.
//!
//! ## Example
//!
//! ```
//! let trials = nmn_data::human_trials();
//! assert_eq!(trials.len(), 25);
//!
//! // Trials that tested 250 mg per day in at least one arm.
//! for t in nmn_data::trials_by_dose(250.0) {
//!     println!("{} ({}): n = {:?}", t.first_author.as_deref().unwrap_or("?"), t.year.unwrap_or(0), t.n_participants);
//! }
//!
//! // Normalise a retail price: 60 capsules of 500 mg for 39.99 GBP.
//! let per_gram = nmn_data::price_per_gram(39.99, 500.0, 60.0);
//! assert!((per_gram - 1.333).abs() < 0.001);
//! ```
//!
//! ## Licence
//!
//! Code: MIT. Data: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), attribution
//! "RGENR8 Longevity Data Lab, <https://rgenr8.com/research>" (see [`ATTRIBUTION`]).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::sync::OnceLock;

/// Required attribution line for the CC BY 4.0 datasets.
pub const ATTRIBUTION: &str = "RGENR8 Longevity Data Lab, https://rgenr8.com/research";

/// Home page of the RGENR8 Longevity Data Lab, where the canonical datasets live.
pub const SOURCE_URL: &str = "https://rgenr8.com/research";

/// Canonical page for the human trials dataset.
pub const TRIALS_URL: &str = "https://rgenr8.com/research/nmn-human-trials";

/// Canonical page and methodology for the UK NMN Price Index.
pub const PRICE_INDEX_URL: &str = "https://rgenr8.com/research/nmn-price-index";

/// Zenodo DOI of the archived dataset release.
pub const DOI: &str = "10.5281/zenodo.23285128";

const TRIALS_JSON: &str = include_str!("../data/nmn-human-trials.json");
const PRICES_JSON: &str = include_str!("../data/nmn-price-index-2026.json");

/// Accepts a string, number, bool or null. Empty or whitespace-only strings become `None`.
fn opt_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::String(s) if s.trim().is_empty() => None,
        Value::String(s) => Some(s),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

/// Accepts a number or a numeric string. Anything else (including "", "n/a") becomes `None`.
fn opt_f64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    })
}

/// Accepts a non-negative whole number or a numeric string.
fn opt_u32<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Number(n) => n.as_u64().and_then(|v| u32::try_from(v).ok()),
        Value::String(s) => s.trim().parse::<u32>().ok(),
        _ => None,
    })
}

/// One published human trial of oral NMN.
///
/// Text fields are kept as the dataset records them. Fields that may be missing or empty
/// in the source are `None`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Trial {
    /// Stable identifier, for example `"irie-2020"`.
    pub id: String,
    /// Title of the publication.
    #[serde(default, deserialize_with = "opt_string")]
    pub title: Option<String>,
    /// First author, for example `"Irie J"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub first_author: Option<String>,
    /// Publication year.
    #[serde(default, deserialize_with = "opt_u32")]
    pub year: Option<u32>,
    /// Journal name.
    #[serde(default, deserialize_with = "opt_string")]
    pub journal: Option<String>,
    /// DOI without the `https://doi.org/` prefix.
    #[serde(default, deserialize_with = "opt_string")]
    pub doi: Option<String>,
    /// PubMed identifier.
    #[serde(default, deserialize_with = "opt_string")]
    pub pmid: Option<String>,
    /// Country where the trial ran.
    #[serde(default, deserialize_with = "opt_string")]
    pub country: Option<String>,
    /// Study design as described by the authors.
    #[serde(default, deserialize_with = "opt_string")]
    pub design: Option<String>,
    /// `"Yes"`, `"No"` or `"Not stated"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub randomised: Option<String>,
    /// `"Yes"` or `"No"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub placebo_controlled: Option<String>,
    /// Blinding as reported, for example `"Double-blind"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub blinded: Option<String>,
    /// Number randomised, where reported.
    #[serde(default, deserialize_with = "opt_u32")]
    pub n_randomised: Option<u32>,
    /// Number of participants analysed or enrolled, as reported.
    #[serde(default, deserialize_with = "opt_u32")]
    pub n_participants: Option<u32>,
    /// Study population.
    #[serde(default, deserialize_with = "opt_string")]
    pub population: Option<String>,
    /// Age range or summary as reported.
    #[serde(default, deserialize_with = "opt_string")]
    pub age_range: Option<String>,
    /// Daily dose as text, for example `"300; 600; 1200"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub daily_dose_mg: Option<String>,
    /// Lowest daily dose tested in any arm, in mg.
    #[serde(default, deserialize_with = "opt_f64")]
    pub dose_min_mg: Option<f64>,
    /// Highest daily dose tested in any arm, in mg.
    #[serde(default, deserialize_with = "opt_f64")]
    pub dose_max_mg: Option<f64>,
    /// Duration in weeks. `None` for single-dose studies.
    #[serde(default, deserialize_with = "opt_f64")]
    pub duration_weeks: Option<f64>,
    /// Duration as described in the paper.
    #[serde(default, deserialize_with = "opt_string")]
    pub duration_text: Option<String>,
    /// Comparator arm, if any.
    #[serde(default, deserialize_with = "opt_string")]
    pub comparator: Option<String>,
    /// Primary outcomes the study measured.
    #[serde(default, deserialize_with = "opt_string")]
    pub primary_outcomes_measured: Option<String>,
    /// The result in the authors' own framing. A record of what was reported, not a claim.
    #[serde(default, deserialize_with = "opt_string")]
    pub authors_reported_result: Option<String>,
    /// Funding sources or declared conflicts of interest.
    #[serde(default, deserialize_with = "opt_string")]
    pub funding_or_conflicts: Option<String>,
    /// Trial registry identifier.
    #[serde(default, deserialize_with = "opt_string")]
    pub registration: Option<String>,
    /// Route of administration (always oral in this dataset).
    #[serde(default, deserialize_with = "opt_string")]
    pub route: Option<String>,
    /// Which sources the row was verified from.
    #[serde(default, deserialize_with = "opt_string")]
    pub verified_from: Option<String>,
    /// PubMed URL.
    #[serde(default, deserialize_with = "opt_string")]
    pub pubmed_url: Option<String>,
    /// DOI URL.
    #[serde(default, deserialize_with = "opt_string")]
    pub doi_url: Option<String>,
    /// Date the row was last checked (ISO 8601).
    #[serde(default, deserialize_with = "opt_string")]
    pub date_checked: Option<String>,
}

impl Trial {
    /// `true` when the dataset records the trial as randomised (`"Yes"`).
    ///
    /// ```
    /// let n = nmn_data::human_trials().iter().filter(|t| t.is_randomised()).count();
    /// assert_eq!(n, nmn_data::randomised_trials().len());
    /// ```
    pub fn is_randomised(&self) -> bool {
        self.randomised.as_deref() == Some("Yes")
    }

    /// `true` when the dataset records the trial as placebo controlled (`"Yes"`).
    ///
    /// ```
    /// assert!(nmn_data::human_trials().iter().any(|t| t.is_placebo_controlled()));
    /// ```
    pub fn is_placebo_controlled(&self) -> bool {
        self.placebo_controlled.as_deref() == Some("Yes")
    }

    /// `true` when `mg` per day falls inside this trial's tested range
    /// (`dose_min_mg..=dose_max_mg`; a missing maximum is treated as equal to the minimum).
    ///
    /// ```
    /// let t = nmn_data::human_trials().iter().find(|t| t.id == "irie-2020").unwrap();
    /// assert!(t.tested_dose(250.0));
    /// assert!(!t.tested_dose(1000.0));
    /// ```
    pub fn tested_dose(&self, mg: f64) -> bool {
        match self.dose_min_mg {
            Some(min) => {
                let max = self.dose_max_mg.unwrap_or(min);
                min <= mg && mg <= max
            }
            None => false,
        }
    }
}

/// One product listing in the UK NMN Price Index 2026.
///
/// Prices are in GBP as checked on `date_checked`. Descriptive fields record what the
/// retailer's page stated; they are not verified claims.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PriceRow {
    /// Brand name.
    #[serde(default, deserialize_with = "opt_string")]
    pub brand: Option<String>,
    /// Product name as listed.
    #[serde(default, deserialize_with = "opt_string")]
    pub product: Option<String>,
    /// Product page URL.
    #[serde(default, deserialize_with = "opt_string")]
    pub url: Option<String>,
    /// `"capsule"`, `"powder"`, `"liposomal"` or `"sublingual"`.
    #[serde(default, deserialize_with = "opt_string")]
    pub format: Option<String>,
    /// mg of NMN per serving.
    #[serde(default, deserialize_with = "opt_f64")]
    pub mg_nmn_per_serving: Option<f64>,
    /// Servings per pack.
    #[serde(default, deserialize_with = "opt_f64")]
    pub servings_per_pack: Option<f64>,
    /// Grams of NMN per pack.
    #[serde(default, deserialize_with = "opt_f64")]
    pub nmn_grams_per_pack: Option<f64>,
    /// One-off price in GBP.
    #[serde(default, deserialize_with = "opt_f64")]
    pub price_gbp: Option<f64>,
    /// Subscription pricing as described on the page (free text).
    #[serde(default, deserialize_with = "opt_string")]
    pub subscription_price_gbp: Option<String>,
    /// Price per gram of NMN in GBP.
    #[serde(default, deserialize_with = "opt_f64")]
    pub price_per_gram_gbp: Option<f64>,
    /// Price per 500 mg of NMN in GBP.
    #[serde(default, deserialize_with = "opt_f64")]
    pub price_per_500mg_gbp: Option<f64>,
    /// Other active ingredients listed.
    #[serde(default, deserialize_with = "opt_string")]
    pub other_actives: Option<String>,
    /// Purity as stated by the seller.
    #[serde(default, deserialize_with = "opt_string")]
    pub stated_purity: Option<String>,
    /// Whether third-party test results are public.
    #[serde(default, deserialize_with = "opt_string")]
    pub third_party_testing_public: Option<String>,
    /// Whether a certificate of analysis is available.
    #[serde(default, deserialize_with = "opt_string")]
    pub coa_available: Option<String>,
    /// Country of manufacture as stated on the page.
    #[serde(default, deserialize_with = "opt_string")]
    pub country_of_manufacture: Option<String>,
    /// Shipping origin as stated on the page.
    #[serde(default, deserialize_with = "opt_string")]
    pub ships_from: Option<String>,
    /// Notes on how the row was derived.
    #[serde(default, deserialize_with = "opt_string")]
    pub notes: Option<String>,
    /// Date the row was checked (ISO 8601).
    #[serde(default, deserialize_with = "opt_string")]
    pub date_checked: Option<String>,
}

/// All published human trials of oral NMN, in dataset order (oldest first).
///
/// Parsed once on first call and cached for the life of the process.
///
/// ```
/// let trials = nmn_data::human_trials();
/// assert!(trials.iter().all(|t| t.doi.is_some() || t.pmid.is_some()));
/// ```
pub fn human_trials() -> &'static [Trial] {
    static CELL: OnceLock<Vec<Trial>> = OnceLock::new();
    CELL.get_or_init(|| {
        serde_json::from_str(TRIALS_JSON).expect("embedded nmn-human-trials.json is valid")
    })
}

/// Trials the dataset records as randomised.
///
/// ```
/// let rcts = nmn_data::randomised_trials();
/// assert!(!rcts.is_empty());
/// assert!(rcts.iter().all(|t| t.randomised.as_deref() == Some("Yes")));
/// ```
pub fn randomised_trials() -> Vec<&'static Trial> {
    human_trials().iter().filter(|t| t.is_randomised()).collect()
}

/// Trials published in `year`.
///
/// ```
/// let t2023 = nmn_data::trials_by_year(2023);
/// assert!(t2023.iter().all(|t| t.year == Some(2023)));
/// assert!(nmn_data::trials_by_year(1999).is_empty());
/// ```
pub fn trials_by_year(year: u32) -> Vec<&'static Trial> {
    human_trials()
        .iter()
        .filter(|t| t.year == Some(year))
        .collect()
}

/// Trials that tested `mg` per day in at least one arm (`dose_min_mg <= mg <= dose_max_mg`).
///
/// ```
/// let at_250 = nmn_data::trials_by_dose(250.0);
/// assert!(!at_250.is_empty());
/// assert!(at_250.iter().all(|t| t.tested_dose(250.0)));
/// ```
pub fn trials_by_dose(mg: f64) -> Vec<&'static Trial> {
    human_trials().iter().filter(|t| t.tested_dose(mg)).collect()
}

/// All rows of the UK NMN Price Index 2026, in dataset order.
///
/// ```
/// let rows = nmn_data::price_index();
/// assert_eq!(rows.len(), 51);
/// ```
pub fn price_index() -> &'static [PriceRow] {
    static CELL: OnceLock<Vec<PriceRow>> = OnceLock::new();
    CELL.get_or_init(|| {
        serde_json::from_str(PRICES_JSON).expect("embedded nmn-price-index-2026.json is valid")
    })
}

/// The `n` cheapest listings by price per gram of NMN, cheapest first.
///
/// Rows with no price per gram are skipped. Ties keep dataset order.
///
/// ```
/// let cheap = nmn_data::cheapest_per_gram(3);
/// assert_eq!(cheap.len(), 3);
/// assert!(cheap[0].price_per_gram_gbp <= cheap[1].price_per_gram_gbp);
/// ```
pub fn cheapest_per_gram(n: usize) -> Vec<&'static PriceRow> {
    let mut rows: Vec<&'static PriceRow> = price_index()
        .iter()
        .filter(|p| p.price_per_gram_gbp.is_some_and(|v| v.is_finite()))
        .collect();
    rows.sort_by(|a, b| {
        a.price_per_gram_gbp
            .partial_cmp(&b.price_per_gram_gbp)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows.truncate(n);
    rows
}

/// Normalises a retail price to GBP per gram of NMN.
///
/// `price_gbp` is the pack price, `mg_per_unit` the mg of NMN in one capsule, scoop or
/// serving, and `units` how many of those are in the pack. Returns `f64::NAN` when the pack
/// holds no NMN (zero or negative mg or units).
///
/// ```
/// // 60 capsules of 500 mg for 39.99 GBP is 30 g, so about 1.33 GBP per gram.
/// let p = nmn_data::price_per_gram(39.99, 500.0, 60.0);
/// assert!((p - 1.333).abs() < 0.001);
/// assert!(nmn_data::price_per_gram(10.0, 0.0, 30.0).is_nan());
/// ```
pub fn price_per_gram(price_gbp: f64, mg_per_unit: f64, units: f64) -> f64 {
    let grams = mg_per_unit * units / 1000.0;
    if grams > 0.0 {
        price_gbp / grams
    } else {
        f64::NAN
    }
}

/// Cost in GBP of `daily_mg` of NMN per day, given a pack's price and contents.
///
/// This is price arithmetic only. It is not a dosing recommendation.
///
/// ```
/// // 60 x 500 mg for 39.99 GBP, at 500 mg a day: about 0.67 GBP per day.
/// let c = nmn_data::cost_per_day(39.99, 500.0, 60.0, 500.0);
/// assert!((c - 0.6665).abs() < 0.001);
/// ```
pub fn cost_per_day(price_gbp: f64, mg_per_unit: f64, units: f64, daily_mg: f64) -> f64 {
    price_per_gram(price_gbp, mg_per_unit, units) * daily_mg / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trials_load_and_have_identifiers() {
        let t = human_trials();
        assert_eq!(t.len(), 25);
        assert!(t.iter().all(|r| !r.id.is_empty()));
        assert!(t.iter().all(|r| r.doi.is_some() || r.pmid.is_some()));
        assert!(t.iter().all(|r| r.year.is_some()));
    }

    #[test]
    fn dose_ranges_are_ordered() {
        for t in human_trials() {
            if let (Some(a), Some(b)) = (t.dose_min_mg, t.dose_max_mg) {
                assert!(a <= b, "{}", t.id);
            }
        }
    }

    #[test]
    fn filters() {
        assert!(randomised_trials().iter().all(|t| t.is_randomised()));
        assert!(!randomised_trials().is_empty());
        let by_year: usize = (2020..=2026).map(|y| trials_by_year(y).len()).sum();
        assert_eq!(by_year, human_trials().len());
        assert!(!trials_by_dose(250.0).is_empty());
        assert!(trials_by_dose(1.0).is_empty());
        // A multi-arm trial covers doses between its arms.
        assert!(trials_by_dose(1500.0).iter().any(|t| t.dose_max_mg == Some(2000.0)));
    }

    #[test]
    fn single_dose_studies_have_no_duration() {
        let t = human_trials().iter().find(|t| t.id == "irie-2020").unwrap();
        assert_eq!(t.duration_weeks, None);
    }

    #[test]
    fn prices_load_and_sort() {
        assert_eq!(price_index().len(), 51);
        let c = cheapest_per_gram(100);
        assert!(c.len() < 51, "rows without a price per gram are skipped");
        assert!(c.windows(2).all(|w| w[0].price_per_gram_gbp <= w[1].price_per_gram_gbp));
        assert!(cheapest_per_gram(0).is_empty());
    }

    #[test]
    fn price_helpers() {
        assert!((price_per_gram(39.99, 500.0, 60.0) - 1.333).abs() < 0.001);
        assert!((price_per_gram(60.0, 500.0, 30.0) - 4.0).abs() < 1e-9);
        assert!(price_per_gram(1.0, 500.0, 0.0).is_nan());
        assert!((cost_per_day(60.0, 500.0, 30.0, 500.0) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn tolerant_parsing() {
        let json = r#"[{"id":"x","year":"2024","dose_min_mg":"250","dose_max_mg":"","doi":"","n_participants":12,"extra":"ignored"}]"#;
        let rows: Vec<Trial> = serde_json::from_str(json).unwrap();
        let r = &rows[0];
        assert_eq!(r.year, Some(2024));
        assert_eq!(r.dose_min_mg, Some(250.0));
        assert_eq!(r.dose_max_mg, None);
        assert_eq!(r.doi, None);
        assert_eq!(r.n_participants, Some(12));
        assert!(r.tested_dose(250.0));
    }
}
