#![allow(dead_code)]


#[derive (Debug)]
pub enum CpuType {
    Intel8088,
    Intel8086,
}

impl Default for CpuType {
    fn default() -> Self { CpuType::Intel8088 }
}

use crate::cpu_808x::*;

pub mod alu;

impl<'a, 'b> Cpu<'a, 'b> {

    pub fn common_test(&self) {
        //log::trace!("I'm a common cpu function!");
    }
}