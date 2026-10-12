# GLib 0.18.5 compatibility security backport

Tauri/GTK3 requires the 0.18 API. Preserve the upstream MIT source and license and backport only the mutable C out-pointer fix from https://github.com/gtk-rs/gtk-rs-core/pull/1343 (RUSTSEC-2024-0429).

Changes to src/variant_iter.rs: `let mut p` and `&mut p` in VariantStrIter::impl_get. No unsafe iterator method is suppressed. `apps/desktop_pet/tests/glib_variant_safety.rs` exercises every affected method in optimized builds.

Keep RustSec reports visible: version-based auditing may continue to flag 0.18.5; this backport must be checked alongside those reports. This is not a claim that upstream 0.18 is maintained or that broader security validation is complete.
