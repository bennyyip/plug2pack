use std::str;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

named!(hash <HashMap<String, Value>>,
       ws!(
         map!(
           delimited!(
             tag!("{"),
             separated_list!(tag!(","), key_value),
             tag!("}")
             )
           ,
           |tuple_vec| {
             let mut h: HashMap<String, Value> = HashMap::new();
             for (k, v) in tuple_vec {
               h.insert(String::from(k), v);
             }
             h
           }
           )
         )
      );

named!(value<Value>,
       ws!(
         alt!(
           array  => { |v:Vec<&str>|   Value::Array(v.iter().map(|&s| String::from(s)).collect()) } |
           string => { |s|   Value::Str(String::from(s)) }
           )
         )
      );

named!(pub parse<(&str, HashMap<String,Value>)>,
       ws!(
         do_parse!(
           tag!("plug")
           >> name: string
           >> options: hash
           >> (name,options)
           )
         )
      );
