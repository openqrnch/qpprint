use terminal_size::{Width, Height, terminal_size};

fn wordify(s: &str) -> Vec<String> {
  let mut words = Vec::new();
  let splt = s.split_whitespace();

  for w in splt {
    words.push(w.to_string());
  }
  words
}


pub struct PPrint {
  indent: u16,
  hang: i16,
  maxwidth: u16
}

impl PPrint {
  pub fn new() -> Self {
    let size = terminal_size();
    let mut maxwidth: u16 = 80;
    if let Some((Width(w), Height(_h))) = size {
      maxwidth = w;
    }

    PPrint { indent: 0, hang: 0, maxwidth }
  }

  pub fn set_indent(&mut self, indent: u16) -> &mut Self {
    self.indent = indent;
    self
  }

  pub fn set_hang(&mut self, hang: i16) -> &mut Self {
    self.hang = hang;
    self
  }

  /*
  pub(crate) fn set_maxwidth(&mut self, maxwidth: u16) -> &mut Self {
    self.maxwidth = maxwidth;
    self
  }
  */

  pub fn print_words<I, S>(&self, out: &mut dyn std::io::Write,
          words: I)
      where I: IntoIterator<Item=S>, S: AsRef<str> {
    let mut firstline = true;
    let mut newline = true;
    let mut space: u16 = 0;
    let mut col: u16 = 0;

    for w in words.into_iter() {
      let w = w.as_ref();
      if col + space + w.len() as u16 > self.maxwidth {
        out.write(b"\n").unwrap();
        newline = true;
      }

      if newline {
        let mut indent: i16 = 0;

        indent += self.indent as i16;
        if firstline {
          indent += self.hang;
        }

        out.write(" ".repeat(indent as usize).as_bytes()).unwrap();
        col = indent as u16;

        newline = false;
        space = 0;
        firstline = false;
      }

      out.write(" ".repeat(space as usize).as_bytes()).unwrap();
      col += space;

      out.write(w.as_bytes()).unwrap();
      col += w.len() as u16;

      let ch = w.chars().last().unwrap();
      match ch {
        '.' | '?' | '!' => {
          space = 2;
        }
        _ => {
          space = 1;
        }
      }
    }
    out.write(b"\n").unwrap();
  }

  pub fn print_p(&self, out: &mut dyn std::io::Write, para: &str) {
    let words = wordify(para);
    self.print_words(out, &words);
  }

  pub fn print_plist<I, S>(&self, out: &mut dyn std::io::Write, parit: I)
      where I: IntoIterator<Item=S>, S: AsRef<str> {
    for p in parit {
      self.print_p(out, p.as_ref());
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
