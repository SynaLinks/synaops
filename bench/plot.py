"""Read the latest pytest-benchmark JSON run and emit PNG charts.

Usage:
    pytest bench/test_bench.py --benchmark-save=latest
    python bench/plot.py

Produces:
- ``bench/speedup.png``  grouped bar chart: speedup (py/rs) per op + size
- ``bench/before_after.png``  per size, absolute medians of py (before) vs rs (after)
"""

from __future__ import annotations

import json
import re
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parent.parent
BENCH_DIR = ROOT / ".benchmarks"
OUT_DIR = Path(__file__).resolve().parent

NAME_RE = re.compile(
    r"test_(?:json|schema)_(?P<lang>py|rs)\["
    r"(?:json|sch):(?P<op>[a-z_]+)-(?P<size>small|medium|large)\]"
)
SIZES = ("small", "medium", "large")


def _latest_json() -> Path:
    candidates = sorted(BENCH_DIR.glob("*/*.json"), key=lambda p: p.stat().st_mtime)
    if not candidates:
        raise SystemExit(
            f"no benchmark runs found under {BENCH_DIR} — "
            f"run pytest with --benchmark-save=<tag> first"
        )
    return candidates[-1]


def _load() -> tuple[list[str], dict[tuple[str, str], dict[str, float]]]:
    data = json.loads(_latest_json().read_text())
    medians: dict[tuple[str, str], dict[str, float]] = defaultdict(dict)
    for entry in data["benchmarks"]:
        m = NAME_RE.search(entry["fullname"])
        if not m:
            continue
        medians[(m["op"], m["size"])][m["lang"]] = entry["stats"]["median"]
    ops = sorted({op for (op, _) in medians})
    return ops, medians


SIZE_COLORS = {"small": "#4c9be8", "medium": "#f0a04b", "large": "#4fae6f"}


def _fmt_speedup(v: float) -> str:
    if not np.isfinite(v):
        return ""
    if v >= 100:
        return f"{v:.0f}×"
    if v >= 10:
        return f"{v:.1f}×"
    return f"{v:.2f}×"


def _fmt_us(v: float) -> str:
    if not np.isfinite(v):
        return ""
    if v >= 1000:
        return f"{v / 1000:.1f}ms"
    if v >= 100:
        return f"{v:.0f}µs"
    if v >= 10:
        return f"{v:.1f}µs"
    return f"{v:.2f}µs"


def plot_speedup(ops, medians):
    speedups = {
        size: [
            (medians[(op, size)].get("py") or np.nan)
            / (medians[(op, size)].get("rs") or np.nan)
            for op in ops
        ]
        for size in SIZES
    }

    x = np.arange(len(ops))
    width = 0.27
    fig, ax = plt.subplots(figsize=(max(13, 0.95 * len(ops)), 7.5))
    for i, size in enumerate(SIZES):
        offs = (i - 1) * width
        vals = speedups[size]
        bars = ax.bar(
            x + offs, vals, width, label=size, color=SIZE_COLORS[size], edgecolor="white", linewidth=0.6
        )
        for rect, v in zip(bars, vals):
            if not np.isfinite(v):
                continue
            ax.text(
                rect.get_x() + rect.get_width() / 2,
                v * 1.08,
                _fmt_speedup(v),
                ha="center",
                va="bottom",
                fontsize=8,
                rotation=90,
            )

    ax.set_xticks(x)
    ax.set_xticklabels(ops, rotation=30, ha="right", fontsize=11)
    ax.tick_params(axis="y", labelsize=11)
    ax.axhline(1.0, color="gray", linewidth=1, linestyle="--")
    ax.text(len(ops) - 0.6, 1.0, "parity (1×)", color="gray", fontsize=9, va="bottom", ha="right")
    ax.set_ylabel("speedup  (py median / rs median)", fontsize=12)
    ax.set_title("synaops (Rust) vs synalinks Python reference — higher is better", fontsize=14)
    ax.set_yscale("log")
    ymax = max((v for s in SIZES for v in speedups[s] if np.isfinite(v)), default=10)
    ax.set_ylim(0.7, ymax * 3.5)
    ax.legend(title="payload size", fontsize=11, title_fontsize=11, loc="upper left")
    ax.grid(axis="y", alpha=0.35, which="major")
    ax.grid(axis="y", alpha=0.12, which="minor")
    ax.set_axisbelow(True)
    fig.tight_layout()
    out = OUT_DIR / "speedup.png"
    fig.savefig(out, dpi=150)
    plt.close(fig)
    return out


def plot_before_after(ops, medians):
    fig, axes = plt.subplots(
        len(SIZES), 1, figsize=(max(14, 0.95 * len(ops)), 4.2 * len(SIZES)), sharex=True
    )
    x = np.arange(len(ops))
    width = 0.4

    for ax, size in zip(axes, SIZES):
        py_vals = [medians[(op, size)].get("py") or np.nan for op in ops]
        rs_vals = [medians[(op, size)].get("rs") or np.nan for op in ops]
        py_us = [v * 1e6 for v in py_vals]
        rs_us = [v * 1e6 for v in rs_vals]
        py_bars = ax.bar(
            x - width / 2, py_us, width, label="before (Python)",
            color="#c94f4f", edgecolor="white", linewidth=0.6,
        )
        rs_bars = ax.bar(
            x + width / 2, rs_us, width, label="after  (Rust)",
            color="#3e7bb0", edgecolor="white", linewidth=0.6,
        )
        for bars, vals in ((py_bars, py_us), (rs_bars, rs_us)):
            for rect, v in zip(bars, vals):
                if not np.isfinite(v):
                    continue
                ax.text(
                    rect.get_x() + rect.get_width() / 2,
                    v * 1.12,
                    _fmt_us(v),
                    ha="center",
                    va="bottom",
                    fontsize=8,
                    rotation=90,
                )
        ax.set_yscale("log")
        all_vals = [v for v in py_us + rs_us if np.isfinite(v)]
        if all_vals:
            ax.set_ylim(min(all_vals) * 0.5, max(all_vals) * 6)
        ax.set_ylabel(f"{size}\nmedian (µs)", fontsize=12)
        ax.tick_params(axis="y", labelsize=11)
        ax.grid(axis="y", alpha=0.35, which="major")
        ax.grid(axis="y", alpha=0.12, which="minor")
        ax.set_axisbelow(True)
        if ax is axes[0]:
            ax.set_title(
                "Before (Python) vs After (Rust) — median time per op, log scale (lower is better)",
                fontsize=14,
            )
            ax.legend(loc="upper left", fontsize=11)

    axes[-1].set_xticks(x)
    axes[-1].set_xticklabels(ops, rotation=30, ha="right", fontsize=11)
    fig.tight_layout()
    out = OUT_DIR / "before_after.png"
    fig.savefig(out, dpi=150)
    plt.close(fig)
    return out


def main() -> None:
    ops, medians = _load()
    for p in (plot_speedup(ops, medians), plot_before_after(ops, medians)):
        print(f"wrote {p.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
