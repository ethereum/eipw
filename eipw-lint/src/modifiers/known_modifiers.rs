/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::lints::Context;
use crate::LintSettings;

use serde::{Deserialize, Serialize};

use std::fmt::Debug;

use super::{default_annotation, Modifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum DefaultModifier<S> {
    SetDefaultAnnotation(default_annotation::SetDefaultAnnotation<S>),
}

impl<S> Modifier for DefaultModifier<S>
where
    S: Debug + AsRef<str>,
{
    fn modify(&self, context: &Context, settings: &mut LintSettings) -> Result<(), super::Error> {
        match self {
            Self::SetDefaultAnnotation(a) => a.modify(context, settings),
        }
    }
}
