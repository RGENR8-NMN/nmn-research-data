package nmndata

import (
	"math"
	"testing"
)

func TestTrialsLoad(t *testing.T) {
	ts := HumanTrials()
	if len(ts) != 25 {
		t.Fatalf("want 25 trials, got %d", len(ts))
	}
	for _, tr := range ts {
		if tr.ID == "" {
			t.Errorf("trial with empty id")
		}
		if tr.DOI == "" && tr.PMID == "" {
			t.Errorf("%s: no DOI or PMID", tr.ID)
		}
		if tr.DoseMinMg != nil && tr.DoseMaxMg != nil && *tr.DoseMinMg > *tr.DoseMaxMg {
			t.Errorf("%s: dose range reversed", tr.ID)
		}
	}
}

func TestFilters(t *testing.T) {
	r := RandomisedTrials()
	if len(r) == 0 {
		t.Fatal("no randomised trials")
	}
	for _, tr := range r {
		if !tr.IsRandomised() {
			t.Errorf("%s not randomised", tr.ID)
		}
	}
	total := 0
	for y := 2020; y <= 2026; y++ {
		total += len(TrialsByYear(y))
	}
	if total != len(HumanTrials()) {
		t.Errorf("years 2020 to 2026 cover %d trials, want %d", total, len(HumanTrials()))
	}
	if len(TrialsByDose(250)) == 0 {
		t.Error("expected trials at 250 mg")
	}
	if len(TrialsByDose(1)) != 0 {
		t.Error("expected no trials at 1 mg")
	}
}

func TestSingleDoseHasNoDuration(t *testing.T) {
	for _, tr := range HumanTrials() {
		if tr.ID == "irie-2020" {
			if tr.DurationWeeks != nil {
				t.Errorf("irie-2020 duration should be nil")
			}
			if !tr.TestedDose(250) || tr.TestedDose(1000) {
				t.Errorf("irie-2020 dose range wrong")
			}
			return
		}
	}
	t.Fatal("irie-2020 not found")
}

func TestPrices(t *testing.T) {
	if n := len(PriceIndex()); n != 51 {
		t.Fatalf("want 51 price rows, got %d", n)
	}
	c := CheapestPerGram(100)
	if len(c) >= 51 {
		t.Errorf("rows without price per gram should be skipped")
	}
	for i := 1; i < len(c); i++ {
		if *c[i-1].PricePerGramGBP > *c[i].PricePerGramGBP {
			t.Fatalf("not sorted at %d", i)
		}
	}
	if len(CheapestPerGram(0)) != 0 || len(CheapestPerGram(-1)) != 0 {
		t.Error("n <= 0 should return nothing")
	}
}

func TestPriceHelpers(t *testing.T) {
	if p := PricePerGram(39.99, 500, 60); math.Abs(p-1.333) > 0.001 {
		t.Errorf("PricePerGram = %v", p)
	}
	if !math.IsNaN(PricePerGram(1, 500, 0)) {
		t.Error("zero units should be NaN")
	}
	if c := CostPerDay(60, 500, 30, 500); math.Abs(c-2) > 1e-9 {
		t.Errorf("CostPerDay = %v", c)
	}
}

func TestTolerantParsing(t *testing.T) {
	ts, err := parseTrials([]byte(`[{"id":"x","year":"2024","dose_min_mg":"250","dose_max_mg":"","doi":"","n_participants":12}]`))
	if err != nil {
		t.Fatal(err)
	}
	tr := ts[0]
	if tr.Year != 2024 || tr.DoseMinMg == nil || *tr.DoseMinMg != 250 || tr.DoseMaxMg != nil || tr.DOI != "" || tr.NParticipants == nil || *tr.NParticipants != 12 {
		t.Errorf("unexpected parse: %+v", tr)
	}
}
