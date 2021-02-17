use std::io::{Read};
use regex::{Captures, Regex, RegexBuilder};

#[derive(Debug, Clone)]
struct AsciiCasePreservingReplace {
  query: String,
  replacement: String,
  regex: Regex,
}

impl AsciiCasePreservingReplace {
  fn new(query: &str, replacement: &str) -> AsciiCasePreservingReplace {
    AsciiCasePreservingReplace {
      query: query.to_owned(),
      replacement: replacement.to_owned(),
      regex: RegexBuilder::new(&query)
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
      let mut query_chars = self.query.chars();
      let mut match_chars = s.unwrap().as_str().chars();
      for rc in self.replacement.chars() {
        // true if char in the source text is upper
        let match_upper = match_chars.next()
           .map(|c| c.is_uppercase())
           .unwrap_or(false);
       // true if char in the query is upper
        let query_upper = query_chars.next()
           .map(|c| c.is_uppercase())
           .unwrap_or(false);

        if !query_upper && match_upper {
            // if query isn't upper and match is upper, then we can capitalize
            dst.push_str(&rc.to_uppercase().to_string());
        } else {
            // otherwise we keep the default replacement text
            dst.push(rc);
        }

      }
    }
  }
}

fn usage() {
    println!("\n\
        Usage: kak-preservecase QUERY REPLACEMENT\n\
        Reads from stdin, where it replaces QUERY with REPLACEMENT in a case preserving manner.\n\
        Example: kak-preservecase alpha Beta\n\
        \n\
        For matching purposes, QUERY is an case insensitive string.\n\
        It does not accept regex, as this would cause ambiguity.\n\
        Every instance of QUERY will be replaced with REPLACEMENT.\n\
        For every match at index i, uppercaseness is: (!QUERY[i] && MATCH[i]) || REPLACEMENT[i]\n\
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
