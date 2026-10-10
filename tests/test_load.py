import nmn_research_data as nrd


def test_trials_have_doi_or_pmid():
    rows = nrd.trials()
    assert len(rows) > 5
    assert all(r.get("doi") or r.get("pmid") for r in rows)


def test_prices_load():
    assert len(nrd.prices()) > 20
    cheap = nrd.cheapest_per_gram(3)
    assert len(cheap) == 3


def test_filters():
    assert nrd.randomised_trials()
    assert all(t["randomised"] == "Yes" for t in nrd.randomised_trials())
    assert nrd.trials_by_dose(250)
