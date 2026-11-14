pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())

conf = configuration_data()
conf.set_quoted('GETTEXT_PACKAGE', 'beat')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)

cargo_bin = find_program('cargo')
cargo_target_dir = meson.project_build_root() / 'src'

cargo_release_opt = ''
if get_option('buildtype') == 'release'
  cargo_release_opt = '--release'
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'sh', '-c',
    'env CARGO_HOME="$1" "$2" build --manifest-path "$3" --target-dir "$4" $5 && cp "$6" "$7"',
    'sh',
    meson.project_build_root() / 'cargo-home',
    cargo_bin.full_path(),
    meson.project_source_root() / 'Cargo.toml',
    cargo_target_dir,
    cargo_release_opt,
    cargo_target_dir / rust_target / meson.project_name(),
    '@OUTPUT@',
  ]
)
