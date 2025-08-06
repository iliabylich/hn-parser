pub(crate) struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub(crate) fn try_new(mut v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            return None;
        }
        let first = v.remove(0);
        Some(Self { first, rest: v })
    }

    pub(crate) fn into_vec(self) -> Vec<T> {
        let NonEmptyVec { first, mut rest } = self;
        let mut out = Vec::with_capacity(1 + rest.len());
        out.push(first);
        out.append(&mut rest);
        out
    }
}
