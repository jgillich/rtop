#![allow(dead_code)]
#![allow(unused_imports)]

use libc::{size_t, c_char, c_longlong, c_float, time_t};
use std::c_str::CString;

#[repr(C)]
pub struct cpu_stats {
    pub user: c_longlong,
    pub kernel: c_longlong,
    pub idle: c_longlong,
    pub iowait: c_longlong,
    pub swap: c_longlong,
    pub nice: c_longlong,
    pub total: c_longlong,
    pub systime: time_t
}

#[repr(C)]
pub struct cpu_percents {
    pub user: c_float,
    pub kernel: c_float,
    pub idle: c_float,
    pub iowait: c_float,
    pub swap: c_float,
    pub nice: c_float,
    pub time_taken: time_t
}
// sg_disk_io_stats

#[repr(C)]
struct sg_disk_io_stats {
    pub disk_name: *const c_char,
    pub read_bytes: c_longlong,
    pub write_bytes: c_longlong,
    pub systime: time_t
}

#[repr(C)]
pub struct disk_io_stats {
    pub disk_name: CString,
    pub read_bytes: c_longlong,
    pub write_bytes: c_longlong,
    pub systime: time_t
}


#[link(name = "statgrab")]
extern {
    fn sg_init() -> int;
    fn sg_get_cpu_stats() -> *const cpu_stats;
    fn sg_get_cpu_percents() -> *const cpu_percents;
    fn sg_get_disk_io_stats(entries: &mut int) -> *const sg_disk_io_stats;
    fn sg_get_disk_io_stats_diff(entries: &mut int) -> *const sg_disk_io_stats;
}

pub fn init() -> int {
    unsafe {
        sg_init()
    }
}

pub fn get_cpu_stats() -> cpu_stats {
    unsafe {
        *sg_get_cpu_stats()
    }
}


pub fn get_cpu_percents() -> cpu_percents {
    unsafe {
        *sg_get_cpu_percents()
    }
}

pub fn get_disk_io_stats() -> Vec<disk_io_stats> {
    unsafe {
        let mut entries: int = 0;
        let stats_ptr = sg_get_disk_io_stats(&mut entries);

        get_disk_io_stats_vector(entries, stats_ptr)
    }
}

pub fn get_disk_io_stats_diff() -> Vec<disk_io_stats> {
    unsafe {
        let mut entries: int = 0;
        let stats_ptr = sg_get_disk_io_stats_diff(&mut entries);

        get_disk_io_stats_vector(entries, stats_ptr)
    }
}

unsafe fn get_disk_io_stats_vector(entries: int, stats_ptr: *const sg_disk_io_stats) -> Vec<disk_io_stats> {
    let mut vec : Vec<disk_io_stats> = Vec::new();

    for i in range(0, entries) {
        let stats = *stats_ptr.offset(i);
        vec.push(disk_io_stats {
            disk_name: CString::new(stats.disk_name, false),
            read_bytes: stats.read_bytes,
            write_bytes: stats.write_bytes,
            systime: stats.systime
        });
    }

    vec
}
