// Docker コンテナ管理
// MCP Server コンテナの起動・停止・状態確認を担当

use bollard::Docker;
use bollard::container::{ListContainersOptions, StartContainerOptions};
use bollard::models::*;

// 公開用の構造体定義
#[derive(Debug, Clone)]
pub struct ContainerStatus {
    pub name: String,
    pub state: String,
    pub is_running: bool,
}

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
}
use std::collections::HashMap;
use std::default::Default;

/// Docker コンテナマネージャー
/// MCP Server コンテナの管理を担当
pub struct ContainerManager {
    docker: Docker,
    container_name: String,
}

impl ContainerManager {
    /// 新しいコンテナマネージャーを作成
    pub async fn new(container_name: &str) -> Result<Self, bollard::errors::Error> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self {
            docker,
            container_name: container_name.to_string(),
        })
    }

    /// コンテナの状態を確認
    pub async fn check_container_status(&self) -> Result<bool, bollard::errors::Error> {
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), vec![self.container_name.clone()]);
        
        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };
        
        let containers = self.docker.list_containers(Some(options)).await?;
        
        if containers.is_empty() {
            return Ok(false);
        }
        
        // コンテナが存在する場合、実行中かどうかを確認
        let container = &containers[0];
        let status = container.state.as_deref().unwrap_or("").to_lowercase();
        
        Ok(status == "running")
    }

    /// コンテナを起動
    pub async fn start_container(&self) -> Result<(), bollard::errors::Error> {
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), vec![self.container_name.clone()]);
        
        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };
        
        let containers = self.docker.list_containers(Some(options)).await?;
        
        if containers.is_empty() {
            return Err(bollard::errors::Error::IOError { 
                err: std::io::Error::new(
                    std::io::ErrorKind::NotFound, 
                    format!("Container {} not found", self.container_name)
                ) 
            });
        }
        
        let container_id = containers[0].id.as_ref().unwrap();
        self.docker.start_container(container_id, None::<StartContainerOptions<String>>).await?;
        
        Ok(())
    }

    /// コンテナを停止
    pub async fn stop_container(&self) -> Result<(), bollard::errors::Error> {
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), vec![self.container_name.clone()]);
        
        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };
        
        let containers = self.docker.list_containers(Some(options)).await?;
        
        if containers.is_empty() {
            return Err(bollard::errors::Error::IOError { 
                err: std::io::Error::new(
                    std::io::ErrorKind::NotFound, 
                    format!("Container {} not found", self.container_name)
                ) 
            });
        }
        
        let container_id = containers[0].id.as_ref().unwrap();
        self.docker.stop_container(container_id, None).await?;
        
        Ok(())
    }
}