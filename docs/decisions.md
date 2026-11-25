# Paket — Product Decisions

Decisions required by S002. This record fixes naming, distribution and license policy before
any UI or packaging work starts. It is a decision record, not a progress log.

## Naming

- Project name: **Paket** (plan D1).
- Binaries: `paket` (GUI) and `kangpaket` (typed privileged binary).
- The Cargo package is `paket`; libraries, desktop entries, documentation, logs and package
  names use the same name. No second product name is introduced.
- D-Bus, desktop and reverse-DNS application IDs are not defined yet. They must be chosen
  when desktop integration starts (Stage M) and must use the `Paket` name.

## Expected distribution model

- Primary distribution is native OS packages per qualified target (RPM, DEB, Arch), built
  in S166. `cargo run` is not a supported production delivery path.
- `kangpaket` ships inside the same native package, installed at a fixed absolute path,
  root-owned and not writable by regular users.
- Flatpak packaging of Paket itself is out of scope until a host-integration design exists,
  because the sandbox must reach host package tools and system services.
- Source builds stay reproducible from `Cargo.toml` + `rust-toolchain.toml`; no vendored
  binaries are shipped.

## Intended project license

**Undecided.** The repository has no `LICENSE` file and the project owner has not selected a
license. Choosing one is an owner decision, not an implementation detail; this task must not
assume it.

Consequences while undecided:

- No new dependency may force a license choice silently.
- A copyleft dependency cannot be adopted until the project license is compatible.
- Slint Kit adoption is blocked (see below).

## Third-party license policy

- Every new dependency is checked for its declared license before it is added.
- Allowed without further review: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC,
  MPL-2.0, and `MIT OR Apache-2.0` dual licensing.
- Copyleft (GPL, LGPL, AGPL) or source-available/commercial terms require an explicit
  decision in this file before the dependency is added.
- No dependency may disable signature verification, add telemetry, or ship prebuilt
  binaries without a recorded license.
- License notices and required attribution ship with release artifacts; S167 publishes an
  SBOM and dependency audit.
- Current dependencies (verified in `Cargo.lock`): `serde` (MIT OR Apache-2.0),
  `serde_json` (MIT OR Apache-2.0), `thiserror` (MIT OR Apache-2.0). All are compatible with
  any plausible project license.
- License texts are not copied into source headers until the project license is chosen; this
  task must not change headers.

## Slint Kit GPL-3.0-or-later compatibility gate

**Slint Kit adoption: NO — blocked until Paket's project license is decided.**

Compatibility verdict: **not established**. Slint Kit 0.9.0 is GPL-3.0-or-later, so it can
only be combined into a Paket binary that is distributed under GPL-3.0-or-later (or a
license the GPL explicitly permits). Paket currently has no license, so compatibility cannot
be claimed.

Evidence (checked 2026-11-24):

- Slint Kit's site states: "The base kit components remain free for open-source use under
  GPL-3.0-or-later. A commercial license for proprietary applications is coming, on request."
  <https://slint-kit.dev/>
- The project `LICENSE` is the GNU General Public License version 3 text:
  <https://github.com/damione1/slint-kit/blob/main/LICENSE>
- The same site records the Kit's constraints: version 0.9.0, Rust 1.95+, Slint 1.19.0.

How this unblocks:

1. The owner licenses Paket as GPL-3.0-or-later (or another GPLv3-compatible license) and
   the release process ships corresponding source and notices; then adoption is allowed.
2. Or a commercial Slint Kit license is obtained; its terms become the evidence.
3. Or Paket stays on standard Slint widgets without Kit components.

Until one of these holds, S145 must not add `slint-kit` or `slint_kit_build` dependencies,
and the UI uses Slint's standard widgets. Re-evaluate this gate at S145.

## Related plan decisions

- D5 gates Slint Kit adoption on license review; this record is that review.
- D3 pins Slint `1.19.0` and Slint Kit `0.9.0` when UI work begins.
- D4 sets the Rust minimum to `1.95` while Slint Kit `0.9.0` is selected.
