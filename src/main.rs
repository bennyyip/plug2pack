#[macro_use]
extern crate nom;

mod parser;

#[derive(Debug, Clone)]
pub struct Package {
  pub name: String,
  pub category: String,
  pub opt: bool,
  /// Load this package on this command
  pub load_command: Option<String>,
  /// Load this package for these types
  pub for_types: Vec<String>,
  /// Build command for this package
  pub build_command: Option<String>,
  /// Local plugin
  pub local: bool,
}


fn main() {
    let h = parser::parse(&b"plug 'bennyyip/plug2pack' { 'for':['rust', 'js', 'python'], 'on': 'run', 'do': 'make' }"[..]);
    println!("{:?}", h);
    let h = parser::parse(&b"pack 'bennyyip/plug2pack' { 'for':['rust', 'js', 'python'], 'on': 'run', 'do': 'make' }"[..]);
    println!("{:?}", h);
}
