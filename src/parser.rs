use std::str;
use std::collections::HashMap;
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Array(Vec<String>),
}

named!(string <&str>,
       delimited!(
         tag!("\'"),
         map_res!(take_until!("\'"), str::from_utf8),
         tag!("\'"))
      );

named!(array <Vec<&str>>,
       ws!(
         delimited!(
           tag!("["),
           separated_list!(tag!(","), string),
           tag!("]")
           )
         )
      );

named!(key_value <(&str,Value)>,
ws!(
  separated_pair!(
    string,
    tag!(":"),
    value
    )
  )
);

named!(hash <HashMap<&str, Value>>,
       ws!(
         map!(
           delimited!(
             tag!("{"),
             separated_list!(tag!(","), key_value),
             tag!("}")
             )
           ,
           |tuple_vec| {
             let mut h: HashMap<&str, Value> = HashMap::new();
             for (k, v) in tuple_vec {
               h.insert(k, v);
             }
             h
           }
           )
         )
      );

named!(value<Value>,
       ws!(
         alt!(
           array  => { | v:Vec<&str> | Value::Array(v.iter().map( | &s | String::from(s)).collect()) } |
           string => { | s           | Value::Str(String::from(s)) }
           )
         )
      );


named!(parse_plug<(&str, Option<HashMap<&str,Value>>)>,
ws!(
  do_parse!(
    tag!("Plug")
    >> name: string
    >> options: alt!(
      eof!()                    => {|_| None} |
      tag!("\"")                => {|_| None} |
      preceded!(tag!(","),hash) => {|h| Some(h)}
      )
    >> (name,options)
    )
  )
);


pub fn parse(content: &[u8]) -> Option<(&str, Option<HashMap<&str, Value>>)> {
    match parse_plug(content) {
        IResult::Done(_, r) => Some(r),
        _ => None,
    }
}

#[test]
fn parse_works() {
    assert_eq!(IResult::Done(&b""[..], "foo"), string(&b"'foo'"[..]));

    assert_eq!(IResult::Done(&b""[..], Value::Str("foo".to_owned())), value(&b"'foo'"[..]));

    assert_eq!(IResult::Done(&b""[..],("for",Value::Str("python".to_owned()))),key_value(&b"'for':'python' "[..]));

    let s = &b" { 'for': 'python' }";
    let mut h = HashMap::new();
    h.entry("for").or_insert(Value::Str("python".to_owned()));

    assert_eq!(IResult::Done(&b""[..],h.clone()),hash(&s[..]));

    let s = &b"Plug 'davidhalter/jedi-vim', { 'for': 'python' }";

    assert_eq!(IResult::Done(&b""[..], ("davidhalter/jedi-vim",Some(h.clone()))), parse_plug(&s[..]) );

    let s = &b"Plug 'davidhalter/jedi-vim'";

    assert_eq!(IResult::Done(&b""[..], ("davidhalter/jedi-vim",None)), parse_plug(&s[..]) );
}
