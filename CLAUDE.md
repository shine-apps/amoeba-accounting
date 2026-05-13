# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

阿米巴单位时间核算表 (Amoeba Unit Time Accounting Table) — a Tauri 2.x desktop app that digitizes Kyocera's Amoeba Management accounting system. Single-user, local SQLite storage.

## Build/Run Commands

```bash
# Install dependencies
pnpm install

# Run in development mode (Vite HMR + Tauri)
pnpm tauri dev

# Frontend only (browser, no Tauri backend — limited functionality)
pnpm dev

# Type-check + build frontend
pnpm build

# Run Rust tests (from src-tauri/)
cd src-tauri && cargo test

# Run a single Rust test
cd src-tauri && cargo test calculator_tests::test_standard_calculation

# Build for release
pnpm tauri build
```

## Architecture

**Desktop shell**: Tauri 2.x with Rust backend + WebView frontend. The Rust backend owns the database, calculation engine, validator, and Excel exporter. The frontend communicates with Rust exclusively through Tauri IPC `invoke()` calls defined in `src-tauri/src/commands/`.

**Frontend**: Vue 3 Composition API + TypeScript + Pinia + Element Plus + ECharts (via vue-echarts). The `@` alias resolves to `src/`.

**Data flow**: User input → Pinia store action → `useTauri` composable → `invoke()` → Tauri command → Rust service layer → Repository → SQLite. Results flow back the same chain.

### Key architectural decisions

- **Duplicated calculation logic**: The 9-formula calculation engine exists in both Rust (`src-tauri/src/services/calculator.rs`) and TypeScript (`src/composables/useAccounting.ts`). The Rust version is authoritative for persistence; the TypeScript version enables instant frontend preview without an IPC round-trip. Both must produce identical results for the same inputs.

- **Database access pattern**: The Rust backend uses a `Mutex<Connection>` managed as Tauri state, not a connection pool. All commands lock the mutex, do their work, and release. SQLite is opened in WAL mode with foreign keys enabled.

- **Save = upsert**: The `save_record` command handles both create and update. A `record_id` parameter `> 0` triggers update; otherwise it creates. On update, expense details are deleted and re-inserted in batch.

### Directory structure

| Path | Purpose |
|------|---------|
| `src/types/` | TypeScript type definitions (mirror Rust models) |
| `src/stores/` | Pinia stores: `amoeba` (Amoeba CRUD), `record` (accounting records), `app` (UI state) |
| `src/composables/` | `useTauri` (IPC bridge), `useAccounting` (frontend calculation) |
| `src/views/` | Route-level pages: Dashboard, DataEntry, ReportView, TrendAnalysis, AmoebaManager, ExportPage |
| `src/components/` | Shared components: AppLayout (sidebar + router-view), AccountingTable, ExpenseEditor, LaborTimeEditor, ResultPreview |
| `src/router/index.ts` | Vue Router: `/`, `/amoeba`, `/entry`, `/entry/:id`, `/report`, `/trend`, `/export` |
| `src/utils/` | `constants.ts` (expense categories, amoeba types, period types), `format.ts` (money/percent formatting) |
| `src-tauri/src/models/` | Rust data models: `Amoeba`, `AccountingRecord`, `ExpenseDetail`, `LaborTime`, `AccountingResult` |
| `src-tauri/src/repository/` | Database CRUD: `db.rs` (schema/migrations), `amoeba_repo`, `record_repo`, `expense_repo`, `labor_repo` |
| `src-tauri/src/services/` | Business logic: `calculator` (9 core formulas), `validator` (input validation), `aggregator` (multi-record rollup) |
| `src-tauri/src/commands/` | Tauri IPC commands exposed to frontend |
| `src-tauri/src/export/` | Excel export via rust_xlsxwriter: 3 sheets (核算表, 费用明细, 趋势分析) |
| `src-tauri/tests/api_tests.rs` | Integration tests: calculator, validator, repository CRUD, aggregator |

### Database schema (4 tables)

- `amoeba` — organizational units (production/marketing/R&D/management types)
- `accounting_record` — main entries with pre-computed result fields (total_sales, unit_value, etc.)
- `expense_detail` — line items per record, FK cascades on delete
- `labor_time` — one-to-one with accounting_record, FK cascades on delete

### Tauri IPC commands (all in `src-tauri/src/commands/`)

`list_amoebas`, `create_amoeba`, `update_amoeba`, `delete_amoeba`, `list_records`, `get_record`, `save_record`, `delete_record`, `export_excel`

### CI

GitHub Actions workflow in `.github/workflows/release.yml` triggers on version tags (`v*`) and builds for macOS (Intel + Apple Silicon), Linux, and Windows via `tauri-action@v0`.
