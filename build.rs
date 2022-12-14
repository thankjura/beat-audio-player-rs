fn main() {
    glib_build_tools::compile_resources(
        "resources/",
        "resources/beat.gresource.xml",
        "beat.gresource",
    );
}