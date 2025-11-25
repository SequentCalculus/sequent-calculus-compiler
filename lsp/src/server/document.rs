use std::os::windows::raw::SOCKET;

use crate::errors::Error;
use fun::{parser::parse_module, syntax::program::CheckedProgram};
use log::info;
use lsp_types::{Location, ParameterInformationSettings, Position, Range, Uri};
//use serde_json::to_string;
use printer::Print;

pub struct Document {
    source: String,
    module: CheckedProgram,
}

impl Document {
    pub fn new() -> Document {
        Document {
            source: "".to_owned(),
            module: CheckedProgram {
                defs: vec![],
                data_types: vec![],
                codata_types: vec![],
            },
        }
    }

    pub fn from_text(text: String) -> Result<Document, Error> {
        info!("loading text {text}");
        let parsed = parse_module(&text)?;
        let checked = parsed.check()?;
        Ok(Document {
            source: text,
            module: checked,
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

    pub fn ind_to_pos(&self, index: usize) -> Position {
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

        Position { line, character }
    }

    fn find_def(&self, ident: &str) -> Option<(Position, Position)> {
        let span = self
            .module
            .defs
            .iter()
            .find_map(|df| (df.name == ident).then_some(df.span))?;
        let mut start = self.ind_to_pos(span.start().to_usize());
        //"def "
        start.character += 4;
        let end = self.ind_to_pos(span.end().to_usize());
        Some((start, end))
    }

    fn find_data(&self, ident: &str) -> Option<(Position, Position)> {
        let span = self.module.data_types.iter().find_map(|data| {
            (data.name == ident || data.ctors.iter().any(|ctor| ctor.name == ident))
                .then_some(data.span)
        })?;
        let mut start = self.ind_to_pos(span.start().to_usize());
        // "data "
        start.character += 5;
        let end = self.ind_to_pos(span.end().to_usize());
        Some((start, end))
    }

    fn find_codata(&self, ident: &str) -> Option<(Position, Position)> {
        let span = self.module.codata_types.iter().find_map(|cod| {
            (cod.name == ident || cod.dtors.iter().any(|dtor| dtor.name == ident))
                .then_some(cod.span)
        })?;
        let mut start = self.ind_to_pos(span.start().to_usize());
        // "codata "
        start.character += 7;
        let end = self.ind_to_pos(span.end().to_usize());
        Some((start, end))
    }

    pub fn find_ident(&self, ident: String, uri: Uri) -> Result<Location, Error> {
        let (start, end) = self
            .find_def(&ident)
            .or_else(|| self.find_data(&ident).or_else(|| self.find_codata(&ident)))
            .ok_or(Error::UndefinedIdentifier(ident))?;
        Ok(Location::new(uri, Range { start, end }))
    }

    //eigener Code
    pub fn find_implementation(&self, ident: String, uri: Uri) -> Result<Location, Error> {
        //Suchen von Funktionsimplementierungen die mit ident übereinstimmen
        if let Some(def) = self.module.defs.iter().find(|def| def.name == ident) {
            let start = self.ind_to_pos(def.span.start().to_usize());
            let end = self.ind_to_pos(def.span.end().to_usize());
            return Ok(Location::new(uri, Range { start, end }));
        }

        //Suchen von Konstruktor die mit ident übereinstimmen
        for data in &self.module.data_types {
            if let Some(ctor) = data.ctors.iter().find(|ctor| ctor.name == ident) {
                let start = self.ind_to_pos(ctor.span.start().to_usize());
                let end = self.ind_to_pos(ctor.span.end().to_usize());
                return Ok(Location::new(uri.clone(), Range { start, end }));
            }
        }

        //Suchen von Destruktoren die mit ident übereinstimmen
        for cod in &self.module.codata_types {
            if let Some(dtor) = cod.dtors.iter().find(|dtor| dtor.name == ident) {
                let start = self.ind_to_pos(dtor.span.start().to_usize());
                let end = self.ind_to_pos(dtor.span.end().to_usize());
                return Ok(Location::new(uri.clone(), Range { start, end }));
            }
        }

        //Fehler abfangen
        Err(Error::UndefinedIdentifier(ident))
    }

    //eigener Code
    pub fn find_declaration(&self, ident: String, uri: Uri) -> Result<Location, Error> {
        //check for type or signature first
        if let Some((start, end)) = self.find_data(&ident).or_else(|| self.find_codata(&ident)) {
            return Ok(Location::new(uri.clone(), Range { start, end }));
        }

        //only if no type or signature exist then search for definition
        if let Some((start, end)) = self.find_def(&ident) {
            return Ok(Location::new(uri.clone(), Range { start, end }));
        }

        Err(Error::UndefinedIdentifier(ident))
    }

    //eigener Code
    //getter für text holen um ihn zu formatieren
    pub fn get_text(&self) -> &str {
        &self.source
    }

    //eigener Code
    //ganzen zusammenhängenden String finden auf dem man gerade ist
    pub fn get_rangeident(&self, pos: Position) -> Result<Range, Error> {
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

        Ok(Range {
            start: Position {
                line: pos.line,
                character: start_pos as u32,
            },
            end: Position {
                line: pos.line,
                character: end_pos as u32,
            },
        })
    }

    //eigener Code
    //determine hover information
    pub fn get_hover_information(&self, ident: &str) -> Result<String, Error> {
        //find definition
        if let Some(def) = self.module.defs.iter().find(|item| item.name == ident) {
            let def_param_list: Vec<String> = def
                .context
                .bindings
                .iter()
                .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                .collect();
            let def_param_string = if def_param_list.is_empty() {
                "...".to_string()
            } else {
                def_param_list.join(", ")
            };
            let return_type = def.ret_ty.print_to_string(None);
            return Ok(format!(
                "```fun\nfn {}({}) of Type {}\n```\n\nDefinition: **{}**.", //TODO anpassen was es sagen soll
                def.name, def_param_string, return_type, def.name
            ));
        }

        //find datatype
        if let Some(datatype) = self
            .module
            .data_types
            .iter()
            .find(|item| item.name == ident)
        {
            return Ok(format!(
                "```fun\ndata {} = ...\n```\n\nDatatype: **{}**.", //TODO anpassen was es sagen soll
                datatype.name, datatype.name
            ));
        }

        //find Constructors
        for data in &self.module.data_types {
            if let Some(ctor) = data.ctors.iter().find(|item| item.name == ident) {
                let ctor_param_list: Vec<String> = ctor
                    .args
                    .bindings
                    .iter()
                    .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                    .collect();

                let ctor_param_string = if ctor_param_list.is_empty() {
                    "...".to_string()
                } else {
                    ctor_param_list.join(", ")
                };

                //let return_type = ctor.
                return Ok(format!(
                    "```fun\n{}({})\n```\n\nConstructor of Type: **{}**.", //TODO anpassen was es sagen soll
                    ctor.name, ctor_param_string, data.name
                ));
            }
        }

        //find Deconstructors
        for cod in &self.module.codata_types {
            if let Some(dtor) = cod.dtors.iter().find(|item| item.name == ident) {
                let dtor_param_list: Vec<String> = dtor
                    .args
                    .bindings
                    .iter()
                    .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                    .collect();

                let dtor_param_string = if dtor_param_list.is_empty() {
                    "...".to_string()
                } else {
                    dtor_param_list.join(", ")
                };
                let return_type = dtor.cont_ty.print_to_string(None);

                return Ok(format!(
                    "```fun\ndestructor {}({}) -> {}\n```\n\nDeconstructor of Type: **{}**.", //TODO anpassen was es sagen soll
                    dtor.name, dtor_param_string, return_type, cod.name
                ));
            }
        }

        Err(Error::UndefinedIdentifier(ident.to_owned()))
    }

    //eigener Code
    //getter für module
    pub fn module(&self) -> &CheckedProgram {
        &self.module
    }
    //eigener Code
    //determine signature information
    pub fn get_signature_information(&self, ident: &str) -> Result<String, Error> {
        //find definition
        if let Some(def) = self.module.defs.iter().find(|item| item.name == ident) {
            let def_param_list: Vec<String> = def
                .context
                .bindings
                .iter()
                .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                .collect();
            let def_param_string = if def_param_list.is_empty() {
                "...".to_string()
            } else {
                def_param_list.join(", ")
            };
            let return_type = def.ret_ty.print_to_string(None);
            return Ok(format!(
                "```fun\nfn {}({}) of Type {}\n```\n\nDefinition: **{}**.", //TODO anpassen was es sagen soll
                def.name, def_param_string, return_type, def.name
            ));
        }

        //find datatype
        if let Some(datatype) = self
            .module
            .data_types
            .iter()
            .find(|item| item.name == ident)
        {
            return Ok(format!(
                "```fun\ndata {} = ...\n```\n\nDatatype: **{}**.", //TODO anpassen was es sagen soll
                datatype.name, datatype.name
            ));
        }

        //find Constructors
        for data in &self.module.data_types {
            if let Some(ctor) = data.ctors.iter().find(|item| item.name == ident) {
                let ctor_param_list: Vec<String> = ctor
                    .args
                    .bindings
                    .iter()
                    .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                    .collect();

                let ctor_param_string = if ctor_param_list.is_empty() {
                    "...".to_string()
                } else {
                    ctor_param_list.join(", ")
                };

                //let return_type = ctor.
                return Ok(format!(
                    "```fun\n{}({})\n```\n\nConstructor of Type: **{}**.", //TODO anpassen was es sagen soll
                    ctor.name, ctor_param_string, data.name
                ));
            }
        }

        //find Deconstructors
        for cod in &self.module.codata_types {
            if let Some(dtor) = cod.dtors.iter().find(|item| item.name == ident) {
                let dtor_param_list: Vec<String> = dtor
                    .args
                    .bindings
                    .iter()
                    .map(|conbin| format!("{}: {}", conbin.var, conbin.ty.print_to_string(None)))
                    .collect();

                let dtor_param_string = if dtor_param_list.is_empty() {
                    "...".to_string()
                } else {
                    dtor_param_list.join(", ")
                };
                let return_type = dtor.cont_ty.print_to_string(None);

                return Ok(format!(
                    "```fun\ndestructor {}({}) -> {}\n```\n\nDeconstructor of Type: **{}**.", //TODO anpassen was es sagen soll
                    dtor.name, dtor_param_string, return_type, cod.name
                ));
            }
        }

        Err(Error::UndefinedIdentifier(ident.to_owned()))
    }

    //eigener Code
    pub fn find_appearences(&self, ident: &str) -> Result<Vec<Range>, Error> {
        let mut appearences = Vec::new();

        for (line_number, line) in self.source.lines().enumerate() {
            let mut search_index = 0;
            while let Some(pos) = line[search_index..].find(ident) {
                let found_appearence = search_index + pos;
                let chars: Vec<char> = line.chars().collect();
                let length = ident.len();

                let infront = found_appearence == 0
                    || !chars[found_appearence - 1].is_alphanumeric()
                        && chars[found_appearence - 1] != '_';

                let behind = found_appearence + length == chars.len()
                    || !chars[found_appearence + length].is_alphanumeric()
                        && chars[found_appearence + length] != '_';

                if infront && behind {
                    appearences.push(Range {
                        start: Position {
                            line: line_number as u32,
                            character: found_appearence as u32,
                        },
                        end: Position {
                            line: line_number as u32,
                            character: (found_appearence + length) as u32,
                        },
                    });
                }
                search_index = found_appearence + ident.len();
            }
        }
        if appearences.is_empty() {
            return Err(Error::UndefinedIdentifier(ident.to_string()));
        }
        Ok(appearences)
    }
}

impl Default for Document {
    fn default() -> Document {
        Document::new()
    }
}
