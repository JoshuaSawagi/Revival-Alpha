#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]
#![feature(proc_macro_hygiene)]

extern crate dynamic;
extern crate locks;
extern crate once_cell;
extern crate rand;
extern crate skyline;
#[cfg(feature = "skyline-web")]
extern crate skyline_web;
extern crate smash;
extern crate smash_rs;
extern crate smash_arc;
extern crate ninput;
extern crate toml;

mod fighters;

use skyline::libc::c_char;
#[cfg(feature = "main_nro")]
use skyline_web::*;
use std::{fs, path::Path};


use std::os::raw::c_void;
#[cfg(feature = "main_nro")]
use utils::STAGE_MANAGER;
use std::sync::atomic::Ordering;
use dynamic::util::MATCH_EXITING;

#[cfg(not(feature = "main_nro"))]
#[unsafe(no_mangle)]
pub fn smashline_install() {
    fighters::install();
}


use skyline::hooks::InlineCtx;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;


#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(feature = "main_nro")]{}

    #[cfg(not(feature = "runtime"))]
    {utils::init();}

    fighters::install();
}