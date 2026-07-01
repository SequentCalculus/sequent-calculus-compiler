use crate::errors::Error;
use fun::{parser::parse_module, syntax::program::CheckedProgram};
use log::info;
use lsp_server::Response;
use lsp_types::{Location, Position, Range, Uri};
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

        if pos.character as usize >= line.len() {
            return Err(Error::InvalidPosition(pos));
        }

        let mut end_pos = pos.character as usize;
        while end_pos < line.len() {
            let character = line.chars().nth(end_pos).unwrap();
            if !character.is_alphanumeric() && character != '_' {
                break;
            }
            end_pos += 1;
        }

        let mut start_pos = pos.character as usize;
        while start_pos > 0 {
            let character = line.chars().nth(start_pos - 1).unwrap();
            if !character.is_alphanumeric() && character != '_' {
                break;
            }
            start_pos -= 1;
        }

        let ident = &line[start_pos..end_pos];
        Ok(ident.to_owned())
    }

    //eigener Code
    // index to position
    pub fn ind_to_pos(&self, index: usize) -> Position {
        let mut line = 0;
        let mut character = 0;

        for (byte_offset, ch) in self.source.char_indices() {
            if byte_offset >= index {
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
    /*
    //alter code
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
        } */

    fn find_def(&self, ident: &str) -> Option<(Position, Position)> {
        let span = self
            .module
            .defs
            .iter()
            .find_map(|df| (df.name == ident).then_some(df.span))?;
        let mut start = self.ind_to_pos(span.offset());
        //"def "
        start.character += 4;
        let end = self.ind_to_pos(span.offset() + span.len());
        Some((start, end))
    }

    fn find_data(&self, ident: &str) -> Option<(Position, Position)> {
        for data in &self.module.data_types {
            if data.name == ident {
                let mut start = self.ind_to_pos(data.span?.offset());
                start.character +=5;
                let end = self.ind_to_pos(data.span?.offset() + data.span?.len());
                return Some((start, end));
            }
            if let Some(ctor) = data.ctors.iter().find(|ctor| ctor.name == ident) {
                if let Some(span) = ctor.span {
                    let start = self.ind_to_pos(span.offset());
                    let end = self.ind_to_pos(span.offset() + span.len());
                    return Some((start, end));
                }
            }          
        }
        None
        
       /* 
        let span = self.module.data_types.iter().find_map(|data| {
            (data.name == ident || data.ctors.iter().any(|ctor| ctor.name == ident))
                .then_some(data.span)
        })??;
        let mut start = self.ind_to_pos(span.offset());
        // "data "
        start.character += 5;
        let end = self.ind_to_pos(span.offset() + span.len());
        Some((start, end)) */
    }


    fn find_codata(&self, ident: &str) -> Option<(Position, Position)> {
        for cod in &self.module.codata_types {
            if cod.name == ident {
                let mut start = self.ind_to_pos(cod.span?.offset());
                start.character +=7;
                let end = self.ind_to_pos(cod.span?.offset() + cod.span?.len());
                return Some((start, end));
            }
            if let Some(dtor) = cod.dtors.iter().find(|dtor| dtor.name == ident) {
                if let Some(span) = dtor.span {
                    let start = self.ind_to_pos(span.offset());
                    let end = self.ind_to_pos(span.offset() + span.len());
                    return Some((start, end));
                }
            }          
        }
        None
    }
   /*/ fn find_codata(&self, ident: &str) -> Option<(Position, Position)> {
        let span = self.module.codata_types.iter().find_map(|cod| {
            (cod.name == ident || cod.dtors.iter().any(|dtor| dtor.name == ident))
                .then_some(cod.span)
        })??;
        let mut start = self.ind_to_pos(span.offset());
        // "codata "
        start.character += 7;
        let end = self.ind_to_pos(span.offset() + span.len());
        Some((start, end))
    }*/

    pub fn is_local(&self, ident: &str, pos:Position, uri: Uri) -> Option<Location>{
        let cursor_ind = self.pos_to_ind(pos);
        let this_def = self.module.defs.iter().find(|def| {
            let start = def.span.offset();
            let end = def.span.offset() + def.span.len();
            cursor_ind >= start && cursor_ind <= end
        })?;

        let def_start = this_def.span.offset();
        let def_end = def_start + this_def.span.len();
        let def_text = &self.source[def_start..def_end];
        let local_offset = def_text.find(ident)?;
        let abs_offset = def_start + local_offset;
        let start = self.ind_to_pos(abs_offset);
        let end = self. ind_to_pos(abs_offset + ident.len());

        Some(Location::new(uri, Range {start, end}))
    } 

    //bearbeitet auch selbstprüfend jetzt
    pub fn find_ident(&self, ident: String, pos: Position, uri: Uri) -> Result<Location, Error> {
        if ident == "i64"{
            return Err(Error::PrimitiveType(ident));
        }
        
        if matches!(ident.as_str(), "new" | "case" | "codata" | "data" | "def") {
        return Err(Error::Keyword(ident));
        }

        if let Some(loc) = self.is_local(&ident, pos, uri.clone()) {
            return Ok(loc);
        } 

        if let Some((start, end)) = self
            .find_def(&ident)
            .or_else(|| self.find_data(&ident))
            .or_else(|| self.find_codata(&ident))
        {
            Ok(Location::new(uri, Range { start, end }))
        } else if let Some(def) = self.module.defs.iter().find(|def| def.name == ident) {
            let start = self.ind_to_pos(def.span.offset());
            let end = self.ind_to_pos(def.span.offset() + def.span.len());
            Ok(Location::new(uri, Range { start, end }))
        } else {
            Err(Error::UndefinedIdentifier(ident))
        }
    }

    //eigener Code
    pub fn find_implementation(&self, ident: String, uri: Uri) -> Result<Location, Error> {
        //Suchen von Funktionsimplementierungen die mit ident übereinstimmen
        if let Some(def) = self.module.defs.iter().find(|def| def.name == ident) {
            let start = self.ind_to_pos(def.span.offset());
            let end = self.ind_to_pos(def.span.offset() + def.span.len());
            return Ok(Location::new(uri, Range { start, end }));
        }

        //Suchen von Konstruktor die mit ident übereinstimmen
        for data in &self.module.data_types {
            if let Some(ctor) = data.ctors.iter().find(|ctor| ctor.name == ident) {
                let span = ctor
                    .span
                    .unwrap_or(miette::SourceSpan::new(0.into(), 0.into()));
                let start = self.ind_to_pos(span.offset());
                let end = self.ind_to_pos(span.offset() + span.len());
                return Ok(Location::new(uri.clone(), Range { start, end }));
            }
        }

        //Suchen von Destruktoren die mit ident übereinstimmen
        for cod in &self.module.codata_types {
            if let Some(dtor) = cod.dtors.iter().find(|dtor| dtor.name == ident) {
                let span = dtor
                    .span
                    .unwrap_or(miette::SourceSpan::new(0.into(), 0.into()));
                let start = self.ind_to_pos(span.offset());
                let end = self.ind_to_pos(span.offset() + span.len());
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
    //echte end Position bestimmen (chars zählen nicht bytes)
    pub fn get_end_pos(&self) -> lsp_types::Position {
        let text = self.get_text();
        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return lsp_types::Position {
                line: 0,
                character: 0,
            };
        }
        let last_line_ind = lines.len() - 1;
        let last_line_char_amount = lines[last_line_ind].chars().count();
        lsp_types::Position {
            line: last_line_ind as u32,
            character: last_line_char_amount as u32,
        }
    }

    //eigener Code
    //ganzen zusammenhängenden String finden auf dem man gerade ist
    pub fn get_rangeident(&self, pos: Position) -> Result<Range, Error> {
        let line = self
            .source
            .lines()
            .nth(pos.line as usize)
            .ok_or(Error::InvalidPosition(pos))?;

        let chars: Vec<char> = line.chars().collect();
        let area_pos = pos.character as usize;

        if area_pos >= chars.len() || (!chars[area_pos].is_alphanumeric() && chars[area_pos] != '_') {
            return Err(Error::InvalidPosition(pos));
        }

        let mut end_pos = area_pos;
        while end_pos < chars.len() && (chars[end_pos].is_alphanumeric() || chars[end_pos] == '_') {
            end_pos += 1;            
        }

        let mut start_pos = area_pos;
        while start_pos > 0 && (chars[start_pos - 1].is_alphanumeric() || chars[start_pos] == '_') {
            start_pos -= 1;            
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
    //position to index
    pub fn pos_to_ind(&self, pos: Position) -> usize {
        let mut line = 0;
        let mut character = 0;

        for (byte_offset, ch) in self.source.char_indices() {
            if line == pos.line && character == pos.character {
                return byte_offset;
            }
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
        }
        self.source.len()
    }
    /*
       //alter code
        pub fn pos_to_ind(&self, pos: Position) -> usize {
            let mut ind = 0;
            for (i, line) in self.source.lines().enumerate(){
                if i == pos.line as usize{
                    break;
                }
                ind += line.len() + 1
            }
            ind += pos.character as usize;
            ind
        }
    */

    //eigener Code
    //determine hover information
    pub fn get_hover_information(
        &self,
        ident: &str,
        pos: lsp_types::Position,
    ) -> Result<String, Error> {
        //find definition
        let cursor_ind = self.pos_to_ind(pos);
        if let Some(own_def) = self.module.defs.iter().find(|item| {
            let start = item.span.offset();
            let end = item.span.offset() + item.span.len();
            cursor_ind >= start && cursor_ind <= end
        }) {
            if let Some(binding) = own_def.context.bindings.iter().find(|b| b.var == ident) {
                let ty = binding.ty.print_to_string(None);
                return Ok(format!(
                    "```fun\nlocal {}: {}\n```\n\nVariable in **{}**.",
                    ident, ty, own_def.name
                ));
            }
        }
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

        //Err(Error::UndefinedIdentifier(ident.to_owned()))
        Ok(format!("{} is undefined", ident))
    }

    //eigener Code
    //getter für module
    pub fn module(&self) -> &CheckedProgram {
        &self.module
    }

    
    //eigener Code
    //getter für die erste funktion die vor der stelle des Cursors ist
    pub fn get_fnc_before_cursor(&self, pos: lsp_types:: Position) -> Option<String> {
        let text = self.get_text();
        let all_lines: Vec<&str> = text.lines().collect();
        let line = all_lines.get(pos.line as usize)?;

        //teil der Zeile vor dem Cursor auswählen
        let cursor_idx = pos.character as usize;
        let pre_cursor: String = line.chars().take(cursor_idx).collect();

        //Letze öffnende Klammer finden und Wort vor Klammer extrahieren
        if let Some(bracket_idx) = pre_cursor.rfind('(') {
            let before_bracket = &pre_cursor[.. bracket_idx].trim_end();

            return before_bracket
                .split_terminator(|c: char| !c.is_alphanumeric() && c != '_')
                .last().map(|s| s
                .to_string());
        }
        None
    }

    //eigener Code
    //Find active parameter(commas after '(')
    pub fn get_active_parameter_idx(&self, pos: lsp_types::Position) -> usize {
        let text = self.get_text();
        let lines: Vec<&str> = text.lines().collect();
        
        if let Some(line) = lines.get(pos.line as usize){
            let cursor_pos = pos.character as usize;
            let pre_cursor: String = line.chars().take(cursor_pos).collect();
            if let Some(bracket_idx) = pre_cursor.rfind('('){
                let post_bracket_text = &pre_cursor[bracket_idx..];
                return post_bracket_text.chars().filter(|&c| c == ',').count();
            }
        }
        0
    }

    //eigener Code
    //
    pub fn get_function_parameter_info(&self, ident:&str) -> Vec<lsp_types::ParameterInformation> {
        if let Some(def) = self.module.defs.iter().find(|d| d.name == ident) {
            def.context.bindings.iter().map(|b| {
                lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::Simple(format!("{}: {}", b.var, b.ty.print_to_string(None))),
                    documentation :None,
                }
            }).collect()
        }else {
            vec![]
        }
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

    //eigener Code
    //




}

impl Default for Document {
    fn default() -> Document {
        Document::new()
    }
}
