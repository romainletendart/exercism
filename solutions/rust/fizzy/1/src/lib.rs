use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> Option<String>>,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Self
    where
        F: Fn(T) -> bool + 'static,
        S: ToString + 'static,
    {
        Self {
            matcher: Box::new(move |n: T| {
                if matcher(n) {
                    Some(subs.to_string())
                } else {
                    None
                }
            }),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Copy + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |n| {
            let result = self.matchers.iter().fold(String::new(), |result, matcher| {
                let mut new_result = result;
                if let Some(matched) = (matcher.matcher)(n) {
                    new_result.push_str(&matched);
                }
                new_result
            });
            if result.is_empty() {
                return n.to_string();
            }
            result
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Rem + From<u8> + ToString,
    <T as Rem>::Output: PartialEq<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
