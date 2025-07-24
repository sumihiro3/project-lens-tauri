#[cfg(test)]
mod tests {
    use super::*;
    use crate::docker::service::DockerService;
    
    #[tokio::test]
    async fn test_docker_service_creation() {
        let _service = DockerService::new("test-container");
        // プライベートフィールドのテストはスキップ
        
        let _default_service = DockerService::default();
        // プライベートフィールドのテストはスキップ
    }
    
    // 注意: 以下のテストはDockerがインストールされている環境でのみ成功します
    // CI環境では条件付きでスキップするか、モックを使用することを検討してください
    
    #[tokio::test]
    #[ignore = "Requires Docker to be installed"]
    async fn test_is_docker_available() {
        let service = DockerService::default();
        let result = service.is_docker_available().await;
        
        // このテストはDockerがインストールされている環境でのみ成功します
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    #[ignore = "Requires Docker to be installed"]
    async fn test_get_docker_version() {
        let service = DockerService::default();
        let result = service.get_docker_version().await;
        
        // このテストはDockerがインストールされている環境でのみ成功します
        assert!(result.is_ok());
        if let Ok(version) = result {
            assert!(version.contains("Docker"));
        }
    }
    
    #[tokio::test]
    #[ignore = "Requires Docker to be running"]
    async fn test_is_docker_running() {
        let service = DockerService::default();
        let result = service.is_docker_running().await;
        
        // このテストはDockerが実行されている環境でのみ成功します
        assert!(result.is_ok());
        if let Ok(running) = result {
            assert!(running);
        }
    }
}