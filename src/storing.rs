use serde::{Deserialize, Serialize};

pub trait Storable {
    fn store(&self);
}

#[derive(Serialize, Deserialize)]
pub struct Stored<T: Storable + Sized>(T);

impl<T: Storable> Stored<T> {
    pub fn new(storable: T) -> Stored<T> {
        storable.store();
        Stored(storable)
    }

    pub fn value(&self) -> &T {
        &self.0
    }

    pub fn into_value(self) -> T {
        self.0
    }
}
