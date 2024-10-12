// Borrowed from https://github.com/rust-lang/annotate-snippets-rs
// At commit c84e388949f0e06f4529de060c195e12e6531fbd

pub use annotate_snippets;

use annotate_snippets as ann;
use std::ops::Range;

#[derive(Debug)]
pub struct Message<'a> {
    pub level: Level,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub snippets: Vec<Snippet<'a>>,
    pub footer: Vec<Message<'a>>,
}

impl<'a> Message<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
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

impl<'a> From<Message<'a>> for ann::Message<'a> {
    fn from(value: Message<'a>) -> Self {
        let msg = ann::Level::from(value.level)
            .title(value.title)
            .snippets(value.snippets.into_iter().map(Into::into))
            .footers(value.footer.into_iter().map(Into::into));

        if let Some(id) = value.id {
            msg.id(id)
        } else {
            msg
        }
    }
}

#[derive(Debug)]
pub struct Snippet<'a> {
    pub origin: Option<&'a str>,
    pub line_start: usize,

    pub source: &'a str,
    pub annotations: Vec<Annotation<'a>>,

    pub fold: bool,
}

impl<'a> Snippet<'a> {
    pub fn source(source: &'a str) -> Self {
        Self {
            origin: None,
            line_start: 1,
            source,
            annotations: vec![],
            fold: false,
        }
    }

    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = line_start;
        self
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
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

impl<'a> From<Snippet<'a>> for ann::Snippet<'a> {
    fn from(value: Snippet<'a>) -> Self {
        let snip = Self::source(value.source)
            .line_start(value.line_start)
            .annotations(value.annotations.into_iter().map(Into::into))
            .fold(value.fold);

        if let Some(origin) = value.origin {
            snip.origin(origin)
        } else {
            snip
        }
    }
}

#[derive(Debug)]
pub struct Annotation<'a> {
    pub range: Range<usize>,
    pub label: Option<&'a str>,
    pub level: Level,
}

impl<'a> Annotation<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl<'a> From<Annotation<'a>> for ann::Annotation<'a> {
    fn from(value: Annotation<'a>) -> Self {
        let a = ann::Level::from(value.level).span(value.range);

        if let Some(label) = value.label {
            a.label(label)
        } else {
            a
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            title,
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
