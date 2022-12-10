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
macro_rules! cfg_quickjs {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_quickjs")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_quickjs")))]
          $item
      )*
  }
}
