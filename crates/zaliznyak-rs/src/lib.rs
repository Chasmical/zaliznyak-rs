#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(core_intrinsics)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(const_from)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_deref)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_option_ops)]
#![feature(const_result_trait_fn)]
#![feature(const_eval_select)]
#![feature(box_vec_non_null)]
#![feature(vec_into_raw_parts)]
#![feature(str_from_raw_parts)]
#![feature(maybe_uninit_slice)]
#![cfg_attr(test, feature(test))]
#![allow(internal_features, confusable_idents)]

pub mod adjective;
pub mod categories;
pub mod declension;
pub mod noun;
pub mod pronoun;
pub mod stress;
pub mod word;

mod util;
