#![cfg_attr(test, feature(test))]
// Const traits features
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(const_cmp)]
#![feature(const_ops)]
#![feature(const_try)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_option_ops)]
#![feature(const_result_trait_fn)]
// Workaround for const PartialEq on enums
#![allow(internal_features)]
#![feature(core_intrinsics)]
// Allow Russian letters in Utf8Letter enum
#![allow(confusable_idents)]

pub mod categories;
pub mod word;

mod util;
