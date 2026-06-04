#![allow(unused)]
#![allow(dead_code)]
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

struct Consent {
    text: String,
    granted: bool,
}

struct RestReady {
    consents: Vec<Consent>,
}

struct KafkaReady;
struct Reservation;

struct Provision<S> {
    user_id: u128,
    state: S,
}

struct Processed;

impl Provision<Reservation> {
    fn new(user_id: u128) -> Self {
        Self {
            user_id,
            state: Reservation,
        }
    }

    fn kafka_ready(&self) -> Provision<KafkaReady> {
        Provision {
            user_id: self.user_id,
            state: KafkaReady,
        }
    }

    fn rest_ready(&self, consents: Vec<Consent>) -> Provision<RestReady> {
        Provision {
            user_id: self.user_id,
            state: RestReady { consents },
        }
    }
}

impl Provision<KafkaReady> {
    fn rest_ready(&self) -> Provision<Processed> {
        Provision {
            user_id: self.user_id,
            state: Processed,
        }
    }
}

impl Provision<RestReady> {
    fn kafka_ready(&self) -> Provision<Processed> {
        Provision {
            user_id: self.user_id,
            state: Processed,
        }
    }
}

trait CacheEntry {
    fn user_id(&self) -> u128;
    fn into_wrapper(self) -> ProvisionWrapper;
}

enum ProvisionWrapper {
    Reservation { provision: Provision<Reservation> },
    KafkaReady { provision: Provision<KafkaReady> },
    RestReady { provision: Provision<RestReady> },
}

impl CacheEntry for Provision<Reservation> {
    fn user_id(&self) -> u128 {
        self.user_id
    }
    fn into_wrapper(self) -> ProvisionWrapper {
        ProvisionWrapper::Reservation { provision: self }
    }
}

impl CacheEntry for Provision<KafkaReady> {
    fn user_id(&self) -> u128 {
        self.user_id
    }
    fn into_wrapper(self) -> ProvisionWrapper {
        ProvisionWrapper::KafkaReady { provision: self }
    }
}

impl CacheEntry for Provision<RestReady> {
    fn user_id(&self) -> u128 {
        self.user_id
    }
    fn into_wrapper(self) -> ProvisionWrapper {
        ProvisionWrapper::RestReady { provision: self }
    }
}

struct ProvisionCache {
    cache: Arc<RwLock<HashMap<u128, ProvisionWrapper>>>,
}

impl ProvisionCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert<S: CacheEntry>(&self, provision: S) {
        let user_id = provision.user_id();
        self.cache
            .write()
            .unwrap()
            .insert(user_id, provision.into_wrapper());
    }

    pub fn take(&self, user_id: u128) -> Option<ProvisionWrapper> {
        self.cache.write().unwrap().remove(&user_id)
    }
}
