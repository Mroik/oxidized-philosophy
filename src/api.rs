use std::error::Error;

use quick_xml::{Reader, events::Event, de::from_str};

use crate::{overview::{ThreadOverview, XMLDiscussion}, thread::{ThreadData, XMLComment, ThreadComment}};

pub fn get_threads(page: u16) -> Result<Vec<ThreadOverview>, Box<dyn Error>> {
    let url = format!("https://thephilosophyforum.com/discussions/p{}", page);
    let body = reqwest::blocking::get(url)?.text()?;
    let mut reader = Reader::from_str(body.as_str());
    reader.trim_text(true);

    let mut result: Vec<ThreadOverview> = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,
            Ok(Event::Start(tag)) => {
                match tag
                    .attributes()
                    .map(|a| a.unwrap().value)
                    .find(|att| {
                        let blob = att.as_ref();
                        let attribute = std::str::from_utf8(blob).unwrap();
                        return attribute == "Item";
                    }) {
                    None => (),
                    Some(_) => {
                        let vv = reader.read_text(tag.to_end().name())?;
                        let mut thread_text = String::new();
                        thread_text.push_str("<html>");
                        thread_text.push_str(vv.as_ref());
                        thread_text.push_str("</html>");

                        let mut url = String::from("https://thephilosophyforum.com/");
                        let discussion: XMLDiscussion = from_str(&thread_text).unwrap();
                        url.push_str(discussion.title.value.href.as_str());

                        result.push(ThreadOverview {
                            title: discussion.title.value.title,
                            url,
                            author: discussion.author.name,
                            replies: discussion.replies.replies,
                        });
                    },
                }
            }
            _ => (),
        }
    }
    return Ok(result);
}

pub fn get_thread(thread: &ThreadOverview) -> Result<ThreadData, Box<dyn Error>> {
    let body = reqwest::blocking::get(&thread.url)?.text()?;
    let mut result = ThreadData {
        title: thread.title.clone(),
        ..Default::default()
    };
    let mut reader = Reader::from_str(body.as_str());
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,
            Ok(Event::Start(tag)) => {
                match tag
                    .attributes()
                    .map(|a| a.unwrap().value)
                    .find(|att| {
                        let blob = att.as_ref();
                        let attribute = std::str::from_utf8(blob).unwrap();
                        return attribute == "Comment";
                    }) {
                    None => (),
                    Some(_) => {
                        let t_data = reader.read_text(tag.to_end().name())?;
                        let mut comment_text = String::new();
                        comment_text.push_str("<html>");
                        comment_text.push_str(t_data.as_ref());
                        comment_text.push_str("</html");

                        let comment: XMLComment = from_str(&comment_text).unwrap();
                        let ris = ThreadComment {
                            author: comment.author.name.value,
                            text: comment.text.text,
                            date: comment.date.value.value.value.value,
                        };
                        result.comments.push(ris);
                    }
                }
            },
            _ => (),
        }
    }

    return Ok(result);
}
