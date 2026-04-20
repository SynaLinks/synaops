// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)
//
// Process-wide cache of compiled `Regex` instances, keyed by pattern source.
// Mirrors Python's `re` module, which caches up to 512 recent patterns.
// `regex::Regex` is cheap to clone (Arc internally), so cache hits hand out
// clones without re-parsing.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use regex::Regex;

const MAX_CACHE: usize = 512;

static CACHE: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn compile(pattern: &str) -> Result<Regex, regex::Error> {
    {
        let cache = CACHE.lock().unwrap();
        if let Some(re) = cache.get(pattern) {
            return Ok(re.clone());
        }
    }
    let re = Regex::new(pattern)?;
    {
        let mut cache = CACHE.lock().unwrap();
        if cache.len() >= MAX_CACHE {
            cache.clear();
        }
        cache.insert(pattern.to_owned(), re.clone());
    }
    Ok(re)
}
