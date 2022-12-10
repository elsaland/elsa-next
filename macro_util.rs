#[macro_export]
macro_rules! cfg_v8 {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_v8")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_v8")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_jsc {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_jsc")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_jsc")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_mozjs {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_spidermonkey")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_spidermonkey")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_quickjs {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_quickjs")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_quickjs")))]
          $item
      )*
  }
}
