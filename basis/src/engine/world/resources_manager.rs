use anymap::{any::Any, Map};

use crate::prelude::*;

pub struct ResourcesManager {
    resources: Map<dyn Any>,
}

impl Default for ResourcesManager {
    fn default() -> Self {
        Self {
            resources: Map::new(),
        }
    }
}

impl ResourcesManager {
    pub fn add<T: Resource>(&mut self, resource: T) {
        self.resources.entry::<T>().or_insert(resource);
    }

    pub fn get<T: Resource>(&self) -> &T {
        self.resources
            .get::<T>()
            .expect("To be able to get determined resource")
    }

    pub fn get_mut<T: Resource>(&mut self) -> &mut T {
        self.resources
            .get_mut::<T>()
            .expect("To be able to get determined resource")
    }
}
