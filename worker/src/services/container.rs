use std::collections::HashMap;
use std::time::Duration;
use std::process::Stdio;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tokio::process::Command;

#[derive(Debug)]
struct Cube {
    id: String,
    user_id: String,
    name: String,
    cube_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileObject {
    name: String,
    content: String,
}

struct ContainerService {
    socket_manager: SocketConnectionManager,
    s3_service: S3Service,
    database: DatabaseService,
    image: String,
    network_name: String,
}

impl ContainerService {
    fn new(
        socket_manager: SocketConnectionManager,
        s3_service: S3Service,
        database: DatabaseService,
        image: String,
        network_name: String,
    ) -> Self {
        Self {
            socket_manager,
            s3_service,
            database,
            image,
            network_name,
        }
    }

    async fn create_container(&self, cube: &Cube, reinit: u8) -> Result<(u16, u16, String)> {
        let container_name = self.get_container_name(&cube.id);

        let port1 = self.socket_manager.find_express_port();
        let port2 = self.socket_manager.find_project_port();

        let args = vec![
            "run",
            "-d",
            "--name", &container_name,
            "--hostname", &container_name,
            "-p", &format!("{}:{}", port1, port1),
            "-p", &format!("{}:{}", port2, port2),
            &self.image,
            &port1.to_string(),
            &port2.to_string(),
            &cube.name,
            &cube.cube_type,
            &cube.id,
            &reinit.to_string(),
            "--y"
        ];

        Command::new("docker")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        Command::new("docker")
            .args(["network", "connect", &self.network_name, &container_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        self.socket_manager.create_socket_connection(port1, port2, cube);

        Ok((port1, port2, container_name))
    }

    async fn burn_container(&self, cube_id: &str, user_id: &str) -> Result<()> {
        let container_name = self.get_container_name(cube_id);

        self.database.update_status(cube_id, "preparing")?;

        let port = self.socket_manager.get_cube_port(cube_id)?;
        let files = self.fetch_container_fs(&container_name, port).await?;

        if !self.s3_service.upload_files(&files, user_id, cube_id)? {
            return Err(anyhow!("Couldn't back up container"));
        }

        Command::new("docker")
            .args(["stop", &container_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        Command::new("docker")
            .args(["rm", &container_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        self.socket_manager.remove_socket(cube_id);
        self.database.update_status(cube_id, "stopped")?;

        Ok(())
    }

    async fn fetch_container_fs(&self, container_name: &str, port: u16) -> Result<Vec<FileObject>> {
        let url = format!("http://{}:{}/project", container_name, port);
        let client = Client::new();
        let response = client.get(&url).send().await?;
        let data = response.json().await?;
        Ok(data)
    }

    async fn reinit_container(&self, cube: &Cube) -> Result<()> {
        let container_name = self.get_container_name(&cube.id);
        let (port1, _, _) = self.create_container(cube, 1).await?;

        let files = self.s3_service.fetch_files(&cube.user_id, &cube.id)?;

        Command::new("docker")
            .args(["start", &container_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        sleep(Duration::from_secs(7)).await;

        let url = format!("http://{}:{}/reinit", container_name, port1);
        let client = Client::new();
        client.put(&url).json(&files).send().await?;

        sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    fn get_container_name(&self, cube_id: &str) -> String {
        format!("project-{}", cube_id)
    }
}

struct SocketConnectionManager;
impl SocketConnectionManager {
    fn find_express_port(&self) -> u16 { 3000 }
    fn find_project_port(&self) -> u16 { 3001 }
    fn create_socket_connection(&self, _p1: u16, _p2: u16, _cube: &Cube) {}
    fn remove_socket(&self, _cube_id: &str) {}
    fn get_cube_port(&self, _cube_id: &str) -> Result<u16> { Ok(3000) }
}

struct S3Service;
impl S3Service {
    fn upload_files(&self, _files: &[FileObject], _user_id: &str, _cube_id: &str) -> Result<bool> { Ok(true) }
    fn fetch_files(&self, _user_id: &str, _cube_id: &str) -> Result<Vec<FileObject>> { Ok(vec![]) }
}

struct DatabaseService;
impl DatabaseService {
    fn update_status(&self, _cube_id: &str, _status: &str) -> Result<()> { Ok(()) }
}
