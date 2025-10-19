use std::ops::Deref;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::cache::Cache;
use crate::config::AppConfig;
use crate::repo::Repository;

/// `AppState` is a cloneable wrapper around `AppStateInner` using `Arc`.
#[derive(Clone, Debug)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(config: AppConfig, repo: Repository, cache: Cache) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
                repo,
                cache,
            }),
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// `AppStateInner` has not to be Clone because `AppState` is the one being cloned.
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub config: AppConfig,
    pub repo: Repository,
    pub cache: Cache,
}
