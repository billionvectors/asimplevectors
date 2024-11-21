use crate::atinyvectors::search::SearchServiceManagerWrapper;
use crate::atinyvectors::space::SpaceServiceManagerWrapper;
use crate::atinyvectors::version::VersionServiceManagerWrapper;
use crate::atinyvectors::vector::VectorServiceManagerWrapper;
use crate::atinyvectors::snapshot::SnapshotServiceManagerWrapper;
use crate::atinyvectors::rbac_token::RbacTokenServiceManagerWrapper;
use crate::atinyvectors::idcache::IdCacheManagerWrapper;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ATinyVectorsBO {
    pub search: Arc<SearchServiceManagerWrapper>,
    pub space: Arc<SpaceServiceManagerWrapper>,
    pub vector: Arc<VectorServiceManagerWrapper>,
    pub version: Arc<VersionServiceManagerWrapper>,
    pub snapshot: Arc<SnapshotServiceManagerWrapper>,
    pub rbac_token: Arc<RbacTokenServiceManagerWrapper>,
    pub id_cache: Arc<IdCacheManagerWrapper>,
}

impl ATinyVectorsBO {
    pub fn new() -> Self {
        unsafe { super::atv_init() };

        Self {
            search: Arc::new(SearchServiceManagerWrapper::new()),
            space: Arc::new(SpaceServiceManagerWrapper::new()),
            vector: Arc::new(VectorServiceManagerWrapper::new()),
            version: Arc::new(VersionServiceManagerWrapper::new()),
            snapshot: Arc::new(SnapshotServiceManagerWrapper::new()),
            rbac_token: Arc::new(RbacTokenServiceManagerWrapper::new()),
            id_cache: Arc::new(IdCacheManagerWrapper::new()), // 초기화
        }
    }
}
