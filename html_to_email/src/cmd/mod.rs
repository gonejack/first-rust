use std::error::Error;
use std::fs;
use std::path::Path;

use lettre::Message;
use lettre::message::SinglePartBuilder;
use lettre::message::header::{ContentTransferEncoding, ContentType};
use log::warn;
use visdom::types::Elements;
use visdom::Vis;

pub struct HtmlToEmail {
    html: String,
}

impl HtmlToEmail {
    pub fn new(html: String) -> Self {
        Self {
            html,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let output = self.html.trim_end_matches(".html").to_string() + ".eml";

        if Path::new(&output).exists() {
            warn!("target {} exist", output);
            return Ok(());
        }

        let content = fs::read_to_string(self.html.clone())?;
        let doc = Vis::load(content.as_str())?;

        self.clean_doc(&doc);

        let body = SinglePartBuilder::new()
            .header(ContentTransferEncoding::Base64)
            .header(ContentType::TEXT_HTML)
            .body(doc.html());

        let email = Message::builder()
            .from("gonejack@outlook.com".parse().unwrap())
            .to("gonejack@hotmail.com".parse().unwrap())
            .subject(doc.find("title").text())
            .singlepart(body)?;

        fs::write(output, email.formatted())?;

        Ok(())
    }

    pub fn clean_doc(&mut self, doc: &Elements) {
        doc.find("body").find(r#"div:contains("ads from inoreader")"#).remove();
        doc.find(r#"img[src='https://img.solidot.org//0/446/liiLIZF8Uh6yM.jpg']"#).remove();
        doc.find("iframe").remove();
        doc.find("link").remove();
        doc.find("script").remove();
        doc.find("button").remove();
        doc.find("input").remove();
        doc.find("*[contenteditable=true]").remove_attr("contenteditable");
    }
}
