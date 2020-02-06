pub mod model;
pub mod parser;

#[cfg(test)]
mod tests {
  use serde_json::Result;
  use std::fs;
  #[test]
  fn it_works() {
    let paths = fs::read_dir("examples-json/").unwrap();

    for path in paths {
      let unwrapped_path = path.unwrap().path();
      let schema_contents =
        fs::read_to_string(&unwrapped_path).expect("Something went wrong reading the file");
      let parsed: Result<crate::parser::FHIRResource> = serde_json::from_str(&schema_contents);
      match parsed {
        Ok(_) => println!("Successfully parsed {}", &unwrapped_path.to_str().unwrap()),
        Err(m) => assert!(
          false,
          "Error parsing {}: {}",
          &unwrapped_path.to_str().unwrap(),
          m
        ),
      }
    }
  }
}
