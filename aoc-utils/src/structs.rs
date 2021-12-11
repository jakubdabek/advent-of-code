use std::cmp::Ordering;

#[derive(Debug)]
pub struct PriorityValue<T, P = usize> {
    pub priority: P,
    pub value: T,
}

impl<T, P: Ord> Eq for PriorityValue<T, P> {}

impl<T, P: Ord> PartialEq<Self> for PriorityValue<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T, P: Ord> PartialOrd<Self> for PriorityValue<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, P: Ord> Ord for PriorityValue<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
