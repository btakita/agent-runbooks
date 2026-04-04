use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "agent-runbooks", about = "Manage agent skill runbooks")]
struct Cli {
    /// Override search path
    #[arg(long, global = true)]
    dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate SKILL.md runbook references
    Audit {
        /// Path to the skill directory
        skill_dir: PathBuf,
    },
    /// List runbooks with metadata
    List {
        /// Path to the skill directory
        skill_dir: PathBuf,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Check a runbook file for format issues
    Validate {
        /// Path to the runbook file
        runbook_file: PathBuf,
    },
    /// Copy a template runbook into a skill directory
    Install {
        /// Path to the skill directory
        skill_dir: PathBuf,
        /// Template file to install from
        #[arg(long)]
        from: PathBuf,
        /// Append a reference line to SKILL.md's ## Runbooks section
        #[arg(long)]
        add_ref: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Audit { skill_dir } => cmd_audit(&resolve_dir(&cli.dir, &skill_dir)),
        Command::List { skill_dir, json } => cmd_list(&resolve_dir(&cli.dir, &skill_dir), json),
        Command::Validate { runbook_file } => cmd_validate(&runbook_file),
        Command::Install {
            skill_dir,
            from,
            add_ref,
        } => cmd_install(&resolve_dir(&cli.dir, &skill_dir), &from, add_ref),
    }
}

fn resolve_dir(global_dir: &Option<PathBuf>, skill_dir: &Path) -> PathBuf {
    match global_dir {
        Some(d) => d.join(skill_dir),
        None => skill_dir.to_path_buf(),
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Extract runbook references from SKILL.md content.
/// Looks for lines in the `## Runbooks` section containing `runbooks/*.md`.
fn parse_skill_refs(content: &str) -> Vec<(String, String)> {
    let mut in_runbooks_section = false;
    let mut refs = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            in_runbooks_section = trimmed == "## Runbooks";
            continue;
        }
        if !in_runbooks_section {
            continue;
        }
        if let Some(filename) = extract_runbook_filename(trimmed) {
            let trigger = extract_trigger(trimmed);
            refs.push((trigger, filename));
        }
    }
    refs
}

/// Extract the runbook filename from a line, e.g. "runbooks/foo.md" -> "foo.md"
fn extract_runbook_filename(line: &str) -> Option<String> {
    let idx = line.find("runbooks/")?;
    let rest = &line[idx + "runbooks/".len()..];
    let end = rest
        .find(|c: char| c == ')' || c == ']' || c == ' ' || c == '`' || c == '"')
        .unwrap_or(rest.len());
    let name = &rest[..end];
    if name.ends_with(".md") && !name.is_empty() {
        Some(name.to_string())
    } else {
        None
    }
}

/// Extract a trigger phrase from a runbook reference line.
fn extract_trigger(line: &str) -> String {
    // Pattern: `trigger` — ...
    if let Some(start) = line.find('`') {
        if let Some(end) = line[start + 1..].find('`') {
            return line[start + 1..start + 1 + end].to_string();
        }
    }
    // Pattern: - text — ... or - text -- ...
    let stripped = line.trim_start_matches("- ").trim_start_matches("* ");
    if let Some(idx) = stripped.find(" — ") {
        return stripped[..idx].trim().to_string();
    }
    if let Some(idx) = stripped.find(" -- ") {
        return stripped[..idx].trim().to_string();
    }
    "(none)".to_string()
}

/// List runbook .md files in a directory.
fn list_runbook_files(runbooks_dir: &Path) -> Result<BTreeSet<String>> {
    let mut files = BTreeSet::new();
    if !runbooks_dir.is_dir() {
        return Ok(files);
    }
    for entry in fs::read_dir(runbooks_dir).context("reading runbooks directory")? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") {
            files.insert(name);
        }
    }
    Ok(files)
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

fn cmd_audit(skill_dir: &Path) -> Result<()> {
    let skill_md = skill_dir.join("SKILL.md");
    let content = fs::read_to_string(&skill_md)
        .with_context(|| format!("reading {}", skill_md.display()))?;
    let refs = parse_skill_refs(&content);
    let runbooks_dir = skill_dir.join("runbooks");
    let on_disk = list_runbook_files(&runbooks_dir)?;

    let referenced: BTreeSet<String> = refs.iter().map(|(_, f)| f.clone()).collect();
    let mut has_issues = false;

    println!("{:<10} {:<40} {}", "STATUS", "PATH", "TRIGGER");
    println!("{}", "-".repeat(70));

    for (trigger, filename) in &refs {
        let path = runbooks_dir.join(filename);
        let status = if path.exists() { "ok" } else { "missing" };
        if status == "missing" {
            has_issues = true;
        }
        println!(
            "{:<10} {:<40} {}",
            status,
            format!("runbooks/{}", filename),
            trigger
        );
    }

    for file in &on_disk {
        if !referenced.contains(file) {
            has_issues = true;
            println!(
                "{:<10} {:<40} {}",
                "orphan",
                format!("runbooks/{}", file),
                ""
            );
        }
    }

    if has_issues {
        std::process::exit(1);
    }
    Ok(())
}

#[derive(Serialize)]
struct RunbookEntry {
    filename: String,
    title: String,
    line_count: usize,
    referenced: bool,
}

fn cmd_list(skill_dir: &Path, json: bool) -> Result<()> {
    let runbooks_dir = skill_dir.join("runbooks");
    let files = list_runbook_files(&runbooks_dir)?;

    let skill_md = skill_dir.join("SKILL.md");
    let referenced: BTreeSet<String> = if skill_md.exists() {
        let content = fs::read_to_string(&skill_md)?;
        parse_skill_refs(&content)
            .into_iter()
            .map(|(_, f)| f)
            .collect()
    } else {
        BTreeSet::new()
    };

    let mut entries = Vec::new();
    for filename in &files {
        let path = runbooks_dir.join(filename);
        let content = fs::read_to_string(&path)?;
        let title = content
            .lines()
            .find(|l| l.starts_with("# "))
            .map(|l| l[2..].trim().to_string())
            .unwrap_or_default();
        let line_count = content.lines().count();
        entries.push(RunbookEntry {
            filename: filename.clone(),
            title,
            line_count,
            referenced: referenced.contains(filename),
        });
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else {
        println!(
            "{:<30} {:<40} {:<6} {}",
            "FILENAME", "TITLE", "LINES", "REF"
        );
        println!("{}", "-".repeat(85));
        for e in &entries {
            println!(
                "{:<30} {:<40} {:<6} {}",
                e.filename,
                e.title,
                e.line_count,
                if e.referenced { "yes" } else { "no" }
            );
        }
    }

    Ok(())
}

fn cmd_validate(runbook_file: &Path) -> Result<()> {
    let content = fs::read_to_string(runbook_file)
        .with_context(|| format!("reading {}", runbook_file.display()))?;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Must have a title
    let has_title = content.lines().any(|l| l.starts_with("# "));
    if !has_title {
        errors.push("Missing title (no `# ...` heading found)".to_string());
    }

    // Should have ## Steps
    let has_steps_section = content.lines().any(|l| l.trim() == "## Steps");
    if !has_steps_section {
        warnings.push("No `## Steps` section found".to_string());
    }

    // Should have numbered steps
    let has_numbered = content.lines().any(|l| {
        let t = l.trim();
        t.len() > 2 && t.as_bytes()[0].is_ascii_digit() && t.contains(". ")
    });
    if !has_numbered {
        warnings.push("No numbered steps found".to_string());
    }

    // Check for broken internal references
    let parent = runbook_file.parent().unwrap_or(Path::new("."));
    for line in content.lines() {
        let mut search = line;
        while let Some(idx) = search.find("](") {
            let rest = &search[idx + 2..];
            if let Some(end) = rest.find(')') {
                let link = &rest[..end];
                if !link.starts_with("http") && !link.starts_with('#') && !link.is_empty() {
                    let file_path = link.split('#').next().unwrap_or(link);
                    if !file_path.is_empty() && !parent.join(file_path).exists() {
                        errors.push(format!("Broken link: {}", link));
                    }
                }
                search = &rest[end..];
            } else {
                break;
            }
        }
    }

    for e in &errors {
        println!("ERROR: {}", e);
    }
    for w in &warnings {
        println!("WARN:  {}", w);
    }

    if errors.is_empty() && warnings.is_empty() {
        println!("OK: no issues found");
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

fn cmd_install(skill_dir: &Path, template: &Path, add_ref: bool) -> Result<()> {
    if !template.exists() {
        bail!("Template file not found: {}", template.display());
    }

    let runbooks_dir = skill_dir.join("runbooks");
    fs::create_dir_all(&runbooks_dir).context("creating runbooks directory")?;

    let basename = template
        .file_name()
        .context("template has no filename")?
        .to_string_lossy()
        .to_string();
    let dest = runbooks_dir.join(&basename);
    fs::copy(template, &dest)
        .with_context(|| format!("copying {} to {}", template.display(), dest.display()))?;
    println!("Installed: {}", dest.display());

    if add_ref {
        let skill_md = skill_dir.join("SKILL.md");
        let mut content = if skill_md.exists() {
            fs::read_to_string(&skill_md)?
        } else {
            String::new()
        };

        let trigger = basename.trim_end_matches(".md").replace('-', " ");
        let ref_line = format!(
            "- `{}` — [runbooks/{}](runbooks/{})\n",
            trigger, basename, basename
        );

        if content.contains("## Runbooks") {
            let section_start = content.find("## Runbooks").unwrap();
            let after_heading = section_start + "## Runbooks".len();
            let insert_pos = content[after_heading..]
                .find("\n## ")
                .map(|i| after_heading + i)
                .unwrap_or(content.len());
            if !content[..insert_pos].ends_with('\n') {
                content.insert(insert_pos, '\n');
                content.insert_str(insert_pos + 1, &ref_line);
            } else {
                content.insert_str(insert_pos, &ref_line);
            }
        } else {
            content.push_str("\n## Runbooks\n\n");
            content.push_str(&ref_line);
        }

        fs::write(&skill_md, content)?;
        println!("Added reference to SKILL.md");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_skill_dir(tmp: &TempDir) -> PathBuf {
        let skill = tmp.path().join("skill");
        fs::create_dir_all(skill.join("runbooks")).unwrap();
        skill
    }

    fn write_skill_md(skill_dir: &Path, content: &str) {
        fs::write(skill_dir.join("SKILL.md"), content).unwrap();
    }

    fn write_runbook(skill_dir: &Path, name: &str, content: &str) {
        fs::write(skill_dir.join("runbooks").join(name), content).unwrap();
    }

    // -- audit --

    #[test]
    fn audit_all_ok() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(
            &skill,
            "# Test\n\n## Runbooks\n\n- `deploy` — [runbooks/deploy.md](runbooks/deploy.md)\n",
        );
        write_runbook(&skill, "deploy.md", "# Deploy\n\n## Steps\n\n1. Do it\n");

        let refs = parse_skill_refs(&fs::read_to_string(skill.join("SKILL.md")).unwrap());
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].0, "deploy");
        assert_eq!(refs[0].1, "deploy.md");

        let on_disk = list_runbook_files(&skill.join("runbooks")).unwrap();
        let referenced: BTreeSet<String> = refs.iter().map(|(_, f)| f.clone()).collect();
        assert_eq!(referenced, on_disk);
    }

    #[test]
    fn audit_detects_missing() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(
            &skill,
            "# Test\n\n## Runbooks\n\n- `deploy` — [runbooks/deploy.md](runbooks/deploy.md)\n",
        );

        let refs = parse_skill_refs(&fs::read_to_string(skill.join("SKILL.md")).unwrap());
        assert_eq!(refs.len(), 1);
        assert!(!skill.join("runbooks/deploy.md").exists());
    }

    #[test]
    fn audit_detects_orphan() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(&skill, "# Test\n\n## Runbooks\n\n");
        write_runbook(&skill, "orphan.md", "# Orphan\n");

        let refs = parse_skill_refs(&fs::read_to_string(skill.join("SKILL.md")).unwrap());
        assert!(refs.is_empty());
        let on_disk = list_runbook_files(&skill.join("runbooks")).unwrap();
        assert!(on_disk.contains("orphan.md"));
    }

    // -- list --

    #[test]
    fn list_extracts_metadata() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(
            &skill,
            "# Skill\n\n## Runbooks\n\n- `setup` — [runbooks/setup.md](runbooks/setup.md)\n",
        );
        write_runbook(
            &skill,
            "setup.md",
            "# Setup Guide\n\nSome content\n\n## Steps\n\n1. First\n2. Second\n",
        );

        let files = list_runbook_files(&skill.join("runbooks")).unwrap();
        assert_eq!(files.len(), 1);

        let content = fs::read_to_string(skill.join("runbooks/setup.md")).unwrap();
        let title = content
            .lines()
            .find(|l| l.starts_with("# "))
            .map(|l| l[2..].trim().to_string())
            .unwrap();
        assert_eq!(title, "Setup Guide");
        assert_eq!(content.lines().count(), 8);
    }

    #[test]
    fn list_json_serialization() {
        let entry = RunbookEntry {
            filename: "test.md".to_string(),
            title: "Test Runbook".to_string(),
            line_count: 10,
            referenced: false,
        };
        let json = serde_json::to_string_pretty(&[entry]).unwrap();
        assert!(json.contains("Test Runbook"));
        assert!(json.contains("\"referenced\": false"));
    }

    #[test]
    fn list_referenced_vs_unreferenced() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(
            &skill,
            "# Skill\n\n## Runbooks\n\n- `a` — [runbooks/a.md](runbooks/a.md)\n",
        );
        write_runbook(&skill, "a.md", "# A\n");
        write_runbook(&skill, "b.md", "# B\n");

        let referenced: BTreeSet<String> =
            parse_skill_refs(&fs::read_to_string(skill.join("SKILL.md")).unwrap())
                .into_iter()
                .map(|(_, f)| f)
                .collect();
        assert!(referenced.contains("a.md"));
        assert!(!referenced.contains("b.md"));
    }

    // -- validate --

    #[test]
    fn validate_good_runbook() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("good.md");
        fs::write(
            &path,
            "# Good Runbook\n\nIntro.\n\n## Steps\n\n1. First step\n2. Second step\n",
        )
        .unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.lines().any(|l| l.starts_with("# ")));
        assert!(content.lines().any(|l| l.trim() == "## Steps"));
        assert!(content.lines().any(|l| {
            let t = l.trim();
            t.len() > 2 && t.as_bytes()[0].is_ascii_digit() && t.contains(". ")
        }));
    }

    #[test]
    fn validate_missing_title_detected() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("notitle.md");
        fs::write(&path, "No title here\n\n## Steps\n\n1. Do stuff\n").unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(!content.lines().any(|l| l.starts_with("# ")));
    }

    #[test]
    fn validate_broken_link_detected() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("broken.md");
        fs::write(
            &path,
            "# Broken\n\nSee [other](nonexistent.md) for details.\n",
        )
        .unwrap();

        let parent = path.parent().unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let mut broken = Vec::new();
        for line in content.lines() {
            let mut search = line;
            while let Some(idx) = search.find("](") {
                let rest = &search[idx + 2..];
                if let Some(end) = rest.find(')') {
                    let link = &rest[..end];
                    if !link.starts_with("http")
                        && !link.starts_with('#')
                        && !link.is_empty()
                    {
                        let file_path = link.split('#').next().unwrap_or(link);
                        if !file_path.is_empty() && !parent.join(file_path).exists() {
                            broken.push(link.to_string());
                        }
                    }
                    search = &rest[end..];
                } else {
                    break;
                }
            }
        }
        assert_eq!(broken, vec!["nonexistent.md"]);
    }

    // -- install --

    #[test]
    fn install_copies_template() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        let template = tmp.path().join("template.md");
        fs::write(&template, "# Template\n\n## Steps\n\n1. Do it\n").unwrap();

        cmd_install(&skill, &template, false).unwrap();
        assert!(skill.join("runbooks/template.md").exists());
        let content = fs::read_to_string(skill.join("runbooks/template.md")).unwrap();
        assert!(content.contains("# Template"));
    }

    #[test]
    fn install_with_add_ref() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(&skill, "# Skill\n\n## Runbooks\n\n");
        let template = tmp.path().join("my-runbook.md");
        fs::write(&template, "# My Runbook\n").unwrap();

        cmd_install(&skill, &template, true).unwrap();
        let skill_content = fs::read_to_string(skill.join("SKILL.md")).unwrap();
        assert!(skill_content.contains("runbooks/my-runbook.md"));
        assert!(skill_content.contains("`my runbook`"));
    }

    #[test]
    fn install_creates_runbooks_section_when_missing() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        write_skill_md(&skill, "# Skill\n\nSome content.\n");
        let template = tmp.path().join("foo.md");
        fs::write(&template, "# Foo\n").unwrap();

        cmd_install(&skill, &template, true).unwrap();
        let content = fs::read_to_string(skill.join("SKILL.md")).unwrap();
        assert!(content.contains("## Runbooks"));
        assert!(content.contains("runbooks/foo.md"));
    }

    #[test]
    fn install_missing_template_fails() {
        let tmp = TempDir::new().unwrap();
        let skill = create_skill_dir(&tmp);
        let result = cmd_install(&skill, Path::new("/nonexistent/template.md"), false);
        assert!(result.is_err());
    }

    // -- parsing --

    #[test]
    fn parse_refs_from_real_format() {
        let content = "\
# Skill

## Runbooks

- `compact exchange` — [runbooks/compact-exchange.md](runbooks/compact-exchange.md)
- `cleanup` — [runbooks/cleanup.md](runbooks/cleanup.md)

## Other Section
";
        let refs = parse_skill_refs(content);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].0, "compact exchange");
        assert_eq!(refs[0].1, "compact-exchange.md");
        assert_eq!(refs[1].0, "cleanup");
        assert_eq!(refs[1].1, "cleanup.md");
    }

    #[test]
    fn parse_refs_ignores_other_sections() {
        let content = "\
# Skill

## Overview

See runbooks/not-a-ref.md for info.

## Runbooks

- `real` — [runbooks/real.md](runbooks/real.md)
";
        let refs = parse_skill_refs(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].1, "real.md");
    }
}
