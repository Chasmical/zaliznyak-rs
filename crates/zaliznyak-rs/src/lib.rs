#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(core_intrinsics)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(const_from)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_option_ops)]
#![feature(const_result_trait_fn)]
#![feature(const_eval_select)]
#![feature(vec_into_raw_parts)]
#![allow(internal_features, confusable_idents)]

pub mod alphabet;
pub mod categories;
pub mod declension;
pub mod inflection_buf;
pub mod stress;

mod util;
