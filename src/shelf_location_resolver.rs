use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::kachaka_api;
use crate::KachakaApiClient;

struct LayoutCollection<T> {
    items: Vec<T>,
    id_index: HashMap<String, usize>,
    name_index: HashMap<String, usize>,
}

impl<T> LayoutCollection<T> {
    pub fn new(
        items: Vec<T>,
        get_id: impl Fn(&T) -> String,
        get_name: impl Fn(&T) -> String,
    ) -> Self {
        let mut id_index = HashMap::new();
        let mut name_index = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            id_index.insert(get_id(item), idx);
            name_index.insert(get_name(item), idx);
        }

        Self {
            items,
            id_index,
            name_index,
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<&T> {
        self.id_index.get(id).map(|&idx| &self.items[idx])
    }

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        self.name_index.get(name).map(|&idx| &self.items[idx])
    }
}

struct ShelfLocationResolverState {
    locations_collection: LayoutCollection<kachaka_api::Location>,
    shelves_collection: LayoutCollection<kachaka_api::Shelf>,
}

pub struct ShelfLocationResolver {
    kachaka_api_client: KachakaApiClient,
    state: Arc<RwLock<ShelfLocationResolverState>>,
}

impl ShelfLocationResolver {
    pub fn new(kachaka_api_client: KachakaApiClient) -> Self {
        Self {
            kachaka_api_client,
            state: Arc::new(RwLock::new(ShelfLocationResolverState {
                locations_collection: LayoutCollection::new(
                    Vec::new(),
                    |location| location.id.clone(),
                    |location| location.name.clone(),
                ),
                shelves_collection: LayoutCollection::new(
                    Vec::new(),
                    |shelf| shelf.id.clone(),
                    |shelf| shelf.name.clone(),
                ),
            })),
        }
    }

    pub async fn run_update_loop(&self) {
        let mut locations_stream = self.kachaka_api_client.clone().watch_locations().await;
        let mut shelves_stream = self.kachaka_api_client.clone().watch_shelves().await;

        loop {
            tokio::select! {
                Some(locations) = locations_stream.next() => {
                    let mut state = self.state.write().await;
                    state.locations_collection = LayoutCollection::new(
                        locations.unwrap(),
                        |location| location.id.clone(),
                        |location| location.name.clone(),
                    );
                }
                Some(shelves) = shelves_stream.next() => {
                    let mut state = self.state.write().await;
                    state.shelves_collection = LayoutCollection::new(
                        shelves.unwrap(),
                        |shelf| shelf.id.clone(),
                        |shelf| shelf.name.clone(),
                    );
                }
            }
        }
    }

    pub async fn get_location_by_id(&self, id: &str) -> Option<kachaka_api::Location> {
        let state = self.state.read().await;
        state.locations_collection.get_by_id(id).cloned()
    }

    pub async fn get_location_by_name(&self, name: &str) -> Option<kachaka_api::Location> {
        let state = self.state.read().await;
        state.locations_collection.get_by_name(name).cloned()
    }

    pub async fn get_all_locations(&self) -> Vec<kachaka_api::Location> {
        self.state.read().await.locations_collection.items.clone()
    }

    pub async fn get_shelf_by_id(&self, id: &str) -> Option<kachaka_api::Shelf> {
        let state = self.state.read().await;
        state.shelves_collection.get_by_id(id).cloned()
    }

    pub async fn get_shelf_by_name(&self, name: &str) -> Option<kachaka_api::Shelf> {
        let state = self.state.read().await;
        state.shelves_collection.get_by_name(name).cloned()
    }

    pub async fn get_all_shelves(&self) -> Vec<kachaka_api::Shelf> {
        self.state.read().await.shelves_collection.items.clone()
    }
}
