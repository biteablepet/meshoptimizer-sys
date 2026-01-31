extern crate alloc;

use alloc::vec::Vec;

use assert2::assert;
use proptest::prelude::*;

use super::*;

proptest! {
    #[test]
    fn smoke_test_index_encode_decode(mut indices in prop::collection::vec(0..100u32, 3..1_000)) {
        let indices: &mut [u32] = indices.as_chunks_mut::<3>().0.as_flattened_mut();

        let mut encoded =
            Vec::with_capacity(unsafe { meshopt_encodeIndexBufferBound(indices.len(), 100) });

        let encoded_size = unsafe {
            meshopt_encodeIndexBuffer(
                encoded.as_mut_ptr(),
                encoded.capacity(),
                indices.as_ptr(),
                indices.len(),
            )
        };
        assert!(encoded_size != 0);
        unsafe {
            encoded.set_len(encoded_size);
        }

        let mut decoded: Vec<u32> = Vec::with_capacity(indices.len());

        let decoded_status = unsafe {
            meshopt_decodeIndexBuffer(
                decoded.as_mut_ptr().cast(),
                indices.len(),
                u32::BITS as usize / 8,
                encoded.as_ptr(),
                encoded.len(),
            )
        };
        assert!(decoded_status == 0);
        unsafe { decoded.set_len(indices.len()) };

        // because meshoptimizer can rotate triangles for efficiency, we sort every triangle
        // ourselves to remove any rotation
        indices.as_chunks_mut().0.iter_mut().for_each(|slice: &mut [u32; 3]| slice.sort_unstable());
        decoded.as_chunks_mut().0.iter_mut().for_each(|slice: &mut [u32; 3]| slice.sort_unstable());
        assert!(indices == decoded);
    }
}

#[test]
fn external_crates_can_use_functions() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile-pass/*.rs");
}
