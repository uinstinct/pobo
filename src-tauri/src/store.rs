use std::path::PathBuf;

use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{with_store, StoreBuilder, StoreCollection};

pub trait PoboStore {
    fn get_path() -> PathBuf;

    fn load(app_handle: AppHandle) {
        let mut store = StoreBuilder::new(app_handle, Self::get_path()).build();
        let _ = store.load();
    }

    fn set<T>(app_handle: &AppHandle, key: &str, value: T)
    where
        T: Serialize,
    {
        let stores = app_handle.state::<StoreCollection<Wry>>();
        let key = key.to_string();
        let value = serde_json::to_value(value).unwrap();

        with_store(
            app_handle.clone(),
            stores,
            SessionStore::get_path(),
            |store| {
                store.insert(key, value)?;
                store.save()
            },
        )
        .unwrap();
    }

    fn get(app_handle: &AppHandle, key: &str) -> Option<Value> {
        let stores = app_handle.state::<StoreCollection<Wry>>();
        let key = key.to_string();

        with_store(
            app_handle.clone(),
            stores,
            SessionStore::get_path(),
            |store| Ok(store.get(key).cloned()),
        )
        .unwrap()
    }
}

pub struct SessionStore {}

impl SessionStore {
    const SESSION_STORE_PATH: &'static str = "./session-store.json";

    const TIMER_SECONDS_KEY: &'static str = "CurrentTimerSeconds";
    const SESSION_COUNTER_KEY: &'static str = "SessionCounter";
    const TIMESTAMP_KEY: &'static str = "TimeStamp";
}

impl PoboStore for SessionStore {
    fn get_path() -> PathBuf {
        SessionStore::SESSION_STORE_PATH.parse().unwrap()
    }
}

impl SessionStore {
    pub fn set_timer_seconds(app_handle: &AppHandle, timer_seconds: u64) {
        SessionStore::set(&app_handle, SessionStore::TIMER_SECONDS_KEY, timer_seconds);
    }

    pub fn get_timer_seconds(app_handle: &AppHandle) -> Option<u64> {
        SessionStore::get(&app_handle, SessionStore::TIMER_SECONDS_KEY)
            .and_then(|value| serde_json::from_value(value).ok())
    }

    pub fn set_session_counter(app_handle: &AppHandle, counter: u64) {
        SessionStore::set(&app_handle, SessionStore::SESSION_COUNTER_KEY, counter);
    }

    pub fn get_session_counter(app_handle: &AppHandle) -> Option<u64> {
        SessionStore::get(&app_handle, SessionStore::SESSION_COUNTER_KEY)
            .and_then(|value| serde_json::from_value(value).ok())
    }

    pub fn set_timestamp(app_handle: &AppHandle) {
        SessionStore::set(
            &app_handle,
            SessionStore::TIMESTAMP_KEY,
            chrono::offset::Local::now(),
        );
    }

    pub fn get_timestamp(app_handle: &AppHandle) -> Option<chrono::DateTime<chrono::Local>> {
        SessionStore::get(&app_handle, SessionStore::TIMESTAMP_KEY)
            .and_then(|value| serde_json::from_value(value).ok())
    }
}
