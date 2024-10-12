// Borrowed from https://github.com/rust-lang/annotate-snippets-rs
// At commit c84e388949f0e06f4529de060c195e12e6531fbd

pub use annotate_snippets;

use annotate_snippets as ann;

use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::ops::Range;

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Message<'a> {
    pub level: Level,
    pub id: Option<Cow<'a, str>>,
    pub title: Cow<'a, str>,
    pub snippets: Vec<Snippet<'a>>,
    pub footer: Vec<Message<'a>>,
}

impl<'a> Message<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(Cow::Borrowed(id));
        self
    }

    pub fn snippet(mut self, slice: Snippet<'a>) -> Self {
        self.snippets.push(slice);
        self
    }

    pub fn snippets(mut self, slice: impl IntoIterator<Item = Snippet<'a>>) -> Self {
        self.snippets.extend(slice);
        self
    }

    pub fn footer(mut self, footer: Message<'a>) -> Self {
        self.footer.push(footer);
        self
    }

    pub fn footers(mut self, footer: impl IntoIterator<Item = Message<'a>>) -> Self {
        self.footer.extend(footer);
        self
    }
}

impl<'a, 'b> From<&'b Message<'a>> for ann::Message<'b> {
    fn from(value: &'b Message<'a>) -> Self {
        let msg = ann::Level::from(value.level)
            .title(&value.title)
            .snippets(value.snippets.iter().map(Into::into))
            .footers(value.footer.iter().map(Into::into));

        if let Some(ref id) = value.id {
            msg.id(id)
        } else {
            msg
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Snippet<'a> {
    pub origin: Option<Cow<'a, str>>,
    pub line_start: usize,

    pub source: Cow<'a, str>,
    pub annotations: Vec<Annotation<'a>>,

    pub fold: bool,
}

impl<'a> Snippet<'a> {
    pub fn source(source: &'a str) -> Self {
        Self {
            origin: None,
            line_start: 1,
            source: Cow::Borrowed(source),
            annotations: vec![],
            fold: false,
        }
    }

    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = line_start;
        self
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(Cow::Borrowed(origin));
        self
    }

    pub fn annotation(mut self, annotation: Annotation<'a>) -> Self {
        self.annotations.push(annotation);
        self
    }

    pub fn annotations(mut self, annotation: impl IntoIterator<Item = Annotation<'a>>) -> Self {
        self.annotations.extend(annotation);
        self
    }

    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

impl<'a, 'b> From<&'b Snippet<'a>> for ann::Snippet<'b> {
    fn from(value: &'b Snippet<'a>) -> Self {
        let snip = Self::source(&value.source)
            .line_start(value.line_start)
            .annotations(value.annotations.iter().map(Into::into))
            .fold(value.fold);

        if let Some(ref origin) = value.origin {
            snip.origin(origin)
        } else {
            snip
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Annotation<'a> {
    pub range: Range<usize>,
    pub label: Option<Cow<'a, str>>,
    pub level: Level,
}

impl<'a> Annotation<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(Cow::Borrowed(label));
        self
    }
}

impl<'a, 'b> From<&'b Annotation<'a>> for ann::Annotation<'b> {
    fn from(value: &'b Annotation<'a>) -> Self {
        let a = ann::Level::from(value.level).span(value.range.clone());

        if let Some(ref label) = value.label {
            a.label(label)
        } else {
            a
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Level {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl Level {
    pub fn title(self, title: &str) -> Message<'_> {
        Message {
            level: self,
            id: None,
            title: Cow::Borrowed(title),
            snippets: vec![],
            footer: vec![],
        }
    }

    pub fn span<'a>(self, span: Range<usize>) -> Annotation<'a> {
        Annotation {
            range: span,
            label: None,
            level: self,
        }
    }
}

impl From<Level> for ann::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Error,
            Level::Warning => Self::Warning,
            Level::Info => Self::Info,
            Level::Note => Self::Note,
            Level::Help => Self::Help,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip_with_escape_sequence() {
        let title = "some \u{1f6a8} diagn\\ostic \"a rope of sand\"";
        let id = "id-\u{1f6a8}-\"-\\";
        let source = "for \u{1f6a8} do \\ostic \"electric boogaloo\"";
        let origin = "\u{1f6a8}";

        let annotation = Level::Help.span(0..4).label(title);

        let snippet = Snippet::source(source)
            .fold(true)
            .origin(origin)
            .line_start(123)
            .annotation(annotation);

        let footer = Level::Help.title(title).id(id);

        let msg = Level::Error
            .title(title)
            .id(id)
            .snippet(snippet)
            .footer(footer);

        let json = serde_json::to_string_pretty(&msg).unwrap();
        let actual: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(actual.title, title);
        assert_eq!(actual.id, Some(Cow::Borrowed(id)));
        assert_eq!(actual.level, Level::Error);
        assert_eq!(actual.footer.len(), 1);
        assert_eq!(actual.footer[0].title, title);
        assert_eq!(actual.footer[0].id, Some(Cow::Borrowed(id)));
        assert_eq!(actual.snippets.len(), 1);
        assert_eq!(actual.snippets[0].source, source);
        assert_eq!(actual.snippets[0].origin, Some(Cow::Borrowed(origin)));
        assert_eq!(actual.snippets[0].annotations.len(), 1);
        assert_eq!(
            actual.snippets[0].annotations[0].label,
            Some(Cow::Borrowed(title))
        );
    }
}
