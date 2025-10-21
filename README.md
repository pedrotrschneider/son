# SON (Simple Object Notation)

A lightweight, human-readable data serialization format for Rust with a JSON-like syntax that supports optional commas and comments.

## Features

- **Human-friendly syntax** - Optional commas, inline and block comments
- **Type-safe serialization/deserialization** - Derive macros for automatic implementation
- **Rich type support** - Primitives, structs, enums, vectors, and nested structures
- **Error handling** - Detailed error messages for parsing and deserialization failures
- **Zero-copy parsing** - Efficient buffered reading for large files

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
son = { path = "." }
```

## Syntax Overview

SON syntax is similar to JSON but more flexible:

```son
{
    // Single-line comments
    name: "Alice"
    age: 30
    active: true
    
    /* Multi-line
       comments */
    address: {
        city: "New York"
        zip: "10021"
    }
    
    // Commas are optional
    skills: ["Rust" "Python" "Go"]
    
    // Enum variants
    status: Active
}
```

### Key Differences from JSON

- **Optional commas** - Commas between fields and array elements are optional
- **Comments** - Supports `//` inline and `/* */` block comments
- **Char literals** - Single quotes for characters: `'A'`
- **Enum variants** - Unquoted identifiers for enum values
- **Flexible whitespace** - More lenient with spacing

## Quick Start

### Basic Serialization

```rust
use son::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
    active: bool,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        active: true,
    };
    
    // Serialize to string
    let son_string = son::to_string(&person);
    println!("{}", son_string);
    
    // Deserialize from string
    let parsed: Person = son::from_str(&son_string).unwrap();
    assert_eq!(person, parsed);
}
```

### Pretty Printing

```rust
use son::{Serialize, to_string_pretty};

let person = Person {
    name: "Bob".to_string(),
    age: 25,
    active: false,
};

// Pretty print with custom indentation
let pretty = to_string_pretty(&person, "  ");
println!("{}", pretty);
```

Output:
```son
{
  name: "Bob"
  age: 25
  active: false
}
```

### Working with Files

```rust
use son::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), son::Error> {
    // Read from file
    let config: Config = son::from_file("config.son")?;
    
    println!("Host: {}, Port: {}", config.host, config.port);
    Ok(())
}
```

## Supported Types

### Primitives

```rust
#[derive(Serialize, Deserialize)]
struct Primitives {
    // Strings
    text: String,
    character: char,
    
    // Booleans
    flag: bool,
    
    // Integers (signed)
    tiny: i8,
    small: i16,
    medium: i32,
    large: i64,
    huge: i128,
    pointer_sized: isize,
    
    // Integers (unsigned)
    byte: u8,
    word: u16,
    dword: u32,
    qword: u64,
    usize_val: usize,
    
    // Floats
    float: f32,
    double: f64,
}
```

### Collections

```rust
#[derive(Serialize, Deserialize)]
struct Collections {
    // Vectors
    numbers: Vec<i32>,
    strings: Vec<String>,
    
    // Nested vectors
    matrix: Vec<Vec<f64>>,
    
    // HashMaps (keys must be ToString)
    metadata: HashMap<String, String>,
}
```

### Nested Structures

```rust
#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    address: Address,
    friends: Vec<String>,
}
```

### Enums (Unit Variants)

```rust
#[derive(Serialize, Deserialize)]
enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Serialize, Deserialize)]
struct Account {
    username: String,
    status: Status,
}
```

**SON representation:**
```son
{
    username: "alice"
    status: Active
}
```

## Advanced Usage

### Optional Fields

```rust
use son::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
    email: Option<String>,
    age: Option<i32>,
}
```

**SON with nulls:**
```son
{
    name: "Alice"
    email: "alice@example.com"
    age: null
}
```

### Complex Nested Structures

```rust
#[derive(Serialize, Deserialize)]
struct Department {
    name: String,
    floor: i32,
}

#[derive(Serialize, Deserialize)]
struct Employee {
    id: i32,
    name: String,
    department: Department,
    skills: Vec<String>,
    active: bool,
}

#[derive(Serialize, Deserialize)]
struct Company {
    name: String,
    employees: Vec<Employee>,
}
```

### Working with the Value Type

The `Value` enum represents any SON value dynamically:

```rust
use son::Value;
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("name".to_string(), Value::String("Alice".to_string()));
map.insert("age".to_string(), Value::Integer(30));
map.insert("scores".to_string(), Value::Array(vec![
    Value::Integer(95),
    Value::Integer(87),
    Value::Integer(92),
]));

let obj = Value::Object(map);
println!("{}", obj);
```

## Error Handling

SON provides detailed error types for different failure scenarios:

```rust
use son::{Error, ParseError, DeserializationError};

match son::from_str::<Person>(invalid_son) {
    Ok(person) => println!("Parsed: {:?}", person),
    Err(Error::ParseError(e)) => {
        eprintln!("Parse error: {}", e);
    }
    Err(Error::DeserializationError(e)) => {
        eprintln!("Deserialization error: {}", e);
    }
    Err(Error::IOError(e)) => {
        eprintln!("IO error: {}", e);
    }
}
```

### Common Errors

- **UnexpectedType** - Type mismatch during deserialization
- **MissingField** - Required field not found in SON data
- **InvalidValue** - Value doesn't meet constraints (e.g., negative number for unsigned type)
- **UnknownVariant** - Unknown enum variant
- **UnexpectedEOF** - File ended unexpectedly
- **UnexpectedToken** - Invalid syntax

## Examples

### Configuration File

**config.son:**
```son
{
    server: {
        host: "localhost"
        port: 8080
        workers: 4
    }
    
    database: {
        url: "postgres://localhost/mydb"
        pool_size: 20
        timeout: 30.0
    }
    
    // Feature flags
    features: {
        auth: true
        cache: true
        metrics: false
    }
}
```

**Rust code:**
```rust
#[derive(Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: u32,
}

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
    timeout: f64,
}

#[derive(Serialize, Deserialize)]
struct Features {
    auth: bool,
    cache: bool,
    metrics: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    features: Features,
}

fn main() -> Result<(), son::Error> {
    let config: Config = son::from_file("config.son")?;
    println!("Server running on {}:{}", config.server.host, config.server.port);
    Ok(())
}
```

### User Data

**users.son:**
```son
[
    {
        id: 1
        name: "Alice"
        email: "alice@example.com"
        roles: ["admin", "user"]
    }
    {
        id: 2
        name: "Bob"
        email: "bob@example.com"
        roles: ["user"]
    }
]
```

**Rust code:**
```rust
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    roles: Vec<String>,
}

fn main() -> Result<(), son::Error> {
    let users: Vec<User> = son::from_file("users.son")?;
    
    for user in users {
        println!("{}: {} ({})", user.id, user.name, user.email);
    }
    
    Ok(())
}
```

## API Reference

### Serialization

- `to_son<T: Serialize>(value: &T) -> Value` - Convert to Value
- `to_string<T: Serialize>(value: &T) -> String` - Convert to compact string
- `to_string_pretty<T: Serialize>(value: &T, indent: &str) -> String` - Convert to formatted string

### Deserialization

- `from_str<T: Deserialize>(s: &str) -> Result<T, Error>` - Parse from string
- `from_file<T: Deserialize>(path: &str) -> Result<T, Error>` - Parse from file

### Traits

- `Serialize` - Marker trait for serializable types
- `ToSon` - Core serialization trait (usually derived)
- `Deserialize` - Marker trait for deserializable types
- `FromSon` - Core deserialization trait (usually derived)

## License

MIT License - Copyright (c) 2025 Pedro Schneider

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Limitations

- Enum support is currently limited to unit variants (no data-carrying variants)
- `u128` cannot be safely serialized (stored internally as `i128`)
- No built-in support for dates, UUIDs, or other specialized types (use strings or custom implementations)

## Future Plans

- Support for enum variants with data
- Macro for custom serialization/deserialization
- Schema validation
- Better error messages with line/column information
- Streaming parser for very large files