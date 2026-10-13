# H7-CRAFT — Implementation Report

**Date:** 13 October 2026  
**Basis:** `H7-CRAFT_Complete_2D_Feature_Audit_and_Upgrade_Specification.docx` and supplied `cadcraft-main.zip`.  
**Identity:** [@HimanshuiRajj](https://github.com/HimanshuiRajj)  
**Scope:** 2D architectural/civil drafting only. No 3D UI is exposed by this H7-CRAFT desktop drafting configuration.

## Changes made in this first upgrade

1. **New Trim Segment (Anywhere) command**
   - Command: `TRIMSEGMENT`; aliases: `TRSEG`, `TRIMSEGMENTANYWHERE`; shortcut hint: `TS`.
   - Picks an entity and two cut points. No cutting boundary is required.
   - Removes the between-points segment using the existing analytic BREAK geometry path for **lines, arcs, circles and open lightweight polylines**.
   - Validates both points against selected geometry using a scale-aware tolerance. Off-entity picks are rejected and leave the drawing unchanged.
   - Locked-layer edits are rejected by the shared BREAK implementation.
   - Unsupported entity types return an explicit error; ellipse/spline trim-anywhere is not claimed as implemented.
   - Added engine regression tests for arbitrary line-segment removal, undo/redo, bad pick rejection and locked-layer refusal.

2. **Core Modify tools are easier to find**
   - Added Trim Segment (Anywhere) to the Modify Tool Set.
   - Moved Eraser / Erase close to Trim in the Modify Tool Set.
   - Exposed Trim, Trim Segment (Anywhere) and Eraser / Erase in the desktop toolbar; toolbar clicks route through the registered engine commands.
   - ERASE stays a distinct selectable command and does not get conflated with BREAK or Trim Segment.

3. **H7-CRAFT product-facing branding**
   - Desktop window title, in-app title row, start-page heading, About/Quit/help menu labels, web document title and loading message use `H7-CRAFT`.
   - README repositions the application as a 2D-first architectural/civil drafting variant and links to the requested GitHub profile.
   - Kept crate/package IDs, environment-variable names, settings locations, upstream notices and repository identifiers unchanged to protect existing builds and user settings.
   - Removed 3D Modeling tab/tools from the H7-CRAFT drafting Tool Sets and removed Orbit from the main toolbar. Internal 3D entity/file-format compatibility code remains untouched; this is not a claim that every underlying 3D engine type has been deleted.

4. **Dimensions/grips: audited, not duplicated**
   - The existing engine already contains `DIMEDIT` text overrides (including the `<>` measured-value token), dimension style/override commands, property editing and hot-grip/edit infrastructure. Existing tests show a dimension override such as `<> TYP` renders from the actual measured value.
   - I have not added a parallel dimension system. Full UI end-to-end validation of manual entry, measured-vs-display properties and save/reopen is still required in a built app.

## Tests and validation status

- Added `crates/engine/tests/trim_segment_anywhere.rs` with three regression tests.
- Static review completed for command registration, toolbar wiring, Tool Set wiring, branding replacements and original licence terms.
- **Not compiled or executed here:** the environment does not have `cargo`/`rustc` installed (`cargo: command not found`). The Rust regression tests must be run on a Rust 1.95+ environment using the repository's normal CI gates before considering these changes release-ready.

## Remaining DOCX requirements (not claimed as completed)

- Build and run the desktop/web app, verify keyboard/toolbar/context-menu interaction and snapshot UI tests.
- Implement a full parameterized architectural symbol browser (door/window/gate/wall/stair/furniture/sanitary/electrical/site symbols), room area labels and wall-opening repair tools.
- Complete typed dynamic input, snap cycling/tracking, point/vertex grip tests across mixed line-arc polylines, and dimension property inspector UI for true measurement vs override state.
- Implement ellipse/spline parameter trimming without tessellation-based geometry loss.
- Complete dimension style manager parity, text/MTEXT, fields, leaders, tables, hatch repair and annotation testing.
- Complete layout/plot/save-as/underlay parity and explicitly lossy file-format warnings.
- Verify lossless unsupported-object handling, autosave/recovery, open-edit-save-reopen corpus tests and large-drawing performance.
- Implement blocks/xrefs/compare/markup/data extraction and alias/profile customization where useful.
- Run `cargo xtask ci` and verify wasm build. Neither can be honestly marked as passed until a Rust toolchain and build dependencies are available.

## Usage example

Create a line from `(0,0)` to `(10,0)`, then run:

```json
{"handle":"<line-handle>","p1":[3,0],"p2":[7,0]}
```

with command `trimsegment` (or type `TRSEG`/`TS`). The result retains the two end pieces `[0,3]` and `[7,10]`. Use OSNAP endpoints/intersections when entering precise cut points.

## Attribution / licensing

This is derived from CADCraft. Keep `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md`, and third-party notices intact. The bundled ArtCraft marks are not open source and are not used to brand this modified version. The project/package names remain unchanged internally for compatibility; product-facing labels use H7-CRAFT.
