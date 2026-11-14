//! Gives `crisol.exe` its icon on Windows.

fn main() {
    println!("cargo:rerun-if-changed=crisol.rc");
    println!("cargo:rerun-if-changed=../../installer/crisol.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_resource::compile("crisol.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("the icon resource compiles");
    }
}
