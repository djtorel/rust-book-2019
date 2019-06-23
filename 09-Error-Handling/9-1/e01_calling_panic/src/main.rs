fn main() {
    // adding panic = 'abort' to a cargo.toml file will force it to abort instead of unwind
    // for example:
    // [profile.release]
    // panic = 'abort'

    panic!("crash and burn");
}
