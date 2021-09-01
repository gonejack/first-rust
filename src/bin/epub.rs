use std::fs::{File, OpenOptions, read_to_string};
use std::io;
use std::io::Write;

use epub_builder::{EpubBuilder, Error, ResultExt};
use epub_builder::EpubContent;
use epub_builder::ErrorKind::Msg;
use epub_builder::ReferenceType;
use epub_builder::Result;
use epub_builder::TocElement;
use epub_builder::ZipLibrary;

use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    trace!("this is trace");
    debug!("this is debug");
    info!("this is info");
    warn!("this is warning");
    error!("this is error");

    let result = run();

    if result.is_ok() {
        println!("{:?}", result.unwrap());
    } else {
        error!("{}", result.unwrap_err());
    }
}


fn run() -> Result<()> {
    let dummy_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops">
<body>
<p>Text of the page</p>
</body>
</html>"#;
    let dummy_image = "Not really a PNG image";
    let dummy_css = "body { background-color: pink }";

    let mut w = OpenOptions::new();
    {
        w.write(true);
        w.create_new(true);
        w.open("test.epub");
        w.chain_err(|| format!("cannot create {}", "test.epub"))?;
    }

    let mut e = EpubBuilder::new(ZipLibrary::new()?)?;
    {
        e.metadata("author", "Joan Doe")?;
        e.metadata("title", "Dummy Book")?;
        e.stylesheet(dummy_css.as_bytes())?;
        e.add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?;
        e.add_resource("some_image.png", dummy_image.as_bytes(), "image/png")?;
        e.add_content(EpubContent::new("cover.xhtml", dummy_content.as_bytes()).title("Cover").reftype(ReferenceType::Cover))?;
        e.add_content(EpubContent::new("title.xhtml", dummy_content.as_bytes()).title("Title").reftype(ReferenceType::TitlePage))?;
        e.add_content(EpubContent::new("chapter_1.xhtml", dummy_content.as_bytes()).title("Chapter 1").reftype(ReferenceType::Text))?;
        e.add_content(EpubContent::new("chapter_2.xhtml", dummy_content.as_bytes()).title("Chapter 2").child(TocElement::new("chapter_2.xhtml#1", "Chapter 2, section 1")))?;
        e.add_content(EpubContent::new("section.xhtml", dummy_content.as_bytes()).title("Chapter 2, section 2").level(2))?;
        e.add_content(EpubContent::new("notes.xhtml", dummy_content.as_bytes()))?;
        e.inline_toc();
        e.generate(&mut w)?;
    }
    w.sync_all();

    Ok(())
}

