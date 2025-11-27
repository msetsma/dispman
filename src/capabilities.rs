use std::collections::HashMap;
use std::fmt;
use crate::vcp::VcpFeature;

#[derive(Debug, Default)]
pub struct Capabilities {
    pub protocol: Option<String>,
    pub display_type: Option<String>,
    pub model: Option<String>,
    pub commands: Vec<String>,
    pub vcp_features: HashMap<u8, Vec<u16>>,
    pub mccs_version: Option<String>,
    pub raw: String,
}

impl Capabilities {
    pub fn parse(raw: &str) -> Self {
        let mut caps = Capabilities {
            raw: raw.to_string(),
            ..Default::default()
        };

        // Simple parser for the nested parenthesis structure
        // We can just look for top-level keys like "prot", "type", "model", "vcp"
        
        // Remove outer parens if present
        let content = raw.trim();
        let content = if content.starts_with('(') && content.ends_with(')') {
            &content[1..content.len()-1]
        } else {
            content
        };

        let mut chars = content.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c.is_alphanumeric() || c == '_' {
                let mut key = String::new();
                key.push(c);
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        key.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                // Expect '('
                if let Some('(') = chars.peek() {
                    chars.next(); // consume '('
                    let value = parse_paren_content(&mut chars);
                    
                    match key.as_str() {
                        "prot" => caps.protocol = Some(value),
                        "type" => caps.display_type = Some(value),
                        "model" => caps.model = Some(value),
                        "mccs_ver" => caps.mccs_version = Some(value),
                        "cmds" => {
                            caps.commands = value.split_whitespace().map(String::from).collect();
                        },
                        "vcp" => {
                            caps.vcp_features = parse_vcp_string(&value);
                        },
                        _ => {} // Ignore unknown keys
                    }
                }
            }
        }

        caps
    }
}

fn parse_paren_content(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                depth += 1;
                content.push(c);
            },
            ')' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                content.push(c);
            },
            _ => content.push(c),
        }
    }
    content
}

fn parse_vcp_string(vcp_str: &str) -> HashMap<u8, Vec<u16>> {
    let mut features = HashMap::new();
    let mut chars = vcp_str.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() { continue; }
        
        // Read hex code
        if c.is_ascii_hexdigit() {
            let mut code_str = String::new();
            code_str.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_hexdigit() {
                    code_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if let Ok(code) = u8::from_str_radix(&code_str, 16) {
                let mut values = Vec::new();

                // Check for nested values in parens
                // Skip whitespace
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some('(') = chars.peek() {
                    chars.next(); // consume '('
                    let values_str = parse_paren_content(&mut chars);
                    for val_str in values_str.split_whitespace() {
                        if let Ok(val) = u16::from_str_radix(val_str, 16) {
                            values.push(val);
                        }
                    }
                }
                
                features.insert(code, values);
            }
        }
    }

    features
}

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Monitor Capabilities:")?;
        if let Some(model) = &self.model {
            writeln!(f, "  Model: {}", model)?;
        }
        if let Some(dtype) = &self.display_type {
            writeln!(f, "  Type: {}", dtype)?;
        }
        if let Some(prot) = &self.protocol {
            writeln!(f, "  Protocol: {}", prot)?;
        }
        if let Some(mccs) = &self.mccs_version {
            writeln!(f, "  MCCS Version: {}", mccs)?;
        }
        
        writeln!(f, "\nSupported VCP Features:")?;
        let mut codes: Vec<_> = self.vcp_features.keys().collect();
        codes.sort();
        
        for code in codes {
            let feature = VcpFeature::from_code(*code);
            let name = feature.name();
            let values = &self.vcp_features[code];
            
            write!(f, "  0x{:02X} ({})", code, name)?;
            
            if !values.is_empty() {
                write!(f, " -> Supported Values: [")?;
                for (i, val) in values.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "0x{:X}", val)?;
                    
                    // Add friendly names for common values
                    if *code == 0x60 { // Input Source
                         let input = crate::vcp::InputSource::from_value(*val);
                         write!(f, " ({:?})", input)?;
                    }
                }
                write!(f, "]")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
