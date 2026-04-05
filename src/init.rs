//! Bundled default runbooks for `.agent/runbooks/`.

use std::fs;
use std::path::Path;

const DEFAULT_PRECOMMIT: &str = include_str!("../runbooks/precommit.md");
const DEFAULT_PRERELEASE: &str = include_str!("../runbooks/prerelease.md");

/// Bundled runbook definition.
pub struct BundledRunbook {
    pub name: &'static str,
    pub content: &'static str,
}

/// All bundled default runbooks.
pub const BUNDLED: &[BundledRunbook] = &[
    BundledRunbook {
        name: "precommit.md",
        content: DEFAULT_PRECOMMIT,
    },
    BundledRunbook {
        name: "prerelease.md",
        content: DEFAULT_PRERELEASE,
    },
];

/// Initialize `.agent/runbooks/` with bundled defaults.
///
/// Only writes files that don't already exist (never overwrites user customizations).
/// Returns the number of files written.
pub fn init_runbooks(root: &Path) -> std::io::Result<usize> {
    let dir = root.join(".agent/runbooks");
    fs::create_dir_all(&dir)?;

    let mut written = 0;
    for runbook in BUNDLED {
        let path = dir.join(runbook.name);
        if !path.exists() {
            fs::write(&path, runbook.content)?;
            written += 1;
        }
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn init_runbooks_creates_defaults() {
        let tmp = TempDir::new().unwrap();
        let written = init_runbooks(tmp.path()).unwrap();
        assert_eq!(written, 2);
        assert!(tmp.path().join(".agent/runbooks/precommit.md").exists());
        assert!(tmp.path().join(".agent/runbooks/prerelease.md").exists());
    }

    #[test]
    fn init_runbooks_does_not_overwrite() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join(".agent/runbooks");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("precommit.md"), "custom content").unwrap();

        let written = init_runbooks(tmp.path()).unwrap();
        assert_eq!(written, 1); // only prerelease.md

        let content = fs::read_to_string(dir.join("precommit.md")).unwrap();
        assert_eq!(content, "custom content");
    }

    #[test]
    fn bundled_content_is_non_empty() {
        for runbook in BUNDLED {
            assert!(!runbook.content.is_empty(), "{} is empty", runbook.name);
        }
    }
}
