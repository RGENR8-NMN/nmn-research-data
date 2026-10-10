// Package nmndata provides typed, embedded open datasets on NMN (nicotinamide
// mononucleotide), maintained by the RGENR8 Longevity Data Lab
// (https://rgenr8.com/research).
//
// Two datasets are compiled into the package with go:embed, so there is no network
// access and no file I/O at runtime:
//
//   - Published human trials of oral NMN: 25 trials verified against PubMed, the DOI
//     record or the trial registry. Canonical page:
//     https://rgenr8.com/research/nmn-human-trials
//   - UK NMN Price Index 2026: 51 product listings sold to UK buyers, with price per
//     gram. Canonical page: https://rgenr8.com/research/nmn-price-index
//
// Archived release: https://doi.org/10.5281/zenodo.23285128
//
// This package is a record of what studies measured and what their authors reported.
// It is not health advice, and NMN has no authorised health claims in Great Britain.
//
// Code is MIT licensed. The data is CC BY 4.0, attribution
// "RGENR8 Longevity Data Lab, https://rgenr8.com/research".
package nmndata

import (
	_ "embed"
	"encoding/json"
	"math"
	"sort"
	"strconv"
	"strings"
	"sync"
)

// Attribution is the required attribution line for the CC BY 4.0 datasets.
const Attribution = "RGENR8 Longevity Data Lab, https://rgenr8.com/research"

// SourceURL is the home page of the RGENR8 Longevity Data Lab.
const SourceURL = "https://rgenr8.com/research"

// TrialsURL is the canonical page for the human trials dataset.
const TrialsURL = "https://rgenr8.com/research/nmn-human-trials"

// PriceIndexURL is the canonical page and methodology for the UK NMN Price Index.
const PriceIndexURL = "https://rgenr8.com/research/nmn-price-index"

// DOI is the Zenodo DOI of the archived dataset release.
const DOI = "10.5281/zenodo.23285128"

//go:embed data/nmn-human-trials.json
var trialsJSON []byte

//go:embed data/nmn-price-index-2026.json
var pricesJSON []byte

// Trial is one published human trial of oral NMN. Text fields are empty when the
// source has no value; optional numbers are nil.
type Trial struct {
	ID                      string
	Title                   string
	FirstAuthor             string
	Year                    int // 0 if unknown
	Journal                 string
	DOI                     string
	PMID                    string
	Country                 string
	Design                  string
	Randomised              string // "Yes", "No" or "Not stated"
	PlaceboControlled       string // "Yes" or "No"
	Blinded                 string
	NRandomised             *int
	NParticipants           *int
	Population              string
	AgeRange                string
	DailyDoseMg             string // as text, for example "300; 600; 1200"
	DoseMinMg               *float64
	DoseMaxMg               *float64
	DurationWeeks           *float64 // nil for single-dose studies
	DurationText            string
	Comparator              string
	PrimaryOutcomesMeasured string
	AuthorsReportedResult   string // the authors' own framing; a record, not a claim
	FundingOrConflicts      string
	Registration            string
	Route                   string
	VerifiedFrom            string
	PubMedURL               string
	DOIURL                  string
	DateChecked             string
}

// IsRandomised reports whether the dataset records the trial as randomised.
func (t Trial) IsRandomised() bool { return t.Randomised == "Yes" }

// IsPlaceboControlled reports whether the dataset records the trial as placebo controlled.
func (t Trial) IsPlaceboControlled() bool { return t.PlaceboControlled == "Yes" }

// TestedDose reports whether mg per day falls inside the trial's tested range
// (DoseMinMg to DoseMaxMg inclusive; a missing maximum is treated as the minimum).
func (t Trial) TestedDose(mg float64) bool {
	if t.DoseMinMg == nil {
		return false
	}
	max := *t.DoseMinMg
	if t.DoseMaxMg != nil {
		max = *t.DoseMaxMg
	}
	return *t.DoseMinMg <= mg && mg <= max
}

// PriceRow is one product listing in the UK NMN Price Index 2026. Prices are in GBP.
// Descriptive fields record what the retailer's page stated; they are not verified claims.
type PriceRow struct {
	Brand                   string
	Product                 string
	URL                     string
	Format                  string // "capsule", "powder", "liposomal" or "sublingual"
	MgNMNPerServing         *float64
	ServingsPerPack         *float64
	NMNGramsPerPack         *float64
	PriceGBP                *float64
	SubscriptionPriceGBP    string // free text
	PricePerGramGBP         *float64
	PricePer500mgGBP        *float64
	OtherActives            string
	StatedPurity            string
	ThirdPartyTestingPublic string
	COAAvailable            string
	CountryOfManufacture    string // as stated on the page
	ShipsFrom               string
	Notes                   string
	DateChecked             string
}

// str returns a trimmed string for strings and numbers, "" otherwise.
func str(m map[string]interface{}, k string) string {
	switch v := m[k].(type) {
	case string:
		return strings.TrimSpace(v)
	case float64:
		return strconv.FormatFloat(v, 'f', -1, 64)
	case bool:
		return strconv.FormatBool(v)
	}
	return ""
}

// num returns a number for numbers and numeric strings, nil otherwise.
func num(m map[string]interface{}, k string) *float64 {
	switch v := m[k].(type) {
	case float64:
		return &v
	case string:
		f, err := strconv.ParseFloat(strings.TrimSpace(v), 64)
		if err == nil {
			return &f
		}
	}
	return nil
}

func integer(m map[string]interface{}, k string) *int {
	f := num(m, k)
	if f == nil || *f < 0 || *f != math.Trunc(*f) {
		return nil
	}
	i := int(*f)
	return &i
}

func parseTrials(b []byte) ([]Trial, error) {
	var raw []map[string]interface{}
	if err := json.Unmarshal(b, &raw); err != nil {
		return nil, err
	}
	out := make([]Trial, 0, len(raw))
	for _, m := range raw {
		year := 0
		if y := integer(m, "year"); y != nil {
			year = *y
		}
		out = append(out, Trial{
			ID:                      str(m, "id"),
			Title:                   str(m, "title"),
			FirstAuthor:             str(m, "first_author"),
			Year:                    year,
			Journal:                 str(m, "journal"),
			DOI:                     str(m, "doi"),
			PMID:                    str(m, "pmid"),
			Country:                 str(m, "country"),
			Design:                  str(m, "design"),
			Randomised:              str(m, "randomised"),
			PlaceboControlled:       str(m, "placebo_controlled"),
			Blinded:                 str(m, "blinded"),
			NRandomised:             integer(m, "n_randomised"),
			NParticipants:           integer(m, "n_participants"),
			Population:              str(m, "population"),
			AgeRange:                str(m, "age_range"),
			DailyDoseMg:             str(m, "daily_dose_mg"),
			DoseMinMg:               num(m, "dose_min_mg"),
			DoseMaxMg:               num(m, "dose_max_mg"),
			DurationWeeks:           num(m, "duration_weeks"),
			DurationText:            str(m, "duration_text"),
			Comparator:              str(m, "comparator"),
			PrimaryOutcomesMeasured: str(m, "primary_outcomes_measured"),
			AuthorsReportedResult:   str(m, "authors_reported_result"),
			FundingOrConflicts:      str(m, "funding_or_conflicts"),
			Registration:            str(m, "registration"),
			Route:                   str(m, "route"),
			VerifiedFrom:            str(m, "verified_from"),
			PubMedURL:               str(m, "pubmed_url"),
			DOIURL:                  str(m, "doi_url"),
			DateChecked:             str(m, "date_checked"),
		})
	}
	return out, nil
}

func parsePrices(b []byte) ([]PriceRow, error) {
	var raw []map[string]interface{}
	if err := json.Unmarshal(b, &raw); err != nil {
		return nil, err
	}
	out := make([]PriceRow, 0, len(raw))
	for _, m := range raw {
		out = append(out, PriceRow{
			Brand:                   str(m, "brand"),
			Product:                 str(m, "product"),
			URL:                     str(m, "url"),
			Format:                  str(m, "format"),
			MgNMNPerServing:         num(m, "mg_nmn_per_serving"),
			ServingsPerPack:         num(m, "servings_per_pack"),
			NMNGramsPerPack:         num(m, "nmn_grams_per_pack"),
			PriceGBP:                num(m, "price_gbp"),
			SubscriptionPriceGBP:    str(m, "subscription_price_gbp"),
			PricePerGramGBP:         num(m, "price_per_gram_gbp"),
			PricePer500mgGBP:        num(m, "price_per_500mg_gbp"),
			OtherActives:            str(m, "other_actives"),
			StatedPurity:            str(m, "stated_purity"),
			ThirdPartyTestingPublic: str(m, "third_party_testing_public"),
			COAAvailable:            str(m, "coa_available"),
			CountryOfManufacture:    str(m, "country_of_manufacture"),
			ShipsFrom:               str(m, "ships_from"),
			Notes:                   str(m, "notes"),
			DateChecked:             str(m, "date_checked"),
		})
	}
	return out, nil
}

var (
	trialsOnce sync.Once
	trials     []Trial
	pricesOnce sync.Once
	prices     []PriceRow
)

func allTrials() []Trial {
	trialsOnce.Do(func() {
		t, err := parseTrials(trialsJSON)
		if err != nil {
			panic("nmndata: embedded nmn-human-trials.json is invalid: " + err.Error())
		}
		trials = t
	})
	return trials
}

func allPrices() []PriceRow {
	pricesOnce.Do(func() {
		p, err := parsePrices(pricesJSON)
		if err != nil {
			panic("nmndata: embedded nmn-price-index-2026.json is invalid: " + err.Error())
		}
		prices = p
	})
	return prices
}

// HumanTrials returns all published human trials of oral NMN in dataset order
// (oldest first). The returned slice is a copy and safe to modify.
func HumanTrials() []Trial {
	return filterTrials(func(Trial) bool { return true })
}

func filterTrials(keep func(Trial) bool) []Trial {
	var out []Trial
	for _, t := range allTrials() {
		if keep(t) {
			out = append(out, t)
		}
	}
	return out
}

// RandomisedTrials returns the trials the dataset records as randomised.
func RandomisedTrials() []Trial {
	return filterTrials(Trial.IsRandomised)
}

// TrialsByYear returns the trials published in year.
func TrialsByYear(year int) []Trial {
	return filterTrials(func(t Trial) bool { return t.Year == year })
}

// TrialsByDose returns the trials that tested mg per day in at least one arm.
func TrialsByDose(mg float64) []Trial {
	return filterTrials(func(t Trial) bool { return t.TestedDose(mg) })
}

// PriceIndex returns all rows of the UK NMN Price Index 2026 in dataset order.
// The returned slice is a copy and safe to modify.
func PriceIndex() []PriceRow {
	p := allPrices()
	out := make([]PriceRow, len(p))
	copy(out, p)
	return out
}

// CheapestPerGram returns the n cheapest listings by price per gram of NMN, cheapest
// first. Rows without a price per gram are skipped; ties keep dataset order.
func CheapestPerGram(n int) []PriceRow {
	var rows []PriceRow
	for _, p := range allPrices() {
		if p.PricePerGramGBP != nil && !math.IsNaN(*p.PricePerGramGBP) && !math.IsInf(*p.PricePerGramGBP, 0) {
			rows = append(rows, p)
		}
	}
	sort.SliceStable(rows, func(i, j int) bool {
		return *rows[i].PricePerGramGBP < *rows[j].PricePerGramGBP
	})
	if n < 0 {
		n = 0
	}
	if n < len(rows) {
		rows = rows[:n]
	}
	return rows
}

// PricePerGram normalises a retail price to GBP per gram of NMN. priceGBP is the pack
// price, mgPerUnit the mg of NMN in one capsule, scoop or serving, and units how many
// are in the pack. It returns NaN when the pack holds no NMN.
//
// For example 60 capsules of 500 mg for 39.99 GBP is 30 g, about 1.33 GBP per gram.
func PricePerGram(priceGBP, mgPerUnit, units float64) float64 {
	grams := mgPerUnit * units / 1000
	if grams <= 0 {
		return math.NaN()
	}
	return priceGBP / grams
}

// CostPerDay returns the cost in GBP of dailyMg of NMN per day, given a pack's price and
// contents. This is price arithmetic only, not a dosing recommendation.
func CostPerDay(priceGBP, mgPerUnit, units, dailyMg float64) float64 {
	return PricePerGram(priceGBP, mgPerUnit, units) * dailyMg / 1000
}
