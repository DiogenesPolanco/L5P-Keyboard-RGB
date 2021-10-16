project('legion-kb-rgb', 'rust', version: '0.7.0')

subdir('data')
subdir('src')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.source_root()
)