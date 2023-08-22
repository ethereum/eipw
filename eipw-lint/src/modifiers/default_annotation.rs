/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::AnnotationType;

use std::fmt::Debug;

use crate::lints::Context;
use crate::LintSettings;

use serde::{Deserialize, Serialize};

use super::{Error, Modifier};

#[derive(Serialize, Deserialize)]
#[serde(remote = "AnnotationType", rename_all = "kebab-case")]
enum AnnotationTypeDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SetDefaultAnnotation<S> {
    pub name: S,
    pub value: S,

    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
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
            settings.default_annotation_type = self.annotation_type;
        }

        Ok(())
    }
}
