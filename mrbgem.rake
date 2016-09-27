MRuby::Gem::Specification.new('mferuby-runtime') do |spec|
  spec.license = 'MIT'
  spec.authors = 'Terence Lee'
  spec.version = '0.0.1'
  spec.description = 'The mferuby Runtime'

  FileUtils.mkdir_p build_dir
  rust_o_path = "#{build.build_dir}/mrbgems/#{spec.name}/src/mferuby_runtime.o"
  spec.linker.flags_before_libraries << rust_o_path unless spec.linker.flags_before_libraries.include?(rust_o_path)
end
