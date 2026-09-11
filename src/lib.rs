pub trait TupleList {
    const LEN: usize;

    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    fn len(&self) -> usize {
        Self::LEN
    }
}

pub trait Append<T> {
    type Output: TupleList;

    fn append(self, elem: T) -> Self::Output;
}

pub trait Extend<L> {
    type Output: TupleList;

    fn extend(self, list: L) -> Self::Output;
}

impl TupleList for () {
    const LEN: usize = 0;
}

impl<Head, Tail> TupleList for (Head, Tail)
where
    Tail: TupleList,
{
    const LEN: usize = Tail::LEN + 1;
}

impl<T> Append<T> for () {
    type Output = (T, ());

    fn append(self, elem: T) -> Self::Output {
        (elem, ())
    }
}

impl<T, Head, Tail> Append<T> for (Head, Tail)
where
    Tail: Append<T>,
{
    type Output = (Head, Tail::Output);

    fn append(self, elem: T) -> Self::Output {
        (self.0, self.1.append(elem))
    }
}

impl<T> Extend<T> for ()
where
    T: TupleList,
{
    type Output = T;

    fn extend(self, list: T) -> Self::Output {
        list
    }
}

impl<T, Head, Tail> Extend<T> for (Head, Tail)
where
    Tail: Extend<T>,
{
    type Output = (Head, Tail::Output);

    fn extend(self, list: T) -> Self::Output {
        (self.0, self.1.extend(list))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let head = ();
        let first = head.append(42);
        let second = first.append(42.);

        assert_eq!(first, (42, ()));
        assert_eq!(second, (42, (42., ())));
    }

    #[test]
    fn test_extend() {
        let head = ();
        let first = head.append(42);
        let second = first.append(42.);

        let list = second.extend(first);

        assert_eq!(list, (42, (42., (42, ()))));
    }

    #[test]
    fn test_len() {
        let head = ();

        let first = head.append(42);
        let second = first.append(42.);

        assert_eq!(head.len(), 0);
        assert_eq!(first.len(), 1);
        assert_eq!(second.len(), 2);
    }
}
