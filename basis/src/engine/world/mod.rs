use std::sync::Mutex;

use anymap::{any::Any, Map};

mod resources_manager;

use crate::prelude::*;
pub use resources_manager::*;

// Generic storage for components of type T
pub struct ComponentStorage<T: Component> {
    data: Vec<Option<T>>,
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, entity: usize, component: T) {
        while self.data.len() <= entity {
            self.data.push(None);
        }
        self.data[entity] = Some(component);
    }

    pub fn get(&self, entity: usize) -> Option<&T> {
        self.data.get(entity).and_then(|opt| opt.as_ref())
    }

    pub fn get_mut(&mut self, entity: usize) -> Option<&mut T> {
        self.data.get_mut(entity).and_then(|opt| opt.as_mut())
    }

    pub fn remove(&mut self, entity: usize) {
        if let Some(opt) = self.data.get_mut(entity) {
            *opt = None;
        }
    }
}

pub type Entity = usize;

#[derive(Default)]
pub struct EntityManager {
    next_id: Entity,
    free_ids: Vec<Entity>,
}

impl EntityManager {
    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.free_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.free_ids.push(entity);
    }

    pub fn active_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        (0..self.next_id).filter(move |&id| !self.free_ids.contains(&id))
    }
}

pub struct World {
    pub entity_manager: EntityManager,
    component_storages: Map<dyn Any>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_storages: Map::new(),
        }
    }
}

impl World {
    pub fn spawn(&mut self) -> Entity {
        self.entity_manager.create()
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        let entity = self.spawn();
        let storage = self
            .component_storages
            .entry::<Mutex<ComponentStorage<T>>>()
            .or_insert_with(|| Mutex::new(ComponentStorage::new()));
        storage.get_mut().unwrap().insert(entity, component);
    }

    pub fn add_entity_component<T: Component>(&mut self, entity: Entity, component: T) {
        let storage = self
            .component_storages
            .entry::<Mutex<ComponentStorage<T>>>()
            .or_insert_with(|| Mutex::new(ComponentStorage::new()));
        storage.get_mut().unwrap().insert(entity, component);
    }

    pub fn with_components_1<A, F>(&self, entity: usize, f: F)
    where
        A: Component,
        F: FnOnce(Option<&A>),
    {
        let a_storage = self.component_storages.get::<Mutex<ComponentStorage<A>>>();
        let a_guard = a_storage.map(|s| s.lock().unwrap());
        let a = a_guard.as_ref().and_then(|g| g.get(entity));
        f(a);
    }

    pub fn with_components_mut_1<A, F>(&self, entity: usize, f: F)
    where
        A: Component,
        F: FnOnce(Option<&mut A>),
    {
        let a_storage = self.component_storages.get::<Mutex<ComponentStorage<A>>>();
        let mut a_guard = a_storage.map(|s| s.lock().unwrap());
        let a = a_guard.as_mut().and_then(|g| g.get_mut(entity));
        f(a);
    }

    pub fn with_components_3<A, B, C, F>(&self, entity: usize, f: F)
    where
        A: Component,
        B: Component,
        C: Component,
        F: FnOnce(Option<&A>, Option<&B>, Option<&C>),
    {
        let a_storage = self.component_storages.get::<Mutex<ComponentStorage<A>>>();
        let a_guard = a_storage.map(|s| s.lock().unwrap());
        let a = a_guard.as_ref().and_then(|g| g.get(entity));

        let b_storage = self.component_storages.get::<Mutex<ComponentStorage<B>>>();
        let b_guard = b_storage.map(|s| s.lock().unwrap());
        let b = b_guard.as_ref().and_then(|g| g.get(entity));

        let c_storage = self.component_storages.get::<Mutex<ComponentStorage<C>>>();
        let c_guard = c_storage.map(|s| s.lock().unwrap());
        let c = c_guard.as_ref().and_then(|g| g.get(entity));

        f(a, b, c);
    }

    pub fn with_components_mut_2<A, B, F>(&self, entity: usize, f: F)
    where
        A: Component,
        B: Component,
        F: FnOnce(Option<&mut A>, Option<&mut B>),
    {
        let a_storage = self.component_storages.get::<Mutex<ComponentStorage<A>>>();
        let b_storage = self.component_storages.get::<Mutex<ComponentStorage<B>>>();

        let mut a_guard = a_storage.map(|s| s.lock().unwrap());
        let mut b_guard = b_storage.map(|s| s.lock().unwrap());

        let a = a_guard.as_mut().and_then(|g| g.get_mut(entity));
        let b = b_guard.as_mut().and_then(|g| g.get_mut(entity));

        f(a, b);
    }

    pub fn remove_component<T: Component>(&mut self, entity: usize) {
        if let Some(storage) = self.component_storages.get_mut::<ComponentStorage<T>>() {
            storage.remove(entity);
        }
    }

    pub fn destroy_entity(&mut self, entity: usize) {
        self.entity_manager.destroy(entity);
        // Note: Component cleanup could be added here if needed
    }
}
