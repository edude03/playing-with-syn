use serde::{Serialize};
use url::{Url};
use my_project::Record;
/*
The goal is to be able to call "save" for structs
*/

trait Record {
  fn path() -> &'static str;
}


trait Mutator {
  fn save<T: Record + Serialize>(&self, thing: T);
}

struct Http {
  base_url: Url
}

impl Http {
  fn new(base_url: Url) -> Self {
    Self {
      base_url
    }
  }
}

impl Mutator for Http {
  fn save<T: Record + Serialize>(&self, thing: T) -> () {
    let url = format!("{}{}", self.base_url, T::path());
    let payload = serde_json::to_value(thing).unwrap();
    println!("Sending payload: {} to {}",payload.to_string(), url)
  }
}


#[derive(Debug, Serialize, Record)]
struct MyStruct {
  name: &'static str,
  age: i32
}

fn main() {
  let mutator = Http::new("https://test.com".parse().unwrap());
  let thing = MyStruct {
    name: "Foo bar",
    age: 23
  };

  mutator.save(thing)
}