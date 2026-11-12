use lru::LruCache;
use regex::RegexSet;
use zspell::Dictionary;

use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

const AFF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/dictionaries/dictionaries/en/index.aff"
));
const DICT: &str = concat!(
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/dictionaries/dictionaries/en/index.dic",
    )),
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/custom.dic"))
);

lazy_static::lazy_static! {
    static ref CACHE: Mutex<LruCache<String, Arc<Dictionary>>> =
        LruCache::new(NonZeroUsize::MIN).into();

    static ref ALLOW: RegexSet = RegexSet::new([
        "^[-+~]?(0x)?[[:xdigit:]]+$",
        "^[[:punct:]]+$",
    ]).unwrap();
}

pub(crate) fn dictionary(personal_dictionary: &str) -> Arc<Dictionary> {
    CACHE
        .lock()
        .unwrap()
        .get_or_insert_ref(personal_dictionary, || {
            zspell::builder()
                .config_str(AFF)
                .dict_str(DICT)
                .personal_str(personal_dictionary)
                .build()
                .expect("failed to build dictionary")
                .into()
        })
        .clone()
}

pub(crate) fn is_allowed(word: &str) -> bool {
    ALLOW.is_match(word)
}
