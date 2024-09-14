use crate::raft_atinyvectors::atinyvectors::{
    SearchDTOManagerWrapper, SpaceDTOManagerWrapper, VectorDTOManagerWrapper, VersionDTOManagerWrapper,
};

#[derive(Clone, Debug)]
pub struct ATinyVectorsBO {
    search_manager: SearchDTOManagerWrapper,
    space_manager: SpaceDTOManagerWrapper,
    vector_manager: VectorDTOManagerWrapper,
    version_manager: VersionDTOManagerWrapper,
}

impl ATinyVectorsBO {
    pub fn new() -> Self {
        Self {
            search_manager: SearchDTOManagerWrapper::new(),
            space_manager: SpaceDTOManagerWrapper::new(),
            vector_manager: VectorDTOManagerWrapper::new(),
            version_manager: VersionDTOManagerWrapper::new(),
        }
    }

    // space manager api
    pub fn create_space(&self, json: &str) -> Result<(), String> {
        self.space_manager.create_space(json);
        Ok(())
    }

    pub fn get_space_by_id(&self, space_id: i32) -> Result<String, String> {
        self.space_manager.get_by_space_id(space_id)
    }

    pub fn get_space_by_name(&self, space_name: &str) -> Result<String, String> {
        self.space_manager.get_by_space_name(space_name)
    }

    pub fn get_space_lists(&self) -> Result<String, String> {
        self.space_manager.get_lists()
    }

    // version manager api
    pub fn create_version(&self, space_name: &str, json_str: &str) -> Result<(), String> {
        self.version_manager.create_version(space_name, json_str);
        Ok(())
    }

    pub fn get_version_by_id(&self, space_name: &str, version_id: i32) -> Result<String, String> {
        self.version_manager.get_by_version_id(space_name, version_id)
    }

    pub fn get_version_by_name(&self, space_name: &str, version_name: &str) -> Result<String, String> {
        self.version_manager.get_by_version_name(space_name, version_name)
    }

    pub fn get_default_version(&self, space_name: &str) -> Result<String, String> {
        self.version_manager.get_default_version(space_name)
    }

    pub fn get_version_lists(&self, space_name: &str) -> Result<String, String> {
        self.version_manager.get_lists(space_name)
    }

    // vector manager api
    pub fn upsert_vectors(&self, space_name: &str, version_id: i32, json: &str) -> Result<(), String> {
        self.vector_manager.upsert_vectors(space_name, version_id, json);
        Ok(())
    }
    
    pub fn get_vectors_by_version_id(&self, version_id: i32) -> Result<String, String> {
        self.vector_manager.get_vectors_by_version_id(version_id)
    }
    
    // search manager api
    pub fn search(&self, space_name: &str, query_json: &str, k: usize) -> Result<String, String> {
        self.search_manager.search(space_name, query_json, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atinyvectors_bo() {
        // ATinyVectorsBO 인스턴스 생성
        let bo = ATinyVectorsBO::new();

        // 공간 생성 JSON 정의
        let create_space_json = r#"
        {
            "name": "spacename",
            "dimension": 128,
            "metric": "cosine",
            "hnsw_config": {
                "ef_construct": 123
            },
            "quantization_config": {
                "scalar": {
                    "type": "int8",
                    "quantile": 0.99,
                    "always_ram": true
                }
            },
            "dense": {
                "dimension": 1536,
                "metric": "Cosine",
                "hnsw_config": {
                    "m": 32,
                    "ef_construct": 123
                },
                "quantization_config": {
                    "scalar": {
                        "type": "int8",
                        "quantile": 0.8
                    }
                }
            },
            "sparse": {
                "metric": "Cosine"
            },
            "indexes": {
                "index1": {
                    "dimension": 4,
                    "metric": "Cosine",
                    "hnsw_config": {
                        "m": 20
                    },
                    "quantization_config": {
                        "scalar": {
                            "type": "int8",
                            "quantile": 0.6
                        }
                    }
                },
                "index2": {
                    "dimension": 4,
                    "metric": "Cosine",
                    "hnsw_config": {
                        "m": 20
                    },
                    "quantization_config": {
                        "scalar": {
                            "type": "int8",
                            "quantile": 0.6
                        }
                    }
                }
            }
        }"#;

        // 공간 생성
        let create_space_result = bo.create_space(create_space_json);
        assert!(
            create_space_result.is_ok(),
            "Failed to create space: {:?}",
            create_space_result.err()
        );
        tracing::info!("space created");

        // 버전 생성 JSON 정의
        let create_version_json = r#"
        {
            "name": "version1",
            "description": "Initial version",
            "tag": "v1.0",
            "is_default": true
        }"#;

        // 버전 생성
        let create_version_result = bo.create_version("spacename", create_version_json);
        assert!(
            create_version_result.is_ok(),
            "Failed to create version: {:?}",
            create_version_result.err()
        );
        tracing::info!("version created");

        // 버전 이름으로 버전 조회
        let get_version_result = bo.get_version_by_name("spacename", "version1");
        assert!(
            get_version_result.is_ok(),
            "Failed to get version by name: {:?}",
            get_version_result.err()
        );

        let version = get_version_result.unwrap();
        tracing::info!("Retrieved version: {}", version);
    }
}