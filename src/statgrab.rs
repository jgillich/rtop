#![allow(dead_code)]
#![allow(unused_imports)]

use libc::{size_t, c_long, c_float, time_t};

#[repr(C)]
struct sg_cpu_stats {
    pub user: c_long,
    pub kernel: c_long,
    pub idle: c_long,
    pub iowait: c_long,
    pub swap: c_long,
    pub nice: c_long,
    pub total: c_long,
    pub systime: time_t
}

#[repr(C)]
struct sg_cpu_percents {
    pub user: c_float,
    pub kernel: c_float,
    pub idle: c_float,
    pub iowait: c_float,
    pub swap: c_float,
    pub nice: c_float,
    pub time_taken: time_t
}

#[link(name = "statgrab")]
extern {
    fn sg_get_cpu_stats() -> *mut sg_cpu_stats;
    fn sg_get_cpu_percents() -> *mut sg_cpu_percents;
}

pub fn get_cpu_stats() -> sg_cpu_stats {
    unsafe {
        *sg_get_cpu_stats()
    }
}


pub fn get_cpu_percents() -> sg_cpu_percents {
    unsafe {
        *sg_get_cpu_percents()
    }
}
