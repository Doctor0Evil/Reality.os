use crate::ClonePipeline;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reality_core::{GitBackend, GitRepo, RealityManifest};
use std::path::Path;

pub struct ClonePipeline;

impl ClonePipeline {
    pub fn run(manifest: &RealityManifest, dest_root: &Path) -> anyhow::Result<()> {
        let mp = MultiProgress::new();
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {msg} ({pos}/{len})",
        )?;

        let mut repos = Vec::new();
        for project in &manifest.projects {
            for repo in &project.repos {
                repos.push((project.name.clone(), repo.url.clone(), repo.branch.clone(), repo.path.clone()));
            }
        }

        let len = repos.len() as u64;
        let pb = mp.add(ProgressBar::new(len));
        pb.set_style(style);
        pb.set_message("cloning CyberOrganic stack");

        for (idx, (project_name, url, branch, path_suffix)) in repos.into_iter().enumerate() {
            let sub = pb.clone();
            let msg = format!("{} :: {}", idx + 1, project_name);
            sub.set_message(msg);

            let mut dest = dest_root.to_path_buf();
            if let Some(suffix) = path_suffix {
                dest.push(suffix);
            }

            let repo = GitRepo { url, branch };
            GitBackend::clone_or_update(&repo, &dest)?;
            sub.inc(1);
        }

        pb.finish_with_message("clone complete");
        Ok(())
    }
}
