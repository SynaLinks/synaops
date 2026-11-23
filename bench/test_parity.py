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
    py_out = call(getattr(py_json, f"_py_{base}"), payload)
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
    py_out = call(getattr(py_sch, f"_py_{base}"), payload)
    rs_out = call(getattr(rs, base), payload)
    assert py_out == rs_out, f"{op_name}[{size_name}]: py != rs"


SCHEMA_STR = {"type": "string"}
SCHEMA_STRS = {"type": "array", "items": {"type": "string"}}
PLURAL_CASES = {
    "plural_only": ({"answers": ["a", "b"]}, {"answers": SCHEMA_STRS}),
    "plural_and_other": (
        {"query": "q", "answers": ["a"]},
        {"query": SCHEMA_STR, "answers": SCHEMA_STRS},
    ),
    "singular_then_plural": (
        {"answer": "x", "answers": ["y"]},
        {"answer": SCHEMA_STR, "answers": SCHEMA_STRS},
    ),
    "plural_then_singular": (
        {"answers": ["y"], "answer": "x"},
        {"answers": SCHEMA_STRS, "answer": SCHEMA_STR},
    ),
}


@pytest.mark.parametrize("case", list(PLURAL_CASES))
def test_factorize_plural_parity(case):
    # SynaLinks/synalinks#200: plural keys dropped, singular+plural lost data.
    from synalinks.src.backend.common import json_schema_utils as py_sch
    from synalinks.src.backend.common import json_utils as py_json
    import synaops as rs

    data, props = PLURAL_CASES[case]
    schema = {"title": "T", "type": "object", "properties": props, "required": list(props)}
    assert rs.factorize_json(data) == py_json._py_factorize_json(dict(data))
    assert rs.factorize_schema(schema) == py_sch._py_factorize_schema(
        {**schema, "properties": dict(props)}
    )
