fn main() {
    // fltk-sys 的 Fl_Native_File_Chooser_MAC.mm 在 macOS 11+ 上使用 UTType，
    // 需要链接 UniformTypeIdentifiers 框架，否则链接报 undefined symbol _OBJC_CLASS_$_UTType
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
    }
}
