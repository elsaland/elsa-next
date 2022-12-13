use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Specifier = String;

crate::cfg_v8! {
  type Module = v8::Global<v8::Module>;
}

#[derive(Default)]
struct ModuleMap {
  inner: HashMap<Specifier, Module>,
}
