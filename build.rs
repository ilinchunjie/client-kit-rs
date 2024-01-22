fn main() {
    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("ClientKit")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/download/download_extern.rs")
        .input_extern_file("src/ffi_string.rs")
        .generate_csharp_file("csharp/ClientKit.cs")
        .unwrap();
}