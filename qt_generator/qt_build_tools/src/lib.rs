extern crate cpp_to_rust_build_tools;
extern crate cpp_to_rust_common;
extern crate qt_generator_common;

use cpp_to_rust_build_tools::Config;
use cpp_to_rust_common::errors::{fancy_unwrap, Result};
use cpp_to_rust_common::target;
use cpp_to_rust_common::cpp_build_config::CppBuildConfigData;
use qt_generator_common::{get_installation_data, real_lib_name, framework_name, lib_dependencies};

pub fn run_and_return(sublib_name: &str) -> Result<()> {

  let mut config = Config::new()?;
  let installation_data = get_installation_data(sublib_name)?;
  config.cpp_build_paths_mut().add_include_path(&installation_data.root_include_path);
  config.cpp_build_paths_mut().add_include_path(&installation_data.lib_include_path);
  let mut cpp_build_config_data = CppBuildConfigData::new();
  if installation_data.is_framework {
    config.cpp_build_paths_mut().add_framework_path(&installation_data.lib_path);
    cpp_build_config_data.add_linked_framework(framework_name(sublib_name));
  } else {
    config.cpp_build_paths_mut().add_lib_path(&installation_data.lib_path);
    cpp_build_config_data.add_linked_lib(real_lib_name(sublib_name));
    if target::current_env() == target::Env::Msvc {
      // we use shared libraries on MSVC, and apparently
      // this requires us to link against dependencies as well as the main library
      for dep in lib_dependencies(sublib_name)? {
        cpp_build_config_data.add_linked_lib(real_lib_name(dep));
      }
    }
  }
  config.cpp_build_config_mut().add(target::Condition::True, cpp_build_config_data);
  config.run_and_return()
}

pub fn run(sublib_name: &str) -> ! {
  fancy_unwrap(run_and_return(sublib_name));
  std::process::exit(0)
}