#![feature(conservative_impl_trait)]

extern crate postgres;
extern crate uuid;
extern crate pom;
extern crate dotenv;
extern crate graphql_parser;

use dotenv::dotenv;

use std::env;
use std::fs::File;
use std::io::{self, Read};
 use std::collections::HashMap;

use postgres::{Connection, TlsMode};

mod graphql;

use graphql_parser::parse_schema;
use graphql_parser::schema::{Definition, SchemaDefinition, TypeDefinition, TypeExtension, DirectiveDefinition, ObjectTypeExtension};

fn establish_connection() -> Connection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  Connection::connect(database_url.clone(), TlsMode::None)
    .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
  let args: Vec<String> = env::args().collect::<Vec<_>>()[1..].to_vec();

  let contents = read_file(&args[0]).unwrap();
  let file = String::from_utf8(contents.clone()).unwrap();

  let ast = parse_schema(&file).unwrap();

  println!("{:?}", ast);

  for def in ast.definitions {
    if Definition::TypeDefinition(typeDef) = def {
      if TypeDefinition::Object(obj) = typeDef {
        let fields = obj.fields;
      }
    }
  }
}

fn read_file(input: &str) -> io::Result<Vec<u8>> {
  let mut contents = vec![];

  File::open(input)?.read_to_end(&mut contents)?;

  Ok(contents)
}
