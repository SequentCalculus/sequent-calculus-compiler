use crate::errors::Error;
use fun::{
    parser::parse_module,
    syntax::declarations::{Declaration, Module},
};
use log::info;
use lsp_types::{Location, Position, Range, Uri};

pub struct Document {
    source: String,
    module: Module,
}

impl Document {
    pub fn new() -> Document {
        Document {
            source: "".to_owned(),
            module: Module {
                declarations: vec![],
            },
        }
    }

    pub fn from_text(text: String) -> Result<Document, Error> {
        info!("loading text {text}");
        let parsed = parse_module(&text)?;
        Ok(Document {
            source: text,
            module: parsed,
        })
    }

    pub fn get_ident(&self, pos: Position) -> Result<String, Error> {
        let line = self
            .source
            .lines()
            .nth(pos.line as usize)
            .ok_or(Error::InvalidPosition(pos))?;

        let mut following = line
            .chars()
            .nth(pos.character as usize)
            .ok_or(Error::InvalidPosition(pos))?;
        let mut end_pos = pos.character as usize;
        while following.is_alphanumeric() || following == '_' {
            end_pos += 1;
            if end_pos == line.len() {
                break;
            }
            following = line.chars().nth(end_pos).unwrap();
        }

        let mut prev = line
            .chars()
            .nth(pos.character as usize)
            .ok_or(Error::InvalidPosition(pos))?;
        let mut start_pos = pos.character as usize;
        while prev.is_alphanumeric() || following == '_' {
            start_pos -= 1;
            if start_pos == 0 {
                break;
            }
            prev = line.chars().nth(start_pos).unwrap();
        }
        if start_pos > 0 {
            start_pos += 1
        }

        let ident = &line[start_pos..end_pos];
        Ok(ident.to_owned())
    }

    fn ind_to_pos(&self, index: usize) -> Result<Position, Error> {
        let mut line = 0;
        let mut character = 0;
        for (ind, ch) in self.source.chars().enumerate() {
            if ind == index {
                break;
            }
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
        }

        Ok(Position { line, character })
    }

    pub fn find_ident(&self, ident: String, uri: Uri) -> Result<Location, Error> {
        let (start, end) = self
            .module
            .declarations
            .iter()
            .find_map(|decl| match decl {
                Declaration::Data(data) => (data.name == ident
                    || data.ctors.iter().any(|ctor| ctor.name == ident))
                .then_some((data.span.start().to_usize(), data.span.end().to_usize())),
                Declaration::Codata(cod) => (cod.name == ident
                    || cod.dtors.iter().any(|dtor| dtor.name == ident))
                .then_some((cod.span.start().to_usize(), cod.span.end().to_usize())),
                Declaration::Def(df) => (df.name == ident
                    || df.context.bindings.iter().any(|bnd| bnd.var == ident))
                .then_some((df.span.start().to_usize(), df.span.end().to_usize())),
            })
            .ok_or(Error::UndefinedIdentifier(ident))?;
        let mut start_pos = self.ind_to_pos(start)?;
        start_pos.character += 4;
        let end_pos = self.ind_to_pos(end)?;
        Ok(Location::new(
            uri,
            Range {
                start: start_pos,
                end: end_pos,
            },
        ))
    }
}

impl Default for Document {
    fn default() -> Document {
        Document::new()
    }
}
