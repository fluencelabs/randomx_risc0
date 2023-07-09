#![no_main]

risc0_zkvm::guest::entry!(main);

// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

// use rust_randomx::Context;
// use rust_randomx::Hasher;
use risc0_zkvm::guest::env;

use host_guest_interface::Interface;

/*
#[no_mangle]
pub extern "C" fn strtoul(nptr: u32, endptr: u32, base: i32) -> u32 {
        0
}

#[no_mangle]
pub extern "C" fn strcmp() {
}

#[no_mangle]
pub extern "C" fn malloc(_size: u32) -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn free(_ptr: u32) {
}
*/

#[no_mangle]
pub extern "C" fn _Unwind_Resume(_ptr: u32) {
    println!("external: Unwind_Resume");
}

#[no_mangle]
pub extern "C" fn _Unwind_DeleteException() {
    println!("external: _Unwind_DeleteException");
}

/*
#[no_mangle]
pub extern "C" fn abort() {
}
*/

#[no_mangle]
pub extern "C" fn _Unwind_GetTextRelBase() {
    println!("external: unwind text rel base");
}

#[no_mangle]
pub extern "C" fn _Unwind_GetDataRelBase() {
    println!("external: unwind text get data rel base");
}

#[no_mangle]
pub extern "C" fn _Unwind_GetRegionStart() {
    println!("external: unwind get region start");
}

#[no_mangle]
pub extern "C" fn _Unwind_GetLanguageSpecificData() {
    println!("external: unwind get lang specific data");
}

#[no_mangle]
pub extern "C" fn _Unwind_GetIPInfo() {
    println!("external: unwind get ip info");
}

#[no_mangle]
pub extern "C" fn _Unwind_SetGR() {
    println!("external: unwind set gr");
}

#[no_mangle]
pub extern "C" fn _Unwind_SetIP() {
    println!("external: unwind set ip");
}

#[no_mangle]
pub extern "C" fn _Unwind_RaiseException() {
    println!("external: unwind raise exception");
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume_or_Rethrow() {
    println!("external: unwind resume or rethrow");
}

#[no_mangle]
pub extern "C" fn allocLargePagesMemory() {
    println!("external: alloc large pages memory");
}

#[no_mangle]
pub extern "C" fn freePagedMemory() {
    println!("external: free paged memory");
}

#[no_mangle]
pub extern "C" fn stdout() {
    println!("external: stdout");
}

#[no_mangle]
pub extern "C" fn stdin() {
    println!("external: stdin");
}

#[no_mangle]
pub extern "C" fn stderr() {
    println!("external: stderr");
}

#[no_mangle]
pub extern "C" fn __eqtf2() {
    println!("external: __eqtf2");
}

#[no_mangle]
pub extern "C" fn _exit(code: i32) {
    risc0_zkvm::guest::abort(&format!("abort({code})"));
}

#[no_mangle]
pub extern "C" fn _fstat() {
    println!("external: _fstat");
}

#[no_mangle]
pub extern "C" fn print_number(number: i32) {
    println!("syscall: print_number(0x{:x})", number);
}


fn sys_alloc_aligned(bytes: usize, align: usize) -> *mut u8 {
    unsafe {
        extern "C" {
            static _end: u8;
        }

        static mut HEAP_POS: usize = 0;
        let mut heap_pos = HEAP_POS;

        if heap_pos == 0 {
            heap_pos = (&_end) as *const u8 as usize;
        }
        
        if heap_pos & 0xFFF != 0 {
            heap_pos = (heap_pos + (0x1000 - 1)) & !(0x1000 - 1)  
        }

        let offset = heap_pos & (align - 1);
        if offset != 0 {
            heap_pos += align - offset;
        }

        let ptr = heap_pos as *mut u8;
        heap_pos += bytes;

        HEAP_POS = heap_pos;
        ptr
    }
}

#[no_mangle]
pub extern "C" fn _sbrk(shift: usize) -> *mut u8 {
    let page_size = 0x1000;
    let rounded_shift = (shift + (page_size - 1)) & !(page_size - 1);
    let result = sys_alloc_aligned(rounded_shift, 4);
    println!("syscall: _sbrk(0x{:x} => ({:x})) => {:?}", shift, rounded_shift, result);
    result
    /*
    unsafe {
        let prev_limit = LIMIT;
        println!("external: _sbrk({}) => {}", shift, prev_limit);
        LIMIT += shift;
        
        LIMIT
    }
    */
}

#[no_mangle]
pub extern "C" fn getpid() {
    println!("external: getpid");
}

#[no_mangle]
pub extern "C" fn __extenddftf2() {
    println!("external: __extenddftf2");
}

#[no_mangle]
pub extern "C" fn __netf2() {
    println!("external: __netf2");
}

#[no_mangle]
pub extern "C" fn __lttf2() {
    println!("external: __lttf2");
}

#[no_mangle]
pub extern "C" fn __trunctfdf2() {
    println!("external: __trunctfdf2");
}

#[no_mangle]
pub extern "C" fn __multf3() {
    println!("external: __multf3");
}

#[no_mangle]
pub extern "C" fn __fixtfsi() {
    println!("external: __fixtfsi");
}

#[no_mangle]
pub extern "C" fn __floatsitf() {
    println!("external: __floatsitf");
}

#[no_mangle]
pub extern "C" fn __subtf3() {
    println!("external: __subtf3");
}

#[no_mangle]
pub extern "C" fn __gttf2() {
    println!("external: __gttf2");
}

#[no_mangle]
pub extern "C" fn _write(handle: i32, buf: u32, count: i32) -> i32 {
    println!("external: _write({} 0x{:x} {})", handle, buf, count);
    count
}

#[no_mangle]
pub extern "C" fn _kill() {
    println!("external: _kill");
}

#[no_mangle]
pub extern "C" fn _getpid() {
    println!("external: _getpid");
}

#[no_mangle]
pub extern "C" fn _close() {
    println!("external: _close");
}

#[no_mangle]
pub extern "C" fn _isatty() {
    println!("external: _isatty");
}

#[no_mangle]
pub extern "C" fn _lseek() {
    println!("external: _lseek");
}

#[no_mangle]
pub extern "C" fn _read() {
    println!("external: _read");
}

/*
#[no_mangle]
pub extern "C" fn fegetenv(_ptr: u32) -> i32 {
    println!("external: fegetenv");
    0
}

#[no_mangle]
pub extern "C" fn fesetenv(_ptr: u32) -> i32 {
    println!("external: fesetenv");
    0
}
*/

#[no_mangle]
pub extern "C" fn fesetround() {
    println!("external: fesetround");
}

pub fn main() {
    unsafe { main1() }
}

pub unsafe fn main1() {
    println!("guest: started");
    let interface: Interface = env::read();

    println!("guest: read interface {:?}", interface);

    // let context = Context::new(interface.key.as_bytes(), true);
    let mut flags = randomx_get_flags();
    println!("guest: flags are {}", flags);
    let mut cache = randomx_alloc_cache(flags);
    println!("guest: cache is allocated {:?}", cache);
    let key = interface.key.as_bytes();
    randomx_init_cache(
        cache,
        key.as_ptr() as *const std::os::raw::c_void,
        key.len() as u32,
    );

    println!("guest: cache initialized");

    let mut dataset = std::ptr::null_mut();
    println!("guest: dataset initialized");

    //let hasher = Hasher::new(Arc::new(context));
    let vm = randomx_create_vm(flags, cache, dataset);

    println!("guest: vm created");

    //let output = hasher.hash(&interface.nonce.to_le_bytes());
    let nonce = interface.nonce.to_le_bytes();
    let mut hash = [0u8; RANDOMX_HASH_SIZE as usize];
    randomx_calculate_hash(
        vm,
        nonce.as_ptr() as *const std::os::raw::c_void,
        nonce.len() as u32,
        hash.as_mut_ptr() as *mut std::os::raw::c_void,
    );

    env::commit(&hash)
}

pub const RANDOMX_HASH_SIZE: u32 = 32;

pub type size_t = ::std::os::raw::c_ulong;
pub type randomx_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randomx_dataset {
        _unused: [u8; 0],
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randomx_cache {
        _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randomx_vm {
        _unused: [u8; 0],
}
extern "C" {
        #[doc = " @return The recommended flags to be used on the current machine."]
        #[doc = "         Does not include:"]
        #[doc = "            RANDOMX_FLAG_LARGE_PAGES"]
        #[doc = "            RANDOMX_FLAG_FULL_MEM"]
        #[doc = "            RANDOMX_FLAG_SECURE"]
        #[doc = "         These flags must be added manually if desired."]
        #[doc = "         On OpenBSD RANDOMX_FLAG_SECURE is enabled by default in JIT mode as W^X is enforced by the OS."]
        pub fn randomx_get_flags() -> randomx_flags;
}
extern "C" {
        #[doc = " Creates a randomx_cache structure and allocates memory for RandomX Cache."]
        #[doc = ""]
        #[doc = " @param flags is any combination of these 2 flags (each flag can be set or not set):"]
        #[doc = "        RANDOMX_FLAG_LARGE_PAGES - allocate memory in large pages"]
        #[doc = "        RANDOMX_FLAG_JIT - create cache structure with JIT compilation support; this makes"]
        #[doc = "                           subsequent Dataset initialization faster"]
        #[doc = "        Optionally, one of these two flags may be selected:"]
        #[doc = "        RANDOMX_FLAG_ARGON2_SSSE3 - optimized Argon2 for CPUs with the SSSE3 instruction set"]
        #[doc = "                                   makes subsequent cache initialization faster"]
        #[doc = "        RANDOMX_FLAG_ARGON2_AVX2 - optimized Argon2 for CPUs with the AVX2 instruction set"]
        #[doc = "                                   makes subsequent cache initialization faster"]
        #[doc = ""]
        #[doc = " @return Pointer to an allocated randomx_cache structure."]
        #[doc = "         Returns NULL if:"]
        #[doc = "         (1) memory allocation fails"]
        #[doc = "         (2) the RANDOMX_FLAG_JIT is set and JIT compilation is not supported on the current platform"]
        #[doc = "         (3) an invalid or unsupported RANDOMX_FLAG_ARGON2 value is set"]
        pub fn randomx_alloc_cache(flags: randomx_flags) -> *mut randomx_cache;
}
extern "C" {
        #[doc = " Initializes the cache memory and SuperscalarHash using the provided key value."]
        #[doc = " Does nothing if called again with the same key value."]
        #[doc = ""]
        #[doc = " @param cache is a pointer to a previously allocated randomx_cache structure. Must not be NULL."]
        #[doc = " @param key is a pointer to memory which contains the key value. Must not be NULL."]
        #[doc = " @param keySize is the number of bytes of the key."]
        pub fn randomx_init_cache(
                    cache: *mut randomx_cache,
                            key: *const ::std::os::raw::c_void,
                                    keySize: size_t,
                                        );
}
extern "C" {
        #[doc = " Creates and initializes a RandomX virtual machine."]
        #[doc = ""]
        #[doc = " @param flags is any combination of these 5 flags (each flag can be set or not set):"]
        #[doc = "        RANDOMX_FLAG_LARGE_PAGES - allocate scratchpad memory in large pages"]
        #[doc = "        RANDOMX_FLAG_HARD_AES - virtual machine will use hardware accelerated AES"]
        #[doc = "        RANDOMX_FLAG_FULL_MEM - virtual machine will use the full dataset"]
        #[doc = "        RANDOMX_FLAG_JIT - virtual machine will use a JIT compiler"]
        #[doc = "        RANDOMX_FLAG_SECURE - when combined with RANDOMX_FLAG_JIT, the JIT pages are never"]
        #[doc = "                              writable and executable at the same time (W^X policy)"]
        #[doc = "        The numeric values of the first 4 flags are ordered so that a higher value will provide"]
        #[doc = "        faster hash calculation and a lower numeric value will provide higher portability."]
        #[doc = "        Using RANDOMX_FLAG_DEFAULT (all flags not set) works on all platforms, but is the slowest."]
        #[doc = " @param cache is a pointer to an initialized randomx_cache structure. Can be"]
        #[doc = "        NULL if RANDOMX_FLAG_FULL_MEM is set."]
        #[doc = " @param dataset is a pointer to a randomx_dataset structure. Can be NULL"]
        #[doc = "        if RANDOMX_FLAG_FULL_MEM is not set."]
        #[doc = ""]
        #[doc = " @return Pointer to an initialized randomx_vm structure."]
        #[doc = "         Returns NULL if:"]
        #[doc = "         (1) Scratchpad memory allocation fails."]
        #[doc = "         (2) The requested initialization flags are not supported on the current platform."]
        #[doc = "         (3) cache parameter is NULL and RANDOMX_FLAG_FULL_MEM is not set"]
        #[doc = "         (4) dataset parameter is NULL and RANDOMX_FLAG_FULL_MEM is set"]
        pub fn randomx_create_vm(
                    flags: randomx_flags,
                            cache: *mut randomx_cache,
                                    dataset: *mut randomx_dataset,
                                        ) -> *mut randomx_vm;
}
extern "C" {
        #[doc = " Calculates a RandomX hash value."]
        #[doc = ""]
        #[doc = " @param machine is a pointer to a randomx_vm structure. Must not be NULL."]
        #[doc = " @param input is a pointer to memory to be hashed. Must not be NULL."]
        #[doc = " @param inputSize is the number of bytes to be hashed."]
        #[doc = " @param output is a pointer to memory where the hash will be stored. Must not"]
        #[doc = "        be NULL and at least RANDOMX_HASH_SIZE bytes must be available for writing."]
        pub fn randomx_calculate_hash(
                    machine: *mut randomx_vm,
                            input: *const ::std::os::raw::c_void,
                                    inputSize: size_t,
                                            output: *mut ::std::os::raw::c_void,
                                                );
}
