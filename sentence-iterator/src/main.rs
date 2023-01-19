#[derive(Debug)]
struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}

impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}

impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next");
        if let Some(idx) = self.s.find(self.delimiter) {
            let v = &self.s[0..idx];
            *self.s = &self.s[idx + self.delimiter.len_utf8()..];
            Some(v)
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let mut s = "This is the 1st sentence.This is the 2nd sentence.It's the last.";
    let mut si = SentenceIter::new(&mut s, '.');
    assert_eq!("This is the 1st sentence", si.next().unwrap());
    assert_eq!("This is the 2nd sentence", si.next().unwrap());
    assert_eq!("It's the last", si.next().unwrap());
    assert_eq!(None, si.next());
}

fn main() {
    let mut s = "a。b。";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);
}
