fn main() {
    glib_build_tools::compile_resources(
        "data/resources/",
        "data/resources/beat.gresource.xml",
        "beat.gresource",
    );
}