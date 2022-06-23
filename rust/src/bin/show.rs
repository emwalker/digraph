use std::env;
use std::io::{self, Write};

use digraph::git::*;
use digraph::prelude::*;

struct Opts {
    filename: String,
}

struct ConsoleOutput<'r> {
    git: &'r mut Git,
    buf: String,
}

impl<'r> Visitor for &mut ConsoleOutput<'r> {
    fn visit_topic(&mut self, topic: &Topic) -> Result<()> {
        let meta = &topic.metadata;
        let s = format! {r#"
Topic: [{}]({})
Parent topics:
"#,
        meta.name(), meta.path};
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
        match &self.git.get(&topic.path)? {
            Object::Topic(topic) => {
                let meta = &topic.metadata;
                let s = format!("  + [{}]({})\n", meta.name(), meta.path);
                self.buf.push_str(&s);
            }
            other => return Err(Error::Repo(format!("expected a topic: {:?}", other))),
        }

        Ok(())
    }

    fn visit_child_topic(&mut self, topic: &Topic) -> Result<()> {
        let meta = &topic.metadata;
        let line = format!("- [{}]({})\n", meta.name(), meta.path);
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
        match &self.git.get(&topic.path)? {
            Object::Topic(topic) => {
                let meta = &topic.metadata;
                let line = format!("- [{}]({})\n", meta.name(), meta.path);
                self.buf.push_str(&line);
            }
            other => return Err(Error::Repo(format!("expected a topic: {:?}", other))),
        }
        Ok(())
    }

    fn visit_topic_child(&mut self, child: &TopicChild) -> Result<()> {
        match &self.git.get(&child.path)? {
            Object::Topic(topic) => {
                self.visit_child_topic(topic)?;
            }
            Object::Link(link) => {
                self.visit_child_link(link)?;
            }
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
    let mut git = Git::new(root_directory);
    let object = git.get(&path)?;

    let mut output = ConsoleOutput {
        git: &mut git,
        buf: String::new(),
    };
    object.accept(&mut output)?;
    io::stdout().write_all(output.as_bytes())?;

    let r = format!("\n{:?}\n", output.git);
    io::stdout().write_all(r.as_bytes())?;

    Ok(())
}
