# Dependency Upgrade Design

## Goal

Upgrade TauriScapes to current stable project dependencies across the JavaScript/Vue/Vite layer and the Rust/Tauri layer, including release-path validation for Tauri updater artifacts. The upgrade should remove Tauri 2 release-candidate and git plugin dependencies where stable package and crate releases are available.

## Current Context

The project is a pnpm-managed Vue/Vite frontend with a Tauri 2 desktop backend.

- JavaScript package manager: pnpm, with `pnpm-lock.yaml`
- Frontend: Vue 3, Vue Router, Pinia, VueUse, Tailwind, DaisyUI, Vite
- Desktop backend: Tauri 2 RC-era dependencies in `src-tauri/Cargo.toml`
- Release behavior: `tauri.conf.json` enables updater artifact creation and points the updater endpoint at GitHub Releases
- Local environment note: `cargo` was not available in the current shell during discovery, so Rust/Tauri verification requires a Rust toolchain before final implementation can be fully validated

## Upgrade Strategy

Use a staged full-upgrade path. The end target is current stable versions, but the implementation should separate changes into layers so failures are attributable.

### Stage 1: Stabilize Tauri Dependencies

Move Tauri and Tauri plugins from RC or git-sourced dependencies to stable releases.

Scope:

- Update npm packages such as `@tauri-apps/api`, `@tauri-apps/cli`, and `@tauri-apps/plugin-*` to stable versions.
- Update Rust crates such as `tauri`, `tauri-build`, `tauri-plugin-*`, and related Tauri plugin dependencies to stable versions.
- Replace git dependencies for Tauri plugins with registry crates when stable registry releases exist.
- Check `tauri.conf.json` for Tauri 2 stable schema compatibility.
- Check updater configuration, updater public key usage, endpoint compatibility, and `createUpdaterArtifacts`.

Expected result:

- Tauri dependency graph no longer relies on Tauri 2 RC versions.
- Release and updater settings remain explicit and compatible with stable Tauri 2.

### Stage 2: Upgrade Application Runtime Dependencies

Upgrade runtime libraries used by the Vue application.

Scope:

- Upgrade Vue and `@vue/compiler-sfc` together.
- Upgrade Vue Router, Pinia, VueUse, date-fns, qs, lodash-es, tailwind-merge, and type packages.
- Fix API, type, or import changes discovered by type checking.
- Keep behavior focused on existing app functionality; do not introduce feature changes.

Expected result:

- Runtime dependencies are current stable versions.
- Existing app behavior remains unchanged except for required compatibility fixes.

### Stage 3: Upgrade Build, Lint, and Styling Tooling

Upgrade the toolchain and migrate config where major versions require it.

Scope:

- Upgrade Vite, `@vitejs/plugin-vue`, TypeScript, vue-tsc, ESLint, `@antfu/eslint-config`, PostCSS, Autoprefixer, Sass, Tailwind, DaisyUI, and related plugins.
- Update config files only as required by the upgraded tools.
- Preserve current app styling intent. Tailwind and DaisyUI changes should be compatibility migrations, not redesign work.

Expected result:

- Build and lint tools run on current stable versions.
- Config files reflect the upgraded toolchain requirements.

## Verification Plan

Run verification after each stage where possible and again at the end.

Required checks:

- `pnpm install`
- `pnpm lint`
- `pnpm lint:ts`
- `pnpm build`
- Rust dependency resolution and compilation checks once `cargo` is available
- Tauri build using `pnpm tauri:build` once the Rust/Tauri toolchain is available

Release-path checks:

- Confirm updater config remains valid in `tauri.conf.json`.
- Confirm Tauri build still requests updater artifacts.
- Confirm generated updater artifacts or the blocking reason if local signing/build requirements are unavailable.
- Confirm package version sourcing from `package.json` still works.

If the local machine lacks Rust/Cargo, the implementation should complete all JavaScript-side validation, document the Rust toolchain blocker clearly, and avoid claiming full Tauri build verification until the toolchain is present.

## Risks And Mitigations

- Tauri RC-to-stable migration may require config or API updates. Mitigate by handling Tauri first and verifying before unrelated frontend major upgrades.
- Vite, TypeScript, ESLint, Tailwind, and DaisyUI major upgrades may require config migrations. Mitigate by applying these after app runtime dependencies are stable.
- Tailwind/DaisyUI visual changes may alter the app unintentionally. Mitigate by limiting changes to compatibility and running a browser smoke check if the app can start.
- Missing Rust/Cargo prevents complete release verification. Mitigate by treating toolchain availability as a required environment prerequisite and reporting exactly which checks could not run.

## Out Of Scope

- Redesigning UI or changing product behavior.
- Changing release hosting or updater endpoint semantics.
- Adding new features during the dependency upgrade.
- Reworking unrelated architecture beyond compatibility fixes required by upgraded dependencies.

## Approval Criteria

The upgrade is complete when:

- Dependencies in `package.json` and `src-tauri/Cargo.toml` are updated to current stable versions or a documented compatibility-pinned stable version.
- Lockfiles are refreshed.
- Frontend lint, type checking, and build pass.
- Tauri/Rust checks pass, or a missing local toolchain is documented as the only blocker.
- Updater artifact configuration is inspected and either verified through `tauri build` or blocked by a specific local environment prerequisite.
