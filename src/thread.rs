use std::fmt::Display;

use ratatui::text::Line;
use serde::{de::Visitor, Deserialize};

#[derive(Default, Debug)]
pub struct ThreadData {
    pub title: String,
    pub comments: Vec<ThreadComment>,
    pub comment_page: u16,
}

#[derive(Debug)]
pub struct ThreadComment {
    pub author: String,
    pub text: Vec<Choice>,
    pub date: String,
}

impl ThreadComment {
    pub fn get_lines(&self) -> Vec<Line> {
        let mut v: Vec<String> = self
            .text
            .iter()
            .map(|c| c.to_string())
            .filter(|c| !c.is_empty())
            .fold(vec![String::new()], |mut acc, s| {
                if s == "\n" || acc.last().unwrap() == "\n" {
                    acc.push(s);
                } else if s.contains('\n') {
                    for ns in s.split('\n') {
                        acc.push(ns.to_string());
                        acc.push("\n".to_string());
                    }
                } else {
                    acc.last_mut().unwrap().push_str(s.as_str());
                }
                acc
            });

        // Remove adjacent newlines
        let mut i = 0;
        while i < v.len() - 1 && !(v.len() < v.len() - 1) {
            if v[i] == "\n" && v[i + 1] == "\n" {
                v.remove(i + 1);
            } else {
                i += 1;
            }
        }
        return v.iter().map(|c| Line::raw(c.clone())).collect();
    }
}

#[derive(Default, Debug)]
pub struct XMLComment {
    pub author: XMLAuthor,
    pub text: XMLMessage,
    pub date: XMLMeta,
}

impl<'de> Deserialize<'de> for XMLComment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CommentVisitor {})
    }
}

struct CommentVisitor {}
impl<'de> Visitor<'de> for CommentVisitor {
    type Value = XMLComment;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize comment")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data = XMLComment {
            ..Default::default()
        };
        let mut counter = 0;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "span" => data.author = map.next_value::<XMLAuthor>()?,
                "div" if counter == 0 => {
                    let _ = map.next_value::<()>();
                    counter += 1;
                }
                "div" if counter == 1 => {
                    data.text = map.next_value::<XMLMessage>()?;
                    counter += 1;
                }
                "div" if counter == 2 => data.date = map.next_value::<XMLMeta>()?,
                _ => {
                    let _ = map.next_value::<()>();
                }
            }
        }
        return Ok(data);
    }
}

#[derive(Default, Debug)]
pub struct XMLAuthor {
    pub name: XMLAuthorInner,
}

impl<'de> Deserialize<'de> for XMLAuthor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AuthorVisitor {})
    }
}

struct AuthorVisitor {}
impl<'de> Visitor<'de> for AuthorVisitor {
    type Value = XMLAuthor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize author")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data = XMLAuthor {
            ..Default::default()
        };
        let mut found = false;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "a" if !found => {
                    data.name = map.next_value::<XMLAuthorInner>()?;
                    found = true;
                }
                _ => {
                    let _ = map.next_value::<()>();
                }
            }
        }
        return Ok(data);
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct XMLAuthorInner {
    #[serde(rename = "@title")]
    pub value: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct XMLMessage {
    #[serde(rename = "$value")]
    pub text: Vec<Choice>,
}

// TODO Parse proper types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Choice {
    Sup,
    S,
    Iframe,
    Ul,
    Img,
    U,
    Span {
        #[serde(rename = "$value")]
        data: Option<Vec<Choice>>
    },
    Div {
        #[serde(rename = "$value")]
        data: Vec<Choice>
    },
    A {
        #[serde(rename = "$text")]
        text: Option<String>
    },
    Blockquote {
        #[serde(rename = "div")]
        data: Vec<Choice>
    },
    Br,
    I {
        #[serde(rename = "$value")]
        text: Option<Vec<Choice>>
    },
    B {
        #[serde(rename = "$value")]
        text: Option<Vec<Choice>>
    },
    #[serde(rename = "$text")]
    Other(String),
}
impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ris = match self {
            Self::Br => String::from("\n"),
            Self::Other(t) => t.trim().to_string(),
            Self::A { text: t } => {
                if t.is_some() {
                    t.as_ref().unwrap().trim().to_string()
                } else {
                    String::new()
                }
            },
            Self::Blockquote { data } => {
                match data.first().unwrap() {
                    Self::Div { data: spans } => {
                        let mut text = match spans.get(1).unwrap() {
                            Self::Span { data: ss } => {
                                ss.as_ref().unwrap().iter().fold(String::new(), |mut acc, s| {
                                    match s {
                                        Self::Other(text) => acc.push_str(text),
                                        Self::B { text } => {
                                            for x in text.as_ref().unwrap().iter() {
                                                if let Self::Other(t) = x {
                                                    acc.push(' ');
                                                    acc.push_str(t);
                                                }
                                            }
                                        },
                                        Self::I { text } => {
                                            for x in text.as_ref().unwrap().iter() {
                                                if let Self::Other(t) = x {
                                                    acc.push(' ');
                                                    acc.push_str(t);
                                                }
                                            }
                                        },
                                        _ => (),
                                    }
                                    acc
                                })
                            },
                            _ => unreachable!(),
                        };

                        if spans.len() > 2 {
                            let author = match spans.get(2).unwrap() {
                                Self::Span { data: ss } => {
                                    if ss.as_ref().unwrap().len() < 2 {
                                        String::new()
                                    } else {
                                        match ss.as_ref().unwrap().get(1).unwrap() {
                                            Self::A { text: auth } => {
                                                auth.as_ref().unwrap().clone()
                                            },
                                            _ => unreachable!()
                                        }
                                    }
                                },
                                _ => unreachable!()
                            };

                            text.push_str("\n-");
                            text.push_str(author.as_str());
                        }
                        text
                    },
                    _ => unreachable!(),
                }
            },
            Self::B { text: t } => {
                if let Some(c) = t.as_ref(){
                    c.iter().fold(String::new(), |mut acc, s| {
                        if let Self::Other(text) = s {
                            acc.push(' ');
                            acc.push_str(text);
                        }
                        acc
                    })
                } else {
                    String::new()
                }
            },
            Self::I { text: t } => {
                if let Some(c) = t.as_ref(){
                    c.iter().fold(String::new(), |mut acc, s| {
                        if let Self::Other(text) = s {
                            acc.push(' ');
                            acc.push_str(text);
                        }
                        acc
                    })
                } else {
                    String::new()
                }
            },
            _ => String::new(),
        };
        write!(f, "{}", ris)
    }
}

#[derive(Default, Debug)]
pub struct XMLMeta {
    pub value: XMLDateWrapper,
}

impl<'de> Deserialize<'de> for XMLMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(MetaVisitor {})
    }
}

struct MetaVisitor {}
impl<'de> Visitor<'de> for MetaVisitor {
    type Value = XMLMeta;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize meta")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data = XMLMeta {
            ..Default::default()
        };
        let mut counter = 0;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "span" if counter == 0 => {
                    let _ = map.next_value::<()>();
                    counter += 1;
                }
                "span" if counter == 1 => {
                    data.value = map.next_value::<XMLDateWrapper>()?;
                    counter += 1;
                }
                _ => {
                    let _ = map.next_value::<()>();
                }
            }
        }
        return Ok(data);
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct XMLDateWrapper {
    #[serde(rename = "a")]
    pub value: XMLDateOuter,
}

#[derive(Deserialize, Default, Debug)]
pub struct XMLDateOuter {
    #[serde(rename = "time")]
    pub value: XMLDate,
}

#[derive(Deserialize, Default, Debug)]
pub struct XMLDate {
    #[serde(rename = "@datetime")]
    pub value: String,
}
