/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut iterators: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut inner_iterator = iterators.next();
    std::iter::from_fn(move || {
        loop {
            match &mut inner_iterator {
                Some(iterator) => match iterator.next() {
                    Some(item) => return Some(item),
                    None => inner_iterator = iterators.next(),
                },
                None => return None,
            }
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || {
        loop {
            match iter.next() {
                Some(item) => {
                    if predicate(&item) {
                        return Some(item);
                    }
                }
                None => return None,
            }
        }
    })
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut length = 0;
    while iter.next().is_some() {
        length += 1;
    }
    length
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || iter.next().map(&function))
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for item in iter {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for item in reverse(iter) {
        acc = function(acc, item);
    }
    acc
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}
