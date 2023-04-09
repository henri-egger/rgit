use serde::{Deserialize, Serialize};

pub trait Storable {
    fn store(&self);
}

#[derive(Serialize, Deserialize)]
pub struct Stored<T: Storable>(T);

impl<T: Storable> Stored<T> {
    pub fn new(storable: T) -> Stored<T> {
        storable.store();
        Stored(storable)
    }

    pub fn value(&self) -> &T {
        &self.0
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
