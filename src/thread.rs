use serde::{Deserialize, de::Visitor};

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
    pub fn get_text(&self) -> String {
        self.text
            .iter()
            .map(|c| c.to_string())
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x.as_str());
                acc
            })
    }
}

#[derive(Default, Debug)]
pub struct XMLComment {
    pub author: XMLAuthor,
    pub text: XMLMessage,
    pub date: XMLMeta,
}

impl<'de> Deserialize<'de> for XMLComment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_map(CommentVisitor {})
    }
}

struct CommentVisitor {}
impl<'de> Visitor<'de> for CommentVisitor {
    type Value = XMLComment;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize comment")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de>, {
        let mut data = XMLComment { ..Default::default() };
        let mut counter = 0;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "span" => data.author = map.next_value::<XMLAuthor>()?,
                "div" if counter == 0 => {
                    let _ = map.next_value::<()>();
                    counter += 1;
                },
                "div" if counter == 1 => {
                    data.text = map.next_value::<XMLMessage>()?;
                    counter += 1;
                },
                "div" if counter == 2 => data.date = map.next_value::<XMLMeta>()?,
                _ => {
                    let _ = map.next_value::<()>();
                },
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_map(AuthorVisitor {})
    }
}

struct AuthorVisitor {}
impl<'de> Visitor<'de> for AuthorVisitor {
    type Value = XMLAuthor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize author")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de>, {
        let mut data = XMLAuthor { ..Default::default() };
        let mut found = false;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "a" if !found => {
                    data.name = map.next_value::<XMLAuthorInner>()?;
                    found = true;
                },
                _ => {
                    let _ = map.next_value::<()>();
                },
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
    //#[serde(rename = "$value")]
    //pub text: Vec<String>,
    //#[serde(rename = "blockquote")]
    //pub quotes: Vec<()>,
    //br: Vec<()>,
    //i: Vec<()>,
    //b: Vec<()>,
    #[serde(rename = "$value")]
    pub text: Vec<Choice>,
}

// TODO Parse proper types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Choice {
    Iframe,
    Ul,
    Img,
    U,
    Span,
    Div,
    A,
    Blockquote,
    Br,
    I,
    B,
    #[serde(rename = "$text")]
    Other(String),
}
impl Choice {
    fn to_string(&self) -> String {
        match self {
            Self::Other(t) => t.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Default, Debug)]
pub struct XMLMeta {
    pub value: XMLDateWrapper,
}

impl<'de> Deserialize<'de> for XMLMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_map(MetaVisitor {})
    }
}

struct MetaVisitor {}
impl<'de> Visitor<'de> for MetaVisitor {
    type Value = XMLMeta;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Couldn't deserialize meta")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de>, {
        let mut data = XMLMeta { ..Default::default() };
        let mut counter = 0;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "span" if counter == 0 => {
                    let _ = map.next_value::<()>();
                    counter += 1;
                },
                "span" if counter == 1 => {
                    data.value = map.next_value::<XMLDateWrapper>()?;
                    counter += 1;
                },
                _ => {
                    let _ = map.next_value::<()>();
                },
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
