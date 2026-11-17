// Integration test for SigmaOS-etc-shadow
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use etc_shadow as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
