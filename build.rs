fn main() {
    // Tell Cargo that if any file in the migrations/postgres directory changes,
    // or if files are added/removed, it must rerun the build and re-evaluate macros.
    println!("cargo:rerun-if-changed=migrations/postgres");
}