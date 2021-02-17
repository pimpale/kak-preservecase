use std::io::{Read};
use regex::{Captures, Regex, RegexBuilder};

#[derive(Debug, Clone)]
struct AsciiCasePreservingReplace {
  to: String,
  regex: Regex,
}

impl AsciiCasePreservingReplace {
  fn new(from: &str, to: &str) -> AsciiCasePreservingReplace {
    AsciiCasePreservingReplace {
      to: to.to_owned(),
      regex: RegexBuilder::new(&from)
        .case_insensitive(true)
        .build()
        .unwrap(),
    }
  }

  fn replace(&self, s: &str) -> String {
    self.regex.replace_all(s, self).to_string()
  }
}

impl<'a> regex::Replacer for &'a AsciiCasePreservingReplace {
  fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
    for s in caps.iter() {
      let mut src_chars = s.unwrap().as_str().chars();
      for rc in self.to.chars() {
        if let Some(sc) = src_chars.next() {
          // TODO what if we want to default to lowercase
          // replacement || source
          if rc.is_uppercase() || sc.is_uppercase() {
            dst.push_str(&rc.to_uppercase().to_string());
          } else {
            dst.push(rc);
          }
        } else {
          dst.push(rc);
        }
      }
    }
  }
}

fn usage() {
    println!("\n\
        Usage: kak-preservecase REGEX REPLACEMENT\n\
        Reads from stdin, where it replaces REGEX with REPLACEMENT in a case preserving manner.\n\
        Example: kak-preservecase alpha Beta\n\
        \n\
        REGEX is an case insensitive extended regular expression.\n\
        Every instance of REGEX will be replaced with REPLACEMENT.\n\
        The replacement character is uppercase if REPLACEMENT is uppercase or the matched text is uppercase.\n\
    ");
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    usage();
    return;
  }
  // regex is first, replacement rule is second
  let preserver = AsciiCasePreservingReplace::new(&args[1], &args[2]);

  // replace
  let mut buffer = String::new();
  std::io::stdin().read_to_string(&mut buffer).unwrap();
  print!("{}", preserver.replace(&buffer));
}
