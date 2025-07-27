use crate::Value;

pub struct Printer {
    indentation: String,
}

impl Printer {
    pub fn new(indentation: String) -> Self {
        return Self { indentation };
    }

    pub fn son_to_string(&self, son: &Value) -> String {
        return self.son_to_string_inner(son, 0);
    }

    fn son_to_string_inner(&self, son: &Value, indent: u32) -> String {
        let mut string = String::new();

        let print_indent = |string: &mut String, indent: u32| {
            for _ in 0..indent {
                string.push_str(&self.indentation);
            }
        };

        match son {
            Value::Null => string.push_str("null"),
            Value::Bool(b) => string.push_str(&b.to_string()),
            Value::Float(f) => string.push_str(&f.to_string()),
            Value::Integer(i) => string.push_str(&i.to_string()),
            Value::String(s) => {
                string.push('\"');
                string.push_str(&s);
                string.push('\"');
            }
            Value::Char(c) => {
                string.push('\'');
                string.push_str(&c.to_string());
                string.push('\'');
            }
            Value::Enum(s) => string.push_str(&s),
            Value::Array(a) => {
                string.push_str("[\n");
                for value in a {
                    print_indent(&mut string, indent + 1);
                    string.push_str(&self.son_to_string_inner(&value, indent + 1));
                }
                print_indent(&mut string, indent);
                string.push(']');
            }
            Value::Object(o) => {
                string.push_str("{\n");

                for (key, value) in o.iter() {
                    print_indent(&mut string, indent + 1);
                    string.push_str(&key.to_string());
                    string.push_str(": ");
                    string.push_str(&self.son_to_string_inner(&value, indent + 1));
                }
                print_indent(&mut string, indent);
                string.push('}');
            }
        };
        string.push('\n');
        return string;
    }
}