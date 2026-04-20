# synaops benchmark

Parity + pytest-benchmark comparison between the Rust `synaops` extension and
the reference Python implementation in the synalinks package.

## Setup

From the repo root (`synalinks-core/`), in a fresh venv:

```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin
pip install -e ../synalinks          # reference Python impl
pip install -r bench/requirements.txt
maturin develop --release            # builds synaops into the venv
```

## Run

```bash
# Correctness: per-op, per-size, py(input) == rs(input)
pytest bench/test_parity.py -v

# Performance: side-by-side py vs rs, grouped by op+size
pytest bench/test_bench.py \
    --benchmark-columns=median,ops,rounds \
    --benchmark-group-by=group \
    --benchmark-sort=name

# Same run, but persist results so the plot script can read them
pytest bench/test_bench.py --benchmark-save=latest
python bench/plot.py    # writes bench/speedup.png
```

Each bench test sets `benchmark.group = "<op>[<size>]"` and tags
`extra_info["lang"]` with `py` or `rs`, so the grouped output shows both
implementations on adjacent rows. `plot.py` reads the most recent JSON
under `.benchmarks/` and renders a grouped speedup chart.

## Payload shapes

- `small` — 12 top-level keys
- `medium` — 96 top-level keys
- `large` — 600 top-level keys

Keys rotate through six realistic variants (`answer_*`, `item_*`,
`items_*`, `nested_*` with depth-3 objects, `person_*` via `$ref`,
`people_*` arrays of `$ref`-based objects) so every operation — including
`factorize_*` / `decompose_*` / `*_mask_*` and the schema paths that walk
`$defs` — has realistic work to do.
