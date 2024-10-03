fn main() {
    build_fathom();
    generate_bindings();
}

fn build_fathom() {
    let cc = &mut cc::Build::new();
    cc.file("./imports/fathom/src/tbprobe.c");
    cc.include("./imports/fathom/src/");
    cc.define("_CRT_SECURE_NO_WARNINGS", None);


    if std::env::consts::OS == "windows" {
        cc.compiler("clang");
    }

    cc.compile("fathom");
}


fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("./imports/fathom/src/tbprobe.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .generate().unwrap();

    bindings.write_to_file("./src/syzygy/bindings.rs").unwrap();
}