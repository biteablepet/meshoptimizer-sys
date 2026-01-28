#![no_std]
#![expect(non_upper_case_globals)]
#![expect(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests;
