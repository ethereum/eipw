/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use crate::{DefaultLint, DefaultModifier};

use olpc_cjson::CanonicalFormatter;
use serde::Serialize;
use sha3::{Digest, Sha3_256};

type Options<S = String> = crate::Options<Vec<DefaultModifier<S>>, HashMap<S, DefaultLint<S>>>;

fn replace_floats(value: &mut serde_json::Value) {
    use serde_json::Value::*;

    let number = match value {
        Null | Bool(_) | String(_) => return,
        Number(n) => n,
        Array(a) => {
            for v in a.iter_mut() {
                replace_floats(v);
            }
            return;
        }
        Object(o) => {
            for (_, v) in o.iter_mut() {
                replace_floats(v);
            }
            return;
        }
    };

    if let Some(f) = number.as_f64() {
        *number = serde_json::Number::from(f as u64);
    }
}

pub fn schema_version() -> String {
    let schema = schemars::schema_for!(Options);
    let mut value = serde_json::to_value(&schema).unwrap();
    replace_floats(&mut value);

    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, CanonicalFormatter::new());
    value.serialize(&mut ser).unwrap();

    let mut hasher = Sha3_256::new();
    hasher.update(&buf);
    let result = hasher.finalize();

    format!("{}+{:x}", env!("CARGO_PKG_VERSION"), result)
}
