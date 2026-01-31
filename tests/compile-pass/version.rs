use meshoptimizer_sys;

fn main() {
    unsafe {
        meshoptimizer_sys::meshopt_encodeIndexVersion(1);
    }
}
