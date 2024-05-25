use std::collections::BTreeMap;
use std::iter::Iterator;
use std::marker::PhantomData;

use super::parser::Parser;
use super::result::ParseResult;
use super::tuple::Tuple;

use rusty_parser_derive::ParserHelper;

// Build Trie using BTreeMap
// Can be implemented by HashMap ( commented out below ),
#[derive(Debug, Clone)]
struct BTreeTrieNode<CharType, Output>
where
    CharType: Ord,
    Output: Clone + Tuple,
{
    children: BTreeMap<CharType, BTreeTrieNode<CharType, Output>>,
    output: Option<Output>,
}

impl<CharType, Output> BTreeTrieNode<CharType, Output>
where
    CharType: Ord,
    Output: Clone + Tuple,
{
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            output: None,
        }
    }

    // returns the old value if the key already exists
    pub fn insert<CharIter>(&mut self, mut key: CharIter, output: Output) -> Option<Output>
    where
        CharIter: Iterator<Item = CharType>,
    {
        if let Some(key_val) = key.next() {
            self.children
                .entry(key_val)
                .or_insert(BTreeTrieNode::new())
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

// Dictionary using trie
// implementation uses BTreeMap; O(log(N)) search
#[derive(Debug, Clone, ParserHelper)]
pub struct DictBTreeParser<Output, CharType, It>
where
    Output: Clone + Tuple,
    CharType: Ord,
    It: Iterator<Item = CharType> + Clone,
{
    trie: BTreeTrieNode<CharType, Output>,
    _phantom: PhantomData<It>,
}

impl<Output, CharType, It> DictBTreeParser<Output, CharType, It>
where
    Output: Clone + Tuple,
    CharType: Ord,
    It: Iterator<Item = CharType> + Clone,
{
    pub fn new() -> Self {
        Self {
            trie: BTreeTrieNode::new(),
            _phantom: PhantomData,
        }
    }

    pub fn insert<CharIter>(&mut self, key: CharIter, output: Output) -> Option<Output>
    where
        CharIter: Iterator<Item = CharType>,
    {
        self.trie.insert(key, output)
    }
}

impl<Output, CharType, It> Parser<It> for DictBTreeParser<Output, CharType, It>
where
    Output: Clone + Tuple,
    CharType: Ord,
    It: Iterator<Item = CharType> + Clone,
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
        let mut dict: DictBTreeParser<(i32,), char, _> = DictBTreeParser::new();
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
        let mut dict: DictBTreeParser<(i32,), char, _> = DictBTreeParser::new();
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
        let mut dict: DictBTreeParser<(i32,), char, _> = DictBTreeParser::new();
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
        let mut dict: DictBTreeParser<(i32,), char, _> = DictBTreeParser::new();
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
