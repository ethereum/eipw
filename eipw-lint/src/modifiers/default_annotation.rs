/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use std::fmt::Debug;

use crate::lints::Context;
use crate::LintSettings;

use serde::{Deserialize, Serialize};

use super::{Error, Modifier};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[serde(remote = "Level", rename_all = "kebab-case")]
enum LevelDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct SetDefaultAnnotation<S> {
    pub name: S,
    pub value: S,

    #[serde(with = "LevelDef")]
    pub annotation_level: Level,
}

impl<S> Modifier for SetDefaultAnnotation<S>
where
    S: Debug + AsRef<str>,
{
    fn modify(&self, context: &Context, settings: &mut LintSettings) -> Result<(), Error> {
        let value = match context.preamble().by_name(self.name.as_ref()) {
            Some(v) => v.value().trim(),
            None => return Ok(()),
        };

        if value == self.value.as_ref() {
            settings.default_annotation_level = self.annotation_level;
        }

        Ok(())
    }
}
