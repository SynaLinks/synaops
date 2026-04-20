"""Side-by-side pytest-benchmark comparisons for every op and size.

Each op/size/language combination is its own test so pytest-benchmark can
report them in a grouped table. Run with:

    pytest bench/test_bench.py --benchmark-columns=median,ops,rounds \\
        --benchmark-group-by=group --benchmark-sort=group

Groups look like `prefix_json[small]`, with `py` and `rs` rows.
"""

from __future__ import annotations

import pytest

from conftest import JSON_FNS, SCHEMA_FNS, SIZES


def _ids(prefix, fns):
    return [f"{prefix}:{n}" for n, _ in fns]


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", JSON_FNS, ids=_ids("json", JSON_FNS))
def test_json_py(benchmark, op, size_name, json_payload_by_size):
    op_name, call = op
    from synalinks.src.backend.common import json_utils as py_json

    base = op_name.removesuffix("_pattern")
    fn = getattr(py_json, base)
    payload = json_payload_by_size[size_name]

    benchmark.group = f"{op_name}[{size_name}]"
    benchmark.extra_info["lang"] = "py"
    benchmark(call, fn, payload)


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", JSON_FNS, ids=_ids("json", JSON_FNS))
def test_json_rs(benchmark, op, size_name, json_payload_by_size):
    op_name, call = op
    import synaops as rs

    base = op_name.removesuffix("_pattern")
    fn = getattr(rs, base)
    payload = json_payload_by_size[size_name]

    benchmark.group = f"{op_name}[{size_name}]"
    benchmark.extra_info["lang"] = "rs"
    benchmark(call, fn, payload)


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", SCHEMA_FNS, ids=_ids("sch", SCHEMA_FNS))
def test_schema_py(benchmark, op, size_name, schema_payload_by_size):
    op_name, call = op
    from synalinks.src.backend.common import json_schema_utils as py_sch

    base = op_name.removesuffix("_pattern")
    fn = getattr(py_sch, base)
    payload = schema_payload_by_size[size_name]

    benchmark.group = f"{op_name}[{size_name}]"
    benchmark.extra_info["lang"] = "py"
    benchmark(call, fn, payload)


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", SCHEMA_FNS, ids=_ids("sch", SCHEMA_FNS))
def test_schema_rs(benchmark, op, size_name, schema_payload_by_size):
    op_name, call = op
    import synaops as rs

    base = op_name.removesuffix("_pattern")
    fn = getattr(rs, base)
    payload = schema_payload_by_size[size_name]

    benchmark.group = f"{op_name}[{size_name}]"
    benchmark.extra_info["lang"] = "rs"
    benchmark(call, fn, payload)
