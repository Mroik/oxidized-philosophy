use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadOverview {
    pub title: String,
    pub url: String,
    pub author: String,
    pub replies: String,
}
impl PartialEq for ThreadOverview {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct XMLAuthor {
    #[serde(rename = "@title")]
    pub name: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct XMLReplies {
    #[serde(rename = "$text")]
    pub replies: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct XMLTitle {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "$text")]
    pub title: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct XMLTitleWrapper {
    #[serde(rename = "a")]
    pub value: XMLTitle,
}

#[derive(Default, Debug)]
pub struct XMLDiscussion {
    pub author: XMLAuthor,
    pub replies: XMLReplies,
    pub title: XMLTitleWrapper,
}

impl<'de> Deserialize<'de> for XMLDiscussion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DiscussionVisitor {})
    }
}

struct DiscussionVisitor {}
impl<'de> Visitor<'de> for DiscussionVisitor {
    type Value = XMLDiscussion;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize discussion")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data = XMLDiscussion {
            ..Default::default()
        };
        let mut counter = 0;

        while let Some(key) = map.next_key::<Option<String>>()? {
            match key.as_ref().unwrap().as_str() {
                "a" => data.author = map.next_value::<XMLAuthor>()?,
                "div" if counter == 0 => {
                    data.replies = map.next_value::<XMLReplies>()?;
                    counter += 1;
                }
                "div" => data.title = map.next_value::<XMLTitleWrapper>()?,
                _ => (),
            }
        }
        return Ok(data);
    }
}
