fn main() {
    csbindgen::Builder::default()
        .csharp_class_accessibility("public")
        .csharp_class_name("FFIWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/ffi.rs")
        .generate_csharp_file("csharp/FFIWrap.cs")
        .unwrap();

    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("ByteBufferWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/byte_buffer.rs")
        .generate_csharp_file("csharp/ByteBufferWrap.cs")
        .unwrap();

    // download
    csbindgen::Builder::default()
        .csharp_class_accessibility("public")
        .csharp_class_name("DownloadConfigWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/download/download_config_extern.rs")
        .generate_csharp_file("csharp/Downloader/DownloadConfigWrap.cs")
        .unwrap();

    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("DownloadOperationWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/download/download_operation_extern.rs")
        .generate_csharp_file("csharp/Downloader/DownloadOperationWrap.cs")
        .unwrap();

    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("DownloadServiceWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/download/download_service_extern.rs")
        .generate_csharp_file("csharp/Downloader/DownloadServiceWrap.cs")
        .unwrap();
    // download

    // vfs
    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("ZipArchiveWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/vfs/zip_archive_extern.rs")
        .generate_csharp_file("csharp/ZipArchive/ZipArchiveWrap.cs")
        .unwrap();

    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("ZipFileWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/vfs/zip_file_extern.rs")
        .generate_csharp_file("csharp/ZipArchive/ZipFileWrap.cs")
        .unwrap();
    // vfs

    //debug
    csbindgen::Builder::default()
        .csharp_class_accessibility("internal")
        .csharp_class_name("DebugWrap")
        .csharp_namespace("URS.ClientKit")
        .csharp_dll_name("clientkit")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("")
        .csharp_method_prefix("")
        .input_extern_file("src/unity_debug/debug_extern.rs")
        .generate_csharp_file("csharp/Debug/DebugWrap.cs")
        .unwrap();
    //debug
}