"""NMN research datasets from the RGENR8 Longevity Data Lab.

Data: CC BY 4.0, attribution "RGENR8 Longevity Data Lab, https://rgenr8.com/research".
"""
import json
from importlib.resources import files

__version__ = "1.0.0"
SOURCE_URL = "https://rgenr8.com/research"


def _load(name):
    return json.loads(files(__package__).joinpath("data", name).read_text(encoding="utf-8"))


def trials():
    """All published human trials of oral NMN, one dict per trial."""
    return _load("nmn-human-trials.json")


def prices():
    """UK NMN Price Index rows, one dict per product listing."""
    return _load("nmn-price-index-2026.json")


def _num(value):
    try:
        return float(value)
    except (TypeError, ValueError):
        return None


def trials_by_year(year):
    return [t for t in trials() if str(t.get("year")) == str(year)]


def trials_by_dose(mg):
    """Trials that tested mg per day in at least one arm."""
    mg = float(mg)
    return [t for t in trials() if _num(t.get("dose_min_mg")) is not None
            and _num(t["dose_min_mg"]) <= mg <= _num(t.get("dose_max_mg") or t["dose_min_mg"])]


def randomised_trials():
    return [t for t in trials() if t.get("randomised") == "Yes"]


def cheapest_per_gram(limit=10):
    rows = [p for p in prices() if _num(p.get("price_per_gram_gbp")) is not None]
    return sorted(rows, key=lambda p: _num(p["price_per_gram_gbp"]))[:limit]
