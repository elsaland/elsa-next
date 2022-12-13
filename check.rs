use stc_ts_builtin_types::Lib;
use stc_ts_env::Env;
use stc_ts_env::ModuleConfig;
use stc_ts_env::Rule;
use stc_ts_file_analyzer::env::EnvFactory;
use stc_ts_module_loader::resolvers::node::NodeResolver;
use stc_ts_type_checker::Checker;
use swc_common::errors::ColorConfig;
use swc_common::errors::EmitterWriter;
use swc_common::errors::Handler;
use swc_common::FileName;
use swc_common::SourceMap;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::TsConfig;

use std::path::PathBuf;
use std::sync::Arc;

pub fn check(file: &str) {
  let cm = Arc::new(SourceMap::default());
  let handler = {
    let emitter = Box::new(EmitterWriter::stderr(
      ColorConfig::Always,
      Some(cm.clone()),
      false,
      false,
    ));
    Arc::new(Handler::with_emitter(true, false, emitter))
  };

  let libs = Lib::load("es5");
  let env = Env::simple(
    Rule {
      ..Default::default()
    },
    EsVersion::latest(),
    ModuleConfig::None,
    &libs,
  );
  let path = PathBuf::from(file);

  let mut checker = Checker::new(
    cm.clone(),
    handler.clone(),
    env.clone(),
    TsConfig {
      ..Default::default()
    },
    None,
    Arc::new(NodeResolver),
  );

  checker.check(Arc::new(FileName::Real(path)));
  for err in checker.take_errors() {
    err.emit(&handler);
  }
}
