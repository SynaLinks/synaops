# synaops

Rust implementations of the JSON and JSON-schema operations used by
[synalinks](https://github.com/SynaLinks/synalinks), exposed to Python as a
PyO3 extension module (`synaops`).

The goal is a drop-in, faster replacement for the equivalent pure-Python
helpers. Input/output types are plain Python `dict` / `list` / scalars — the
boundary is handled via [`pythonize`](https://crates.io/crates/pythonize), so
callers do not need to know there is Rust underneath.

Parity with the Python reference is asserted on every op and payload size
(see `bench/test_parity.py`). Headline speedups on realistic payloads:
**~485× on `factorize_schema`**, **~280× on `factorize_json`** at 600 keys,
4–8× on masking ops, 2–4× on simple key rewrites. Full table below.

## Build

```bash
pip install maturin
maturin develop --release   # builds and installs into the active venv
```

## Python API

```python
import synaops
```

### JSON object operations

| Function | Signature | Description |
|---|---|---|
| `prefix_json` | `(json, prefix)` | Prepend `prefix_` to every top-level key. |
| `suffix_json` | `(json, suffix)` | Append `_suffix` to every top-level key. |
| `concatenate_json` | `(json1, json2)` | Merge two objects; on key collision append `_1`, `_2`, … to disambiguate. |
| `factorize_json` | `(json)` | Group keys sharing a singular base into a single array under the plural key. |
| `out_mask_json` | `(json, mask=None, pattern=None, recursive=True)` | Drop keys whose base name is in `mask` or whose base name matches the regex `pattern`. Numerical suffixes are ignored when matching. |
| `in_mask_json` | `(json, mask=None, pattern=None, recursive=True)` | Keep only the keys whose base name is in `mask` or matches `pattern`. In recursive mode, arrays are preserved and their object items are filtered in place. |

### JSON schema operations

Operate on JSON-Schema-shaped dicts (`properties`, `required`, `$defs`, `type`, …).

| Function | Signature | Description |
|---|---|---|
| `prefix_schema` | `(schema, prefix)` | Prepend `prefix_` to every property key and update `title` / `required` accordingly. |
| `suffix_schema` | `(schema, suffix)` | Append `_suffix` to every property key and update `title` / `required` accordingly. |
| `concatenate_schema` | `(schema1, schema2)` | Merge two schemas (properties, `required`, `$defs`); on key collision append numeric suffixes and regenerate titles. |
| `factorize_schema` | `(schema)` | Group similar singular-keyed properties into array-typed plural-keyed properties; folds heterogeneous `items` into `anyOf`. |
| `out_mask_schema` | `(schema, mask=None, pattern=None, recursive=True)` | Remove properties whose base name is in `mask` or matches `pattern`. With `recursive=True`, descends into nested object/array properties and `$defs`, then prunes `$defs` entries no longer referenced. |
| `in_mask_schema` | `(schema, mask=None, pattern=None, recursive=True)` | Keep only properties whose base name is in `mask` or matches `pattern`. Same recursive/`$defs`-pruning behavior as `out_mask_schema`. |
| `standardize_schema` | `(schema)` | Placeholder for schema normalization (currently identity). |

> `is_object`, `is_array`, `is_schema_equal`, `contains_schema` are intentionally **not** ported. They are single-key lookups or dict comparisons whose cost is dominated by the PyO3 / dict-conversion boundary, so the pure-Python versions in synalinks are faster.

## Matching semantics

Both `*_mask_*` families and `factorize_*` rely on the NLP
helpers in `nlp_utils.rs`: they strip trailing numerical suffixes
(`answer_3` → `answer`) and normalize singular/plural forms
(`answers` ↔ `answer`) before comparing keys. The `pattern` argument is a
regular expression matched via substring search against the base key (same
semantics as Python's `re.search`).

## Benchmark

Measured on realistic payloads: nested objects, arrays of `$ref`-based
objects, schema `$defs`. Three payload sizes (12, 96, 600 top-level keys).
Parity with the Python reference is verified before each timing run
(`pytest bench/test_parity.py`, 45/45 pass).

### Speedup summary

Ratio `py_median / rs_median` per op. Higher is better; dashed line is parity (1×).

![speedup](https://raw.githubusercontent.com/SynaLinks/synaops/main/bench/speedup.png)

| Operation | small (12) | medium (96) | large (600) |
|---|---:|---:|---:|
| `factorize_schema` | 8.78× | 64.8× | 485× |
| `factorize_json` | 9.73× | 46.2× | 282× |
| `in_mask_json` | 7.75× | 7.11× | 7.47× |
| `out_mask_json` | 4.23× | 4.12× | 4.29× |
| `out_mask_json_pattern` | 3.77× | 4.15× | 4.20× |
| `in_mask_schema` | 4.83× | 4.12× | 4.15× |
| `out_mask_schema` | 4.25× | 3.78× | 4.11× |
| `prefix_schema` | 3.63× | 3.62× | 3.89× |
| `suffix_schema` | 3.66× | 3.64× | 3.85× |
| `concatenate_schema` | 2.21× | 2.15× | 2.85× |
| `suffix_json` | 2.36× | 2.22× | 2.28× |
| `concatenate_json` | 2.25× | 2.18× | 2.27× |
| `prefix_json` | 2.26× | 2.25× | 2.26× |

`factorize_*` scales super-linearly because the Python reference does
repeated O(n) key scans per group; the Rust path groups in a single pass.
Simple key rewrites (`prefix_*`, `suffix_*`, `concatenate_*`) are bounded
by the PyO3 dict-conversion boundary, which is why they cluster around 2–4×.

### Before (Python) vs After (Rust) — absolute medians

Log scale, lower is better. Rows are the three payload sizes.

![before/after](https://raw.githubusercontent.com/SynaLinks/synaops/main/bench/before_after.png)

See `bench/README.md` for the harness, payload shapes, and how to
regenerate these charts.

## Development

```bash
cargo test              # run Rust unit tests
maturin develop         # install debug build into venv
maturin develop --release

# Parity + performance against the Python reference impl
pytest bench/test_parity.py -v
pytest bench/test_bench.py --benchmark-save=latest
python bench/plot.py    # writes bench/speedup.png
```

## License

Apache-2.0
