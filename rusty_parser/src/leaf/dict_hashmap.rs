use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone, Default)]
struct HashTrieNode<CharType, Output>
where
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    children: HashMap<CharType, HashTrieNode<CharType, Output>>,
    output: Option<Output>,
}

impl<CharType, Output> HashTrieNode<CharType, Output>
where
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            output: None,
        }
    }
    pub fn insert<CharIter>(&mut self, mut key: CharIter, output: Output) -> Option<Output>
    where
        CharIter: Iterator<Item = CharType>,
    {
        if let Some(key_val) = key.next() {
            self.children
                .entry(key_val)
                .or_insert(HashTrieNode::new())
                .insert(key, output)
        } else {
            let old_output = self.output.clone();
            self.output = Some(output);
            old_output
        }
    }

    pub fn match_longest<CharIter>(
        &self,
        mut key: CharIter,
        longest_matched: ParseResult<Output, CharIter>,
    ) -> ParseResult<Output, CharIter>
    where
        CharIter: Iterator<Item = CharType> + Clone,
    {
        if let Some(key_val) = key.next() {
            if let Some(child) = self.children.get(&key_val) {
                if child.output.is_some() {
                    return child.match_longest(
                        key.clone(),
                        ParseResult {
                            output: child.output.clone(),
                            it: key,
                        },
                    );
                } else {
                    return child.match_longest(key, longest_matched);
                }
            }
        }
        longest_matched
    }
}

impl<CharType, Output> IntoParser for DictHashMapParser<Output, CharType>
where
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

// Dictionary using trie
// implementation uses HashMap; O(1) search
#[derive(Debug, Clone, Default)]
pub struct DictHashMapParser<Output, CharType>
where
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    trie: HashTrieNode<CharType, Output>,
}

impl<Output, CharType> DictHashMapParser<Output, CharType>
where
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    pub fn new() -> Self {
        Self {
            trie: HashTrieNode::new(),
        }
    }

    pub fn insert<CharIter>(&mut self, key: CharIter, output: Output) -> Option<Output>
    where
        CharIter: Iterator<Item = CharType>,
    {
        self.trie.insert(key, output)
    }
}

impl<Output, CharType, It> Parser<It> for DictHashMapParser<Output, CharType>
where
    It: InputIteratorTrait + Iterator<Item = CharType> + Clone,
    CharType: Hash + Eq,
    Output: Clone + Tuple,
{
    type Output = Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.trie
            .match_longest(it.clone(), ParseResult { output: None, it })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success1() {
        let mut dict: DictHashMapParser<(i32,), char> = DictHashMapParser::new();
        dict.insert("hello_world".chars(), (0,));
        dict.insert("hello_trie".chars(), (1,));
        dict.insert("hello".chars(), (2,));

        let str = "hello_";
        let res = dict.parse(str.chars());
        let rest: String = res.it.collect();

        assert_eq!(res.output, Some((2,)));
        assert_eq!(rest, "_");
    }
    #[test]
    fn success2() {
        let mut dict: DictHashMapParser<(i32,), char> = DictHashMapParser::new();
        dict.insert("hello_world".chars(), (0,));
        dict.insert("hello_trie".chars(), (1,));
        dict.insert("hello".chars(), (2,));

        let str = "hello_world2";
        let res = dict.parse(str.chars());
        let rest: String = res.it.collect();

        assert_eq!(res.output, Some((0,)));
        assert_eq!(rest, "2");
    }
    #[test]
    fn success3() {
        let mut dict: DictHashMapParser<(i32,), char> = DictHashMapParser::new();
        dict.insert("hello_world".chars(), (0,));
        dict.insert("hello_trie".chars(), (1,));
        dict.insert("hello".chars(), (2,));

        let str = "hello_trie4";
        let res = dict.parse(str.chars());
        let rest: String = res.it.collect();

        assert_eq!(res.output, Some((1,)));
        assert_eq!(rest, "4");
    }
    #[test]
    fn fail1() {
        let mut dict: DictHashMapParser<(i32,), char> = DictHashMapParser::new();
        dict.insert("hello_world".chars(), (0,));
        dict.insert("hello_trie".chars(), (1,));
        dict.insert("hello".chars(), (2,));

        let str = "hell";
        let res = dict.parse(str.chars());
        let rest: String = res.it.collect();

        assert_eq!(res.output, None);
        assert_eq!(rest, "hell");
    }
}
