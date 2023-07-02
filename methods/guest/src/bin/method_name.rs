#![no_main]

risc0_zkvm::guest::entry!(main);

// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

mod bindings;

// use rust_randomx::Context;
// use rust_randomx::Hasher;
use risc0_zkvm::guest::env;

use host_guest_interface::Interface;

pub fn main() {
    unsafe { main() }
}

pub unsafe fn main1() {
    println!("started");
    let interface: Interface = env::read();

    println!("read interface {:?}", interface);

    // let context = Context::new(interface.key.as_bytes(), true);
    let mut flags = bindings::randomx_get_flags();
    flags = flags | bindings::randomx_flags_RANDOMX_FLAG_FULL_MEM;
    let mut cache = bindings::randomx_alloc_cache(flags);
    let key = interface.key.as_bytes();
    bindings::randomx_init_cache(
        cache,
        key.as_ptr() as *const std::os::raw::c_void,
        key.len() as u32,
    );

    println!("cache initialized");

    let mut dataset = bindings::randomx_alloc_dataset(flags);
    let length = bindings::randomx_dataset_item_count() as usize;
    bindings::randomx_init_dataset(dataset, cache, length as u32, length as u32);

    println!("dataset initialized");

    //let hasher = Hasher::new(Arc::new(context));
    let vm = bindings::randomx_create_vm(flags, cache, dataset);

    //let output = hasher.hash(&interface.nonce.to_le_bytes());
    let nonce = interface.nonce.to_le_bytes();
    let mut hash = [0u8; bindings::RANDOMX_HASH_SIZE as usize];
    bindings::randomx_calculate_hash(
        vm,
        nonce.as_ptr() as *const std::os::raw::c_void,
        nonce.len() as u32,
        hash.as_mut_ptr() as *mut std::os::raw::c_void,
    );

    env::commit(&hash)
}
