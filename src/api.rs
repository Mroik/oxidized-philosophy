use std::{error::Error, io::Stdout};

use quick_xml::{de::from_str, events::Event, Reader};
use ratatui::{backend::CrosstermBackend, Terminal};
use reqwest::blocking::Client;

use crate::{
    overview::{ThreadOverview, XMLDiscussion},
    thread::{ThreadComment, ThreadData, XMLComment},
    ui::print_info,
};

pub fn get_threads(
    client: &Client,
    page: u16,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    draw: bool,
) -> Result<Vec<ThreadOverview>, Box<dyn Error>> {
    let url = format!("https://thephilosophyforum.com/discussions/p{}", page);
    if draw {
        print_info(terminal, "Fetching threads overviews...")?;
    }
    let body = client.get(url).send()?.text()?;
    let mut reader = Reader::from_str(body.as_str());
    reader.trim_text(true);

    let mut result: Vec<ThreadOverview> = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => return Err(Box::new(e)),
            Ok(Event::Eof) => break,
            Ok(Event::Start(tag)) => {
                if tag.attributes().map(|a| a.unwrap().value).any(|att| {
                    let blob = att.as_ref();
                    let attribute = std::str::from_utf8(blob).unwrap();
                    attribute == "Item"
                }) {
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
                }
            }
            _ => (),
        }
    }
    return Ok(result);
}

pub fn get_thread(
    client: &Client,
    thread: &ThreadOverview,
    page: u16,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    draw: bool,
) -> Result<ThreadData, Box<dyn Error>> {
    let mut url = thread.url.clone();
    url.push_str(format!("/p{}", page).as_str());
    if draw {
        print_info(terminal, "Fetching thread data...")?;
    }
    let body = client.get(url).send()?.text()?;
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
                if tag.attributes().map(|a| a.unwrap().value).any(|att| {
                    let blob = att.as_ref();
                    let attribute = std::str::from_utf8(blob).unwrap();
                    attribute == "Comment"
                }) {
                    let t_data = reader
                        .read_text(tag.to_end().name())?
                        .replace("&mdash;", "-");
                    let mut comment_text = String::new();
                    comment_text.push_str("<html>");
                    comment_text.push_str(t_data.as_ref());
                    comment_text.push_str("</html>");

                    let comment: XMLComment = from_str(&comment_text)?;
                    let ris = ThreadComment {
                        author: comment.author.name.value,
                        text: comment.text.text,
                        date: comment.date.value.value.value.value,
                    };
                    result.comments.push(ris);
                }
            }
            _ => (),
        }
    }

    return Ok(result);
}
