/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod default_annotation;
mod known_modifiers;

use crate::lints::Context;
use crate::LintSettings;

pub use self::default_annotation::SetDefaultAnnotation;
pub use self::known_modifiers::DefaultModifier;

use snafu::Snafu;

use std::fmt::Debug;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    Custom {
        source: Box<dyn std::error::Error + 'static>,
    },
}

impl Error {
    pub fn custom<E>(source: E) -> Self
    where
        E: 'static + std::error::Error,
    {
        Self::Custom {
            source: Box::new(source) as Box<dyn std::error::Error>,
        }
    }
}

pub trait Modifier: Debug {
    fn modify(&self, context: &Context, settings: &mut LintSettings) -> Result<(), Error>;
}
