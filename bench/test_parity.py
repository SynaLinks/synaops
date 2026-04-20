"""Parity: for each (op, size), assert py(input) == rs(input).

Parametrized so each op failure is reported independently.
"""

from __future__ import annotations

import pytest

from conftest import JSON_FNS, SCHEMA_FNS, SIZES


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", JSON_FNS, ids=[n for n, _ in JSON_FNS])
def test_json_parity(op, size_name, json_payload_by_size):
    op_name, call = op
    from synalinks.src.backend.common import json_utils as py_json
    import synaops as rs

    base = op_name.removesuffix("_pattern")
    payload = json_payload_by_size[size_name]
    py_out = call(getattr(py_json, base), payload)
    rs_out = call(getattr(rs, base), payload)
    assert py_out == rs_out, f"{op_name}[{size_name}]: py != rs"


@pytest.mark.parametrize("size_name", list(SIZES.keys()))
@pytest.mark.parametrize("op", SCHEMA_FNS, ids=[n for n, _ in SCHEMA_FNS])
def test_schema_parity(op, size_name, schema_payload_by_size):
    op_name, call = op
    from synalinks.src.backend.common import json_schema_utils as py_sch
    import synaops as rs

    base = op_name.removesuffix("_pattern")
    payload = schema_payload_by_size[size_name]
    py_out = call(getattr(py_sch, base), payload)
    rs_out = call(getattr(rs, base), payload)
    assert py_out == rs_out, f"{op_name}[{size_name}]: py != rs"
