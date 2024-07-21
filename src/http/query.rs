
use std::collections::HashMap;

#[derive(Debug)]
pub struct Query<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Query<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        s.split('&').for_each(|pair| {
            let mut key = pair;
            let mut value = "";

            if let Some(i) = pair.find('=') {
                key = &pair[..i];
                value = &pair[i + 1..];
            }
            data.entry(key)
            .and_modify(|existing| match existing {
                Value::Single(prev) => {
                    *existing = Value::Multiple(vec![prev, value]);
                }                
                Value::Multiple(v) => v.push(value),
            })
            .or_insert(Value::Single(value));
        });
        Query {data}
    }
}