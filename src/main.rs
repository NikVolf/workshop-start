extern crate tiny_keccak;
extern crate bigint;

#[cfg(test)]
mod tests;

fn main() {
    println!("Welcome to the Parity WebAssembly Workshop!");
}

/// This function takes string of length `source_len` at memory indicated
/// by the raw pointer `source_ptr`, runs 10001 rounds of keccak-256 on it
/// and writes the result (of length 32) to memory indicated by the
/// raw pointer `dest_ptr`
#[no_mangle]
pub fn brain_wallet_derive(_source_ptr: *const u8, _source_len: u32, _dest_ptr: *mut u8) {
}

/// This function takes the 256-bit (32 bytes) integer from the memory
/// at the location specified by `source_ptr`, makes modular exponentation
/// to the power of 1000000 mod 190336703473395182854426616575356495301 and writes result
/// to memory at `dest_ptr`.
/// input and result are both in little-endian
#[no_mangle]
pub fn modexp(_source_ptr: *const u8, _dest_ptr: *mut u8) {
}