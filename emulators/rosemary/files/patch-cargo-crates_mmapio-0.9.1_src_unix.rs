--- cargo-crates/mmapio-0.9.1/src/unix.rs.orig	2026-10-30 01:06:22 UTC
+++ cargo-crates/mmapio-0.9.1/src/unix.rs
@@ -18,19 +18,7 @@ const MAP_STACK: libc::c_int = 0;
 )))]
 const MAP_STACK: libc::c_int = 0;
 
-
-#[cfg(any(
-all(target_os = "linux", not(target_arch = "mips")),
-target_os = "freebsd",
-target_os = "android"
-))]
-const MAP_LOCKED: libc::c_int = libc::MAP_LOCKED;
-
-#[cfg(not(any(
-all(target_os = "linux", not(target_arch = "mips")),
-target_os = "freebsd",
-target_os = "android"
-)))]
+/// I am well aware of the laziness committed here.
 const MAP_LOCKED: libc::c_int = 0;
 
 pub struct MmapInner {
