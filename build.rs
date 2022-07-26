fn main() {
    let jabcode_c_files = std::fs::read_dir("jabcode/src/jabcode")
        .unwrap()
        .into_iter()
        .map(|path| path.unwrap().path())
        .filter(|path| path.extension().is_some() && path.extension().unwrap() == "c");
    cc::Build::new()
        .include("jabcode/src/jabcode/include")
        .files(jabcode_c_files)
        .flag_if_supported("-O2")
        .flag_if_supported("-std=c11")
        .flag_if_supported("-w")
        .compile("jabcode");
}
