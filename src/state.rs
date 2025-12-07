use std::ops::Deref;
use std::sync::Arc;

use crate::cache::Cache;
use crate::config::AppConfig;
use crate::jwt_codec::JwtCodec;
use crate::repo::Repo;

/// `AppState` is a cloneable wrapper around `AppStateInner` using `Arc`.
#[derive(Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(config: AppConfig, repo: Repo, cache: Cache, jwt_codec: JwtCodec) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
                repo,
                cache,
                jwt_codec,
            }),
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }

    pub fn repo(&self) -> &Repo {
        &self.inner.repo
    }

    pub fn cache(&self) -> &Cache {
        &self.inner.cache
    }

    pub fn jwt_codec(&self) -> &JwtCodec {
        &self.inner.jwt_codec
    }
}

/// `AppStateInner` has not to be Clone because `AppState` is the one being cloned.
pub struct AppStateInner {
    config: AppConfig,
    repo: Repo,
    cache: Cache,
    jwt_codec: JwtCodec,
}
