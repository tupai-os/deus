use core::{
    cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering},
    marker::PhantomData,
};
use alloc::collections::BTreeMap;

pub struct Id<T>(usize, PhantomData<T>);

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0, PhantomData)
    }
}
impl<T> Eq for Id<T> {}
impl<T> PartialEq<Id<T>> for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        self.0 == other.0
    }
}
impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub struct Store<T> {
    items: BTreeMap<Id<T>, T>,
    id_counter: usize,
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            id_counter: 0,
        }
    }
}

impl<T> Store<T> {
    fn new_id(&mut self) -> Id<T> {
        self.id_counter += 1;
        Id(self.id_counter, PhantomData)
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.items
            .get(&id)
            .expect("ID no longer valid")
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.items
            .get_mut(&id)
            .expect("ID no longer valid")
    }

    pub fn insert(&mut self, item: T) -> Id<T> {
        let id = self.new_id();
        self.items.insert(id, item);
        id
    }

    pub fn remove(&mut self, id: Id<T>) {
        self.items.remove(&id);
    }
}
