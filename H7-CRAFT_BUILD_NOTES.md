# H7-CRAFT Source Build Notes

This source tree keeps the upstream Cargo workspace and internal package IDs for compatibility. The product-facing desktop and web titles use H7-CRAFT. Installer manifests, platform app IDs and the inherited CADCraft icon assets still use upstream identifiers and should not be used to publish distributable H7-CRAFT installers until they have been reviewed and replaced with original, properly-attributed H7-CRAFT artwork.

## Build requirements

- Rust toolchain 1.95+ recommended by the upstream desktop/web code.
- `cargo run --release -p cadcraft -- --sample` from the repository root.
- For verification, run the repository's documented `cargo xtask ci` quality gates, plus the `trim_segment_anywhere` regression tests.

## Implemented command

`TRIMSEGMENT` takes `handle`, `p1`, and `p2`, and removes the between-points part of a line, arc, circle, or open lightweight polyline. Both points must lie on the selected geometry. Test input example:

```json
{"handle":"<line-handle>","p1":[3,0],"p2":[7,0]}
```

The command uses existing engine geometry and undo/redo transactions rather than pixel-only screen operations. Ellipse/spline trimming remains explicitly unsupported in this first upgrade.
