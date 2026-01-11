use anyhow::Result;
use directories::ProjectDirs;
use reality_core::{ProjectKind, RealityManifest, RealityProject, RealityRepo, RealityTarget, TargetKind};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const MANIFEST_FILE: &str = "manifest.reality.toml";

#[derive(Debug, Default)]
pub struct ManifestLoader;

impl ManifestLoader {
    fn config_path(&self) -> Result<PathBuf> {
        let proj = ProjectDirs::from("ai", "Phoenix-XR-Sleep-Lab", "RealityOrchestrator")
            .ok_or_else(|| anyhow::anyhow!("unable to resolve config directory"))?;
        Ok(proj.config_dir().join(MANIFEST_FILE))
    }

    pub fn load_or_init(&self) -> Result<RealityManifest> {
        let path = self.config_path()?;
        if path.exists() {
            let raw = fs::read_to_string(&path)?;
            let manifest: RealityManifest = toml::from_str(&raw)?;
            Ok(manifest)
        } else {
            let manifest = self.default_manifest();
            self.save_manifest(&manifest, false)?;
            Ok(manifest)
        }
    }

    pub fn write_default_manifest(&self, force: bool) -> Result<()> {
        let manifest = self.default_manifest();
        self.save_manifest(&manifest, force)
    }

    fn save_manifest(&self, manifest: &RealityManifest, force: bool) -> Result<()> {
        let path = self.config_path()?;
        if path.exists() && !force {
            anyhow::bail!("manifest already exists at {:?} (use --force to overwrite)", path);
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let raw = toml::to_string_pretty(manifest)?;
        fs::write(&path, raw)?;
        Ok(())
    }

    fn default_manifest(&self) -> RealityManifest {
        RealityManifest {
            id: Uuid::new_v4(),
            version: "0.1.0".to_string(),
            projects: vec![
                RealityProject {
                    name: "Reality.os".to_string(),
                    description: "Primary Reality.os orchestration repo".to_string(),
                    kind: ProjectKind::RealityOs,
                    repos: vec![RealityRepo {
                        name: "Reality.os".to_string(),
                        url: "https://github.com/Doctor0Evil/Reality.os.git".to_string(),
                        branch: Some("main".to_string()),
                        path: Some("Reality.os".to_string()),
                    }],
                    targets: vec![RealityTarget {
                        name: "local-shell".to_string(),
                        kind: TargetKind::LocalShell,
                        profile: "default".to_string(),
                    }],
                },
                RealityProject {
                    name: "Dreamscape.os".to_string(),
                    description: "Dreamscape derivative of Reality.os".to_string(),
                    kind: ProjectKind::DreamscapeOs,
                    repos: vec![RealityRepo {
                        name: "Dreamscape.os".to_string(),
                        url: "https://github.com/Doctor0Evil/Dreamscape.os.git".to_string(),
                        branch: Some("main".to_string()),
                        path: Some("Dreamscape.os".to_string()),
                    }],
                    targets: vec![RealityTarget {
                        name: "local-shell".to_string(),
                        kind: TargetKind::LocalShell,
                        profile: "default".to_string(),
                    }],
                },
                RealityProject {
                    name: "XR-Grid-Infrastructure".to_string(),
                    description: "XR grid control-plane and governance".to_string(),
                    kind: ProjectKind::XrGridInfrastructure,
                    repos: vec![RealityRepo {
                        name: "XR-Grid-Infrastructure".to_string(),
                        url: "https://github.com/Doctor0Evil/XR-Grid-Infrastructure.git".to_string(),
                        branch: Some("main".to_string()),
                        path: Some("XR-Grid-Infrastructure".to_string()),
                    }],
                    targets: vec![RealityTarget {
                        name: "local-shell".to_string(),
                        kind: TargetKind::LocalShell,
                        profile: "default".to_string(),
                    }],
                },
            ],
        }
    }
}
