pub trait Append<T> {
    type Output;

    fn append(self, elem: T) -> Self::Output;
}

pub trait Extend<L> {
    type Output;

    fn extend(self, list: L) -> Self::Output;
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

impl<L> Extend<()> for L {
    type Output = L;

    fn extend(self, _list: ()) -> Self::Output {
        self
    }
}

impl<L, Head, Tail> Extend<(Head, Tail)> for L
where
    L: Append<Head>,
    L::Output: Extend<Tail>,
{
    type Output = <L::Output as Extend<Tail>>::Output;

    fn extend(self, list: (Head, Tail)) -> Self::Output {
        self.append(list.0).extend(list.1)
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
}
