gtk_rest_sources = [
  'main.vala',
  'application.vala',
  'window.vala',
  'widgets/about.vala'
]

gtk_rest_deps = [
  dependency('gtk4'),
  dependency('libadwaita-1'),
]

gtk_rest_sources += gnome.compile_resources(
  'gtk-rest-resources',
  'gtk-rest.gresource.xml',
  c_name: 'gtk_rest'
)

executable('gtk-rest', gtk_rest_sources,
  dependencies: gtk_rest_deps,
       install: true,
)
