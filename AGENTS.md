# ROBOTS.md

## Project Overview

This Rust monorepo provides an internal web application for a karate club to bridge HelloAsso (online registration/payment platform) and Kalisport (members management software with CSV import, no API). The goal is to automate the retrieval and mapping of new HelloAsso registrations, allow manual selection, and export a ready-to-import CSV for Kalisport.

We are currently in **active development**. The codebase is evolving, and **architectural changes are not only permitted but encouraged** when they lead to **better modularity**, **performance**, or **security**.

---

## Architecture

- **Workspace root (`Cargo.toml`)**: Declares a Rust workspace with three main crates.
- **backend/**: Rust API server (Axum, async), serves a REST API, handles HelloAsso integration, mapping, export, and persistence (SQLite).
- **frontend/**: Yew SPA (WebAssembly), provides UI for login, selection, and export, communicates with the backend via HTTP (using types shared with backend).
- **shared/**: Rust library crate with all types/models used by both backend and frontend (`Adherent`, registration models, CSV mapping, etc.).

> 🛠️ **You are free to restructure the project**: you may rename files, move modules across crates, create sub-crates or folders, or reorganize responsibilities between layers if that improves the architecture.

---

## Key Conventions

- All business logic and data models are defined in `shared/` for maximum type safety and to avoid duplication.
- Backend endpoints are always versioned and located under `/api/`.
- Frontend communicates via REST using `gloo-net` and deserializes responses with `serde`.
- CSV export logic: Mapping and formatting is strictly handled on the backend; the frontend only triggers and downloads exports.

> ✨ **Architecture and naming are not frozen** — improvements are welcome, as long as consistency is maintained across crates.

---

## File Structure (simplified – subject to change)

├── backend/ # Axum API server (main.rs, app.rs, api/, services/, db/, models/, auth/)  
├── frontend/ # Yew SPA (main.rs, app.rs, routes/, components/, services/, static/)  
├── shared/ # Shared types (lib.rs, models.rs)  
├── Cargo.toml # Workspace definition  
└── README.md  

---

## What should a code assistant do?

- You **can refactor or rename modules, folders, or files** if it improves maintainability, modularity, or clarity.
- When adding or editing features, **always update shared types in `shared/`** if data structures change.
- Never duplicate models between backend and frontend – use the shared crate.
- New REST endpoints should be documented with OpenAPI comments if possible.
- CSV field mapping and any data transformation must be explicit and testable.
- Use JWT (or secure cookies) for authentication. API keys or sensitive data must go through env variables.

---

## Special Business Rules

- Each HelloAsso registration must be mapped to the Kalisport format before export.
- Exported lines must be marked as "already exported" in DB.
- The frontend must allow selecting/filtering only new (unexported) entries.
- Mapping logic must be robust to HelloAsso field changes.
- The app is multi-user and should support basic roles (admin/secrétariat).

---

## Coding & Quality Standards

- Rust 2024 edition; use the latest stable crate versions unless pinned for compatibility.
- Clippy and `cargo check` must pass at all times.
- All code must be modular, well-documented, and idiomatic.
- Yew code must use function components with props; no local type duplication.
- When doing refactors: prefer readability and separation of concerns.

---

## You are allowed to:

- 🔧 **Restructure or rename parts of the codebase**
- 🔧 **Split or merge crates if that improves architecture**
- ✅ Propose a new `ARCHITECTURE.md` if changes are significant

## You are NOT allowed to:

- Add business logic in the frontend (unless UI-specific)
- Commit hardcoded API credentials or secrets
- Add features unrelated to the app's current scope

---

## When in doubt:

- Default to placing shared logic in `shared/`.
- Leave inline TODOs or comments to request review or feedback.
- If a decision impacts the business flow (e.g., CSV structure), ask for human clarification.
