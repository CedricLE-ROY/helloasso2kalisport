# ROBOTS.md

## Project Overview

This Rust monorepo provides an internal web application for a karate club to bridge HelloAsso (online registration/payment platform) and Kalisport (members management software with CSV import, no API). The goal is to automate the retrieval and mapping of new HelloAsso registrations, allow manual selection, and export a ready-to-import CSV for Kalisport.

## Architecture

- **Workspace root (`Cargo.toml`):** Declares a Rust workspace with three main crates.
- **backend/**: Rust API server (Axum, async), serves a REST API, handles HelloAsso integration, mapping, export, and persistence (SQLite).
- **frontend/**: Yew SPA (WebAssembly), provides UI for login, selection, and export, communicates with the backend via HTTP (using types shared with backend).
- **shared/**: Rust library crate with all types/models used by both backend and frontend (`Adherent`, registration models, CSV mapping, etc.).

## Key Conventions

- **All business logic and data models are defined in `shared/`** for maximum type safety and to avoid duplication.
- **Backend endpoints** are always versioned and located under `/api/`.
- **Frontend communicates via REST using `gloo-net` and deserializes responses with `serde`.**
- **CSV export logic**: Mapping and formatting is strictly handled on the backend; the frontend only triggers and downloads exports.

## File Structure (simplified)

├── backend/ # Axum API server (main.rs, app.rs, api/, services/, db/, models/, auth/)
├── frontend/ # Yew SPA (main.rs, app.rs, routes/, components/, services/, static/)
├── shared/ # Shared types (lib.rs, models.rs)
├── Cargo.toml # Workspace definition
└── README.md


## What should a code assistant do?

- When adding or editing features, **always update shared types in `shared/` if the structure of data exchanged between backend and frontend changes**.
- **Never duplicate models between backend and frontend** – always use the shared crate for all user, registration, and export types.
- **When creating new REST endpoints, document them with OpenAPI comments if possible.**
- **CSV field mapping and any data transformation should be explicit and testable.**
- **Security**: Prefer JWT (or cookies) for authentication. Any sensitive keys (HelloAsso API, DB) must be kept in environment variables.

## Special Business Rules

- Each HelloAsso registration must be mapped to the Kalisport format before export.
- Exported lines must be marked as "already exported" (in the DB), and the UI must allow filtering/selecting only new (unexported) entries.
- The mapping logic must be robust to HelloAsso field changes.
- The app is multi-user and should eventually support simple user management (admin/secrétariat roles).

## Coding & Quality

- Rust 2024 edition, always use the latest stable versions of crates unless otherwise required.
- Clippy and cargo check must always pass.
- All code must be documented and modular. Use Rust idioms everywhere (Result, Option, error handling, async, etc).
- Yew code should use function components and props, never duplicate models locally.

## You are NOT allowed to

- Add business logic in the frontend (keep it in backend or shared).
- Commit hardcoded API credentials or sensitive information.
- Create endpoints or features not covered by the actual workflow of the club.

## What to do when unsure?

- Default to sharing types and logic in `shared/`.
- Leave detailed comments/questions in code for review by humans.
- When in doubt about CSV mapping or business logic, request human clarification before proceeding.

