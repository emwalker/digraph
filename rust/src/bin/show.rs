use std::env;
use std::io::{self, Write};

use digraph::git::*;
use digraph::prelude::*;
use digraph::types::Timespec;

struct Opts {
    filename: String,
}

struct ConsoleOutput<'r> {
    git: &'r mut Client,
    buf: String,
}

impl<'r> Visitor for &mut ConsoleOutput<'r> {
    fn visit_topic(&mut self, topic: &Topic) -> Result<()> {
        let meta = &topic.metadata;
        let s = format! {r#"
Topic: [{}]({})
Parent topics:
"#,
        meta.name(Locale::EN), meta.path};
        self.buf.push_str(&s);

        for topic in &topic.parent_topics {
            self.visit_parent_topic(topic)?;
        }

        self.buf.push_str("Children:\n");

        for child in &topic.children {
            self.visit_topic_child(child)?;
        }

        Ok(())
    }

    fn visit_link(&mut self, link: &Link) -> Result<()> {
        let meta = &link.metadata;
        let s = format! {r#"
Link: [{}]({})
Parent topics:
"#,
        meta.title, meta.url};
        self.buf.push_str(&s);

        for topic in &link.parent_topics {
            self.visit_parent_topic(topic)?;
        }

        Ok(())
    }
}

impl<'r> ConsoleOutput<'r> {
    fn as_bytes(&self) -> &[u8] {
        self.buf.as_bytes()
    }

    fn visit_child_parent_topic(&mut self, topic: &ParentTopic) -> Result<()> {
        match &self.git.fetch(&PathSpec::try_from(&topic.path)?) {
            Some(Object::Topic(topic)) => {
                let meta = &topic.metadata;
                let s = format!("  + [{}]({})\n", topic.name(Locale::EN), meta.path);
                self.buf.push_str(&s);
            }
            other => return Err(Error::Repo(format!("expected a topic: {:?}", other))),
        }

        Ok(())
    }

    fn visit_child_topic(&mut self, topic: &Topic) -> Result<()> {
        let line = format!("- [{}]({})\n", topic.name(Locale::EN), topic.path()?);
        self.buf.push_str(&line);

        for topic in &topic.parent_topics {
            self.visit_child_parent_topic(topic)?;
        }

        Ok(())
    }

    fn visit_child_link(&mut self, link: &Link) -> Result<()> {
        let meta = &link.metadata;
        let line = format!("- [{}]({})\n", meta.title, meta.url);
        self.buf.push_str(&line);

        for topic in &link.parent_topics {
            self.visit_child_parent_topic(topic)?;
        }

        Ok(())
    }

    fn visit_parent_topic(&mut self, topic: &ParentTopic) -> Result<()> {
        match &self.git.fetch(&PathSpec::try_from(&topic.path)?) {
            Some(Object::Topic(topic)) => {
                let line = format!("- [{}]({})\n", topic.name(Locale::EN), topic.path()?);
                self.buf.push_str(&line);
            }
            other => return Err(Error::Repo(format!("expected a topic: {:?}", other))),
        }
        Ok(())
    }

    fn visit_topic_child(&mut self, child: &TopicChild) -> Result<()> {
        let path = PathSpec::try_from(&child.path)?;
        match &self.git.fetch(&path) {
            Some(Object::Topic(topic)) => {
                self.visit_child_topic(topic)?;
            }

            Some(Object::Link(link)) => {
                self.visit_child_link(link)?;
            }

            None => {}
        }
        Ok(())
    }
}

fn parse_args() -> Opts {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("a file is required").to_owned();

    Opts { filename }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let opts = parse_args();
    let (root_directory, path) = parse_path(&opts.filename)?;
    let mut git = Client::new(&Viewer::service_account(), &root_directory, Timespec);
    let object = git.fetch(&path);
    if object.is_none() {
        return Err(Error::NotFound(format!(
            "{} does not contain {}",
            root_directory, path
        )));
    }
    let object = object.unwrap();

    let mut output = ConsoleOutput {
        git: &mut git,
        buf: String::new(),
    };
    object.accept(&mut output)?;
    io::stdout().write_all(output.as_bytes())?;

    Ok(())
}
