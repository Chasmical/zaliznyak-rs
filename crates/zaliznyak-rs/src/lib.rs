#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(core_intrinsics)]
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
#![feature(const_eval_select)]
#![feature(box_vec_non_null)]
#![feature(vec_into_raw_parts)]
#![feature(str_from_raw_parts)]
#![feature(maybe_uninit_slice)]
#![feature(ptr_cast_array)]
#![feature(cast_maybe_uninit)]
#![feature(slice_from_ptr_range)]
#![feature(const_slice_from_ptr_range)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![cfg_attr(test, feature(test))]
#![allow(internal_features, confusable_idents)]
#![allow(clippy::deref_addrof)]

pub mod adjective;
pub mod categories;
pub mod declension;
pub mod noun;
pub mod pronoun;
pub mod stress;
pub mod word;

mod util;
