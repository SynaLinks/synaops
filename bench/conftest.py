"""Shared fixtures: payloads + both implementations side-by-side.

The Rust side is `synaops` (built via `maturin develop --release`).
The Python side is the reference impl in the synalinks package.
"""

from __future__ import annotations

import pytest
import synaops as rs
from synalinks.src.backend.common import json_schema_utils as py_sch
from synalinks.src.backend.common import json_utils as py_json


# --- module pairs -----------------------------------------------------------

JSON_FNS = [
    ("prefix_json", lambda f, d, **kw: f(d, "p")),
    ("suffix_json", lambda f, d, **kw: f(d, "s")),
    ("concatenate_json", lambda f, d, **kw: f(d, d)),
    ("factorize_json", lambda f, d, **kw: f(d)),
    ("decompose_json", lambda f, d, **kw: f(d)),
    (
        "out_mask_json",
        lambda f, d, **kw: f(d, mask=["answer", "item"], recursive=True),
    ),
    (
        "in_mask_json",
        lambda f, d, **kw: f(d, mask=["answer", "item"], recursive=True),
    ),
    (
        "out_mask_json_pattern",
        lambda f, d, **kw: f(d, pattern=r"^k\d", recursive=True),
    ),
]

SCHEMA_FNS = [
    ("prefix_schema", lambda f, s, **kw: f(s, "p")),
    ("suffix_schema", lambda f, s, **kw: f(s, "s")),
    ("concatenate_schema", lambda f, s, **kw: f(s, s)),
    ("factorize_schema", lambda f, s, **kw: f(s)),
    ("decompose_schema", lambda f, s, **kw: f(s)),
    (
        "out_mask_schema",
        lambda f, s, **kw: f(s, mask=["answer", "item"], recursive=True),
    ),
    (
        "in_mask_schema",
        lambda f, s, **kw: f(s, mask=["answer", "item"], recursive=True),
    ),
]


def _resolve(name: str, module):
    # strip "_pattern" variant suffix used only to pick a different call shape
    base = name.removesuffix("_pattern")
    return getattr(module, base)


@pytest.fixture
def ops_json():
    """Returns list of (name, py_fn, rs_fn, call_shape)."""
    return [
        (name, _resolve(name, py_json), _resolve(name, rs), call)
        for name, call in JSON_FNS
    ]


@pytest.fixture
def ops_schema():
    return [
        (name, _resolve(name, py_sch), _resolve(name, rs), call)
        for name, call in SCHEMA_FNS
    ]


# --- payloads ---------------------------------------------------------------


def _make_json(size: int) -> dict:
    """Realistic mixed payload: scalars, nested dicts (depth 3), lists of dicts."""
    d = {}
    for i in range(size):
        m = i % 6
        if m == 0:
            d[f"answer_{i}"] = f"value_{i} with some text"
        elif m == 1:
            d[f"item_{i}"] = i
        elif m == 2:
            d[f"items_{i}"] = [f"x_{j}" for j in range(5)]
        elif m == 3:
            d[f"nested_{i}"] = {
                "a": i,
                "b": f"s{i}",
                "deep": {
                    "x": bool(i % 2),
                    "y": [1.0, 2.0, 3.0, 4.0],
                    "meta": {"tag": f"t{i}", "rank": i * 0.5},
                },
            }
        elif m == 4:
            d[f"person_{i}"] = {
                "name": f"Person{i}",
                "email": f"p{i}@example.com",
                "address": {
                    "street": f"{i} Main St",
                    "city": "Paris",
                    "country": "FR",
                },
                "tags": [
                    {"name": "a", "weight": 1.0},
                    {"name": "b", "weight": 2.0},
                ],
            }
        else:
            d[f"people_{i}"] = [
                {
                    "name": f"P{i}_{j}",
                    "email": f"p{i}_{j}@example.com",
                    "address": {
                        "street": f"{j} Oak Ave",
                        "city": "Lyon",
                        "country": "FR",
                    },
                    "tags": [{"name": f"t{j}", "weight": j * 1.0}],
                }
                for j in range(3)
            ]
    return d


def _make_schema(size: int) -> dict:
    """Realistic JSON schema: $defs with cross-refs, nested objects, arrays-of-objects."""
    defs = {
        "Address": {
            "type": "object",
            "properties": {
                "street": {"type": "string"},
                "city": {"type": "string"},
                "zip": {"type": "string"},
                "country": {"type": "string"},
            },
            "required": ["street", "city", "country"],
        },
        "Tag": {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "weight": {"type": "number"},
            },
            "required": ["name"],
        },
        "Person": {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "email": {"type": "string", "format": "email"},
                "address": {"$ref": "#/$defs/Address"},
                "tags": {"type": "array", "items": {"$ref": "#/$defs/Tag"}},
            },
            "required": ["name"],
        },
    }

    props = {}
    required = []
    for i in range(size):
        m = i % 6
        if m == 0:
            key = f"answer_{i}"
            props[key] = {
                "title": f"Answer {i}",
                "type": "string",
                "description": f"answer field {i}",
            }
        elif m == 1:
            key = f"item_{i}"
            props[key] = {
                "title": f"Item {i}",
                "type": "integer",
                "minimum": 0,
            }
        elif m == 2:
            key = f"items_{i}"
            props[key] = {
                "title": f"Items {i}",
                "type": "array",
                "items": {"type": "string"},
            }
        elif m == 3:
            key = f"nested_{i}"
            props[key] = {
                "title": f"Nested {i}",
                "type": "object",
                "properties": {
                    "a": {"type": "integer"},
                    "b": {"type": "string"},
                    "deep": {
                        "type": "object",
                        "properties": {
                            "x": {"type": "boolean"},
                            "y": {
                                "type": "array",
                                "items": {"type": "number"},
                            },
                        },
                        "required": ["x"],
                    },
                },
                "required": ["a"],
            }
        elif m == 4:
            key = f"person_{i}"
            props[key] = {"$ref": "#/$defs/Person"}
        else:
            key = f"people_{i}"
            props[key] = {
                "title": f"People {i}",
                "type": "array",
                "items": {"$ref": "#/$defs/Person"},
            }
        required.append(key)

    return {
        "$defs": defs,
        "additionalProperties": False,
        "properties": props,
        "required": required,
        "title": "Payload",
        "type": "object",
    }


SIZES = {"small": 12, "medium": 96, "large": 600}


@pytest.fixture(params=list(SIZES.keys()))
def size(request):
    return request.param


@pytest.fixture
def json_payload(size):
    return _make_json(SIZES[size])


@pytest.fixture
def schema_payload(size):
    return _make_schema(SIZES[size])


@pytest.fixture(scope="session")
def json_payload_by_size():
    return {name: _make_json(n) for name, n in SIZES.items()}


@pytest.fixture(scope="session")
def schema_payload_by_size():
    return {name: _make_schema(n) for name, n in SIZES.items()}
