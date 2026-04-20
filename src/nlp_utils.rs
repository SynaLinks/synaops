// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

use std::collections::HashMap;
use std::sync::LazyLock;

static IRREGULAR_PLURALS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("addendum", "addenda"),
        ("aircraft", "aircraft"),
        ("alga", "algae"),
        ("alumna", "alumnae"),
        ("alumnus", "alumni"),
        ("alveolus", "alveoli"),
        ("amoeba", "amoebae"),
        ("analysis", "analyses"),
        ("antenna", "antennae"),
        ("antithesis", "antitheses"),
        ("apex", "apices"),
        ("appendix", "appendices"),
        ("automaton", "automata"),
        ("axis", "axes"),
        ("bacillus", "bacilli"),
        ("bacterium", "bacteria"),
        ("baculum", "bacula"),
        ("barracks", "barracks"),
        ("basis", "bases"),
        ("beau", "beaux"),
        ("bison", "bison"),
        ("buffalo", "buffalo"),
        ("bureau", "bureaus"),
        ("cactus", "cacti"),
        ("calf", "calves"),
        ("carcinoma", "carcinomata"),
        ("carp", "carp"),
        ("census", "censuses"),
        ("chassis", "chassis"),
        ("cherub", "cherubim"),
        ("child", "children"),
        ("château", "châteaus"),
        ("cloaca", "cloacae"),
        ("cod", "cod"),
        ("codex", "codices"),
        ("concerto", "concerti"),
        ("consortium", "consortia"),
        ("corpus", "corpora"),
        ("crisis", "crises"),
        ("criterion", "criteria"),
        ("curriculum", "curricula"),
        ("cystoma", "cystomata"),
        ("datum", "data"),
        ("deer", "deer"),
        ("diagnosis", "diagnoses"),
        ("die", "dice"),
        ("dwarf", "dwarfs"),
        ("echo", "echoes"),
        ("elf", "elves"),
        ("elk", "elk"),
        ("ellipsis", "ellipses"),
        ("embargo", "embargoes"),
        ("emphasis", "emphases"),
        ("erratum", "errata"),
        ("faux pas", "faux pas"),
        ("fez", "fezes"),
        ("firmware", "firmware"),
        ("fish", "fish"),
        ("focus", "foci"),
        ("foot", "feet"),
        ("formula", "formulae"),
        ("fungus", "fungi"),
        ("gallows", "gallows"),
        ("genus", "genera"),
        ("glomerulus", "glomeruli"),
        ("goose", "geese"),
        ("graffito", "graffiti"),
        ("grouse", "grouse"),
        ("half", "halves"),
        ("hamulus", "hamuli"),
        ("hero", "heroes"),
        ("hippopotamus", "hippopotami"),
        ("hoof", "hooves"),
        ("hovercraft", "hovercraft"),
        ("hypothesis", "hypotheses"),
        ("iliac", "ilia"),
        ("incubus", "incubi"),
        ("index", "indices"),
        ("interstitium", "interstitia"),
        ("kakapo", "kakapo"),
        ("knife", "knives"),
        ("larva", "larvae"),
        ("leaf", "leaves"),
        ("libretto", "libretti"),
        ("life", "lives"),
        ("loaf", "loaves"),
        ("loculus", "loculi"),
        ("locus", "loci"),
        ("louse", "lice"),
        ("man", "men"),
        ("matrix", "matrices"),
        ("means", "means"),
        ("measles", "measles"),
        ("media", "media"),
        ("medium", "media"),
        ("memorandum", "memoranda"),
        ("millennium", "millennia"),
        ("minutia", "minutiae"),
        ("moose", "moose"),
        ("mouse", "mice"),
        ("nebula", "nebulae"),
        ("nemesis", "nemeses"),
        ("neurosis", "neuroses"),
        ("news", "news"),
        ("nucleolus", "nucleoli"),
        ("nucleus", "nuclei"),
        ("oasis", "oases"),
        ("occiput", "occipita"),
        ("offspring", "offspring"),
        ("omphalos", "omphaloi"),
        ("opus", "opera"),
        ("ovum", "ova"),
        ("ox", "oxen"),
        ("paralysis", "paralyses"),
        ("parenthesis", "parentheses"),
        ("person", "people"),
        ("phenomenon", "phenomena"),
        ("phylum", "phyla"),
        ("pike", "pike"),
        ("polyhedron", "polyhedra"),
        ("potato", "potatoes"),
        ("primus", "primi"),
        ("prognosis", "prognoses"),
        ("quiz", "quizzes"),
        ("radius", "radii"),
        ("referendum", "referenda"),
        ("salmon", "salmon"),
        ("scarf", "scarves"),
        ("scrotum", "scrota"),
        ("self", "selves"),
        ("seminoma", "seminomata"),
        ("series", "series"),
        ("sheep", "sheep"),
        ("shelf", "shelves"),
        ("shrimp", "shrimp"),
        ("simulacrum", "simulacra"),
        ("soliloquy", "soliloquies"),
        ("spacecraft", "spacecraft"),
        ("species", "species"),
        ("spectrum", "spectra"),
        ("squid", "squid"),
        ("stimulus", "stimuli"),
        ("stratum", "strata"),
        ("swine", "swine"),
        ("syconium", "syconia"),
        ("syllabus", "syllabi"),
        ("symposium", "symposia"),
        ("synopsis", "synopses"),
        ("synthesis", "syntheses"),
        ("tableau", "tableaus"),
        ("testis", "testes"),
        ("that", "those"),
        ("thesis", "theses"),
        ("thief", "thieves"),
        ("this", "these"),
        ("thrombus", "thrombi"),
        ("tomato", "tomatoes"),
        ("tooth", "teeth"),
        ("torus", "tori"),
        ("trout", "trout"),
        ("tuna", "tuna"),
        ("umbilicus", "umbilici"),
        ("uterus", "uteri"),
        ("vertebra", "vertebrae"),
        ("vertex", "vertices"),
        ("veto", "vetoes"),
        ("vita", "vitae"),
        ("vortex", "vortices"),
        ("watercraft", "watercraft"),
        ("wharf", "wharves"),
        ("wife", "wives"),
        ("wolf", "wolves"),
        ("woman", "women"),
    ])
});

static IRREGULAR_SINGULARS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    IRREGULAR_PLURALS
        .iter()
        .map(|(&singular, &plural)| (plural, singular))
        .collect()
});

/// Convert a singular word to its plural form.
pub fn to_plural(word: &str) -> String {
    if let Some(&plural) = IRREGULAR_PLURALS.get(word) {
        return plural.to_owned();
    }
    if word.len() >= 2 && word.ends_with('y') {
        let before_y = word.as_bytes()[word.len() - 2];
        if !b"aeiou".contains(&before_y) {
            let stem = &word[..word.len() - 1];
            return format!("{stem}ies");
        }
    }
    if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("sh")
        || word.ends_with("ch")
    {
        return format!("{word}es");
    }
    format!("{word}s")
}

/// Convert a plural word to its singular form.
pub fn to_singular(word: &str) -> String {
    if let Some(&singular) = IRREGULAR_SINGULARS.get(word) {
        return singular.to_owned();
    }
    if word.ends_with("ies") && word.len() > 3 {
        let stem = &word[..word.len() - 3];
        return format!("{stem}y");
    }
    if word.ends_with("es") && word.len() > 2 {
        let stem = &word[..word.len() - 2];
        if stem.ends_with('s')
            || stem.ends_with('x')
            || stem.ends_with('z')
            || stem.ends_with("sh")
            || stem.ends_with("ch")
        {
            return stem.to_owned();
        }
        return word[..word.len() - 1].to_owned();
    }
    if word.ends_with('s') && !word.ends_with("ss") && word.len() > 1 {
        return word[..word.len() - 1].to_owned();
    }
    word.to_owned()
}

/// Convert the last word of a property key to its plural form.
pub fn to_plural_property(property_key: &str) -> String {
    let mut words = property_key.split('_').collect::<Vec<_>>();
    let last = words.last().unwrap();
    let pluralised = to_plural(last);
    let len = words.len();
    words[len - 1] = &pluralised;
    words.join("_")
}

/// Convert the last word of a property key to its singular form.
pub fn to_singular_property(property_key: &str) -> String {
    let mut words = property_key.split('_').collect::<Vec<_>>();
    let last = words.last().unwrap();
    let singularised = to_singular(last);
    let len = words.len();
    words[len - 1] = &singularised;
    words.join("_")
}

/// Remove the trailing numerical suffix from a property key (e.g. "answer_1" -> "answer").
pub fn remove_numerical_suffix(property_key: &str) -> &str {
    if let Some(pos) = property_key.rfind('_') {
        let suffix = &property_key[pos + 1..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return &property_key[..pos];
        }
    }
    property_key
}

/// Add a numerical suffix to a property key.
pub fn add_suffix(property_key: &str, suffix: usize) -> String {
    format!("{property_key}_{suffix}")
}

/// Convert a property key to its base (singular) form by removing
/// the numerical suffix and converting to singular.
pub fn to_singular_without_numerical_suffix(property_key: &str) -> String {
    let base = remove_numerical_suffix(property_key);
    to_singular_property(base)
}

/// Convert a property key to its list (plural) form by removing
/// the numerical suffix and converting to plural.
pub fn to_plural_without_numerical_suffix(property_key: &str) -> String {
    let base = remove_numerical_suffix(property_key);
    to_plural_property(base)
}

/// Check if the last word of a property key is in plural form.
pub fn is_plural(property_key: &str) -> bool {
    let words = property_key.split('_').collect::<Vec<_>>();
    let noun = if words.len() > 1 {
        words[words.len() - 1]
    } else {
        words[0]
    };
    let singular_form = to_singular(noun);
    singular_form != noun
}

#[cfg(test)]
#[path = "nlp_utils_test.rs"]
mod tests;
