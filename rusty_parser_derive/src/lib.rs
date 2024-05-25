use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// This macro automatically generates the following member functions for Parsers
//  - seq( self, rhs )
//  - or_( self, rhs )
//  - repeat( self, RangeBounds )
//  - void_( self )
//  - ref_( &self )
//  - boxed( self )     -> BoxedParser      Box<Parser> wrapper
//  - refcelled( self ) -> RefCelledParser  RefCell<Parser> wrapper
//  - rced( self )      -> RcedParser       Rc<Parser> wrapper
#[proc_macro_derive(ParserHelper)]
pub fn derive_result_void(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // TODO, find name of generic type for Parser<'IteratorName'>
    // currently hardcoded to 'It'

    let expanded = quote! {
        impl #impl_generics #name #ty_generics
        #where_clause
        Self: Parser<It>,
        {

          // map
          pub fn map<MapperType__, MapOutput__>(
            self,
            map: MapperType__,
          ) -> crate::wrapper::map::MapParser<Self, MapperType__, MapOutput__, It>
          where
            MapperType__: Fn(<Self as Parser<It>>::Output) -> MapOutput__,
            MapOutput__: crate::core::tuple::Tuple,
          {
            crate::wrapper::map::MapParser::new(self, map)
          }

          // seq
          pub fn seq<Rhs__>(
            self,
            rhs: Rhs__,
          ) -> crate::wrapper::seq::SeqParser<Self, Rhs__, It>
          where
            Rhs__: Parser<It>,
            <Self as Parser<It>>::Output: crate::wrapper::tuplemerge::AppendTupleToTuple<<Rhs__ as Parser<It>>::Output>,
            <<Self as Parser<It>>::Output as crate::wrapper::tuplemerge::AppendTupleToTuple<<Rhs__ as Parser<It>>::Output>>::Output: crate::core::tuple::Tuple,
          {
            crate::wrapper::seq::SeqParser::new(self, rhs)
          }

          // or
          pub fn or_<Rhs__>(
            self,
            rhs: Rhs__,
          ) -> crate::wrapper::or_::OrParser<Self, Rhs__, It>
          where
            Rhs__: Parser<It, Output = <Self as Parser<It>>::Output>,
          {
            crate::wrapper::or_::OrParser::new(self, rhs)
          }

          // repeat
          pub fn repeat<RangeType__, Idx__>(
            self,
            range: RangeType__,
          ) -> crate::wrapper::repeat::RepeatParser<Self, RangeType__, Idx__, It>
          where
            RangeType__: std::ops::RangeBounds<Idx__>,
            Idx__: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
            i32: PartialOrd + PartialEq + PartialOrd<Idx__> + PartialEq<Idx__>,
            <Self as Parser<It>>::Output: crate::wrapper::vecmerge::VectorOutputSpecialize,
            <<Self as Parser<It>>::Output as crate::wrapper::vecmerge::VectorOutputSpecialize>::Output: crate::core::tuple::Tuple,
          {
            crate::wrapper::repeat::RepeatParser::new(self, range)
          }

          // void
          pub fn void_(self) -> crate::wrapper::void::VoidParser<Self, It>
          {
            crate::wrapper::void::VoidParser::new(self)
          }

          // ref
          pub fn ref_<'life_of_self__>(&'life_of_self__ self) -> crate::wrapper::reference::ReferenceParser<'life_of_self__, Self, It>
          {
            crate::wrapper::reference::ReferenceParser::new(self)
          }

          // boxed
          pub fn boxed<'life_of_self__>(self)
          -> crate::wrapper::boxed::BoxedParser<'life_of_self__, <Self as Parser<It>>::Output, It>
          where Self: 'life_of_self__,
          {
            crate::wrapper::boxed::BoxedParser::new(self)
          }

          // refcelled
          pub fn refcelled(self)
          -> crate::wrapper::refcelled::RefCelledParser<Self, It>
          {
            crate::wrapper::refcelled::RefCelledParser::new(self)
          }

          // RCed
          pub fn rced(self)
          -> crate::wrapper::rced::RcedParser<Self, It>
          {
            crate::wrapper::rced::RcedParser::new(self)
          }
        }
    };

    TokenStream::from(expanded)
}
