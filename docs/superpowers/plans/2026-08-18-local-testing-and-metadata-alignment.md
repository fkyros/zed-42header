# Local Testing, Upstream Attribution & Language Registration Alignment Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align project metadata and licensing with upstream Public Domain status, clarify community port branding, optimize `extension.toml` language configuration, and document local Zed testing procedures.

**Architecture:** Update project metadata across manifests (`extension.toml`, `Cargo.toml`, `README.md`, `LICENSE`), verify language registration mechanisms for Zed, and create a self-contained local testing guide.

**Tech Stack:** Zed Extension API, TOML, Rust, Markdown.

**Spec:** [`docs/superpowers/specs/2026-08-18-zed-42header-design.md`](file:///Users/gabri/mV/proyectos/fkyros/zed-42header/docs/superpowers/specs/2026-08-18-zed-42header-design.md)

## Global Constraints
- Upstream attribution must cite `42Paris/42header`'s Public Domain Dedication.
- Community student port nature must be explicit (avoiding false claims of official school endorsement).
- Language list in `extension.toml` must match Zed's recognized language IDs.
- Workspace test suite must remain green across all crates.

---

### Task 1: Upstream Licensing & Community Port Attribution

**Files:**
- Modify: `LICENSE`
- Modify: `README.md`
- Modify: `extension.toml`
- Modify: `Cargo.toml`
- Modify: `src/Cargo.toml`
- Modify: `server/Cargo.toml`

**Interfaces:**
- Project metadata descriptions updated to "Community-driven port of the 42 School file header for Zed Editor".
- `LICENSE` updated with dual attribution (MIT License for this codebase with notice of upstream Public Domain dedication by 42 Paris).

- [ ] **Step 1: Update `LICENSE` with upstream Public Domain notice**
- [ ] **Step 2: Update `extension.toml` description and author info**
- [ ] **Step 3: Update `Cargo.toml` and `README.md` descriptions**
- [ ] **Step 4: Commit changes**

```bash
git add LICENSE README.md extension.toml Cargo.toml src/Cargo.toml server/Cargo.toml
git commit -m "docs: clarify community port status and upstream public domain attribution"
```

---

### Task 2: Language Registration Review & Documentation

**Files:**
- Modify: `extension.toml`
- Modify: `README.md`

**Interfaces:**
- Grouped, documented list of language IDs in `extension.toml`.
- Documentation in `README.md` explaining why Zed requires explicit language enumeration for LSP activation.

- [ ] **Step 1: Clean and verify language entries in `extension.toml` (keeping in mind all 42 projects and their possible languages)**
- [ ] **Step 2: Add explanation in `README.md` regarding Zed's LSP activation model**
- [ ] **Step 3: Run `cargo test --workspace` to ensure all tests pass**
- [ ] **Step 4: Commit changes**

```bash
git add extension.toml README.md
git commit -m "chore(extension): organize language list and document Zed LSP activation"
```

---

### Task 3: Local Dev Verification & Interactive Testing Checklist

**Files:**
- Create: `docs/LOCAL_TESTING.md`

- [ ] **Step 1: Create `docs/LOCAL_TESTING.md` with step-by-step local testing workflow in Zed**
- [ ] **Step 2: Verify local binary installation in `~/.cargo/bin/header42-lsp`**
- [ ] **Step 3: Commit**

```bash
git add docs/LOCAL_TESTING.md
git commit -m "docs: add local Zed extension testing guide"
```
