/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::AnnotationType;

use std::fmt::Debug;

use crate::lints::Context;
use crate::Settings;

use super::{Error, Modifier};

#[derive(Debug)]
pub struct SetDefaultAnnotation<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub annotation_type: AnnotationType,
}

impl<'a> Modifier for SetDefaultAnnotation<'a> {
    fn modify(&self, context: &Context, settings: &mut Settings) -> Result<(), Error> {
        let value = match context.preamble().by_name(self.name) {
            Some(v) => v.value().trim(),
            None => return Ok(()),
        };

        if value == self.value {
            settings.default_annotation_type = self.annotation_type;
        }

        Ok(())
    }
}
