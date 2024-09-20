use crate::atinyvectors::search::SearchDTOManagerWrapper;
use crate::atinyvectors::space::SpaceDTOManagerWrapper;
use crate::atinyvectors::version::VersionDTOManagerWrapper;
use crate::atinyvectors::vector::VectorDTOManagerWrapper;
use crate::atinyvectors::snapshot::SnapshotDTOManagerWrapper;
use crate::atinyvectors::rbac_token::RbacTokenDTOManagerWrapper;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ATinyVectorsBO {
    pub search: Arc<SearchDTOManagerWrapper>,
    pub space: Arc<SpaceDTOManagerWrapper>,
    pub vector: Arc<VectorDTOManagerWrapper>,
    pub version: Arc<VersionDTOManagerWrapper>,
    pub snapshot: Arc<SnapshotDTOManagerWrapper>,
    pub rbac_token: Arc<RbacTokenDTOManagerWrapper>,
}

impl ATinyVectorsBO {
    pub fn new() -> Self {
        unsafe { super::atv_init() };

        Self {
            search: Arc::new(SearchDTOManagerWrapper::new()),
            space: Arc::new(SpaceDTOManagerWrapper::new()),
            vector: Arc::new(VectorDTOManagerWrapper::new()),
            version: Arc::new(VersionDTOManagerWrapper::new()),
            snapshot: Arc::new(SnapshotDTOManagerWrapper::new()),
            rbac_token: Arc::new(RbacTokenDTOManagerWrapper::new()),
        }
    }
}