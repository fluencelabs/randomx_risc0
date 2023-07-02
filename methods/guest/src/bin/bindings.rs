/* automatically generated by rust-bindgen */
#![allow(dead_code)]
#![no_main]

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const RANDOMX_HASH_SIZE: u32 = 32;
pub const RANDOMX_DATASET_ITEM_SIZE: u32 = 64;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub const randomx_flags_RANDOMX_FLAG_DEFAULT: randomx_flags = 0;
pub const randomx_flags_RANDOMX_FLAG_LARGE_PAGES: randomx_flags = 1;
pub const randomx_flags_RANDOMX_FLAG_HARD_AES: randomx_flags = 2;
pub const randomx_flags_RANDOMX_FLAG_FULL_MEM: randomx_flags = 4;
pub const randomx_flags_RANDOMX_FLAG_JIT: randomx_flags = 8;
pub const randomx_flags_RANDOMX_FLAG_SECURE: randomx_flags = 16;
pub const randomx_flags_RANDOMX_FLAG_ARGON2_SSSE3: randomx_flags = 32;
pub const randomx_flags_RANDOMX_FLAG_ARGON2_AVX2: randomx_flags = 64;
pub const randomx_flags_RANDOMX_FLAG_ARGON2: randomx_flags = 96;
pub type randomx_flags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randomx_dataset {
    _unused: [u8; 0],
}
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
    #[doc = " Releases all memory occupied by the randomx_cache structure."]
    #[doc = ""]
    #[doc = " @param cache is a pointer to a previously allocated randomx_cache structure."]
    pub fn randomx_release_cache(cache: *mut randomx_cache);
}
extern "C" {
    #[doc = " Creates a randomx_dataset structure and allocates memory for RandomX Dataset."]
    #[doc = ""]
    #[doc = " @param flags is the initialization flags. Only one flag is supported (can be set or not set):"]
    #[doc = "        RANDOMX_FLAG_LARGE_PAGES - allocate memory in large pages"]
    #[doc = ""]
    #[doc = " @return Pointer to an allocated randomx_dataset structure."]
    #[doc = "         NULL is returned if memory allocation fails."]
    pub fn randomx_alloc_dataset(flags: randomx_flags) -> *mut randomx_dataset;
}
extern "C" {
    #[doc = " Gets the number of items contained in the dataset."]
    #[doc = ""]
    #[doc = " @return the number of items contained in the dataset."]
    pub fn randomx_dataset_item_count() -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[doc = " Initializes dataset items."]
    #[doc = ""]
    #[doc = " Note: In order to use the Dataset, all items from 0 to (randomx_dataset_item_count() - 1) must be initialized."]
    #[doc = " This may be done by several calls to this function using non-overlapping item sequences."]
    #[doc = ""]
    #[doc = " @param dataset is a pointer to a previously allocated randomx_dataset structure. Must not be NULL."]
    #[doc = " @param cache is a pointer to a previously allocated and initialized randomx_cache structure. Must not be NULL."]
    #[doc = " @param startItem is the item number where intialization should start."]
    #[doc = " @param itemCount is the number of items that should be initialized."]
    pub fn randomx_init_dataset(
        dataset: *mut randomx_dataset,
        cache: *mut randomx_cache,
        startItem: ::std::os::raw::c_ulong,
        itemCount: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[doc = " Returns a pointer to the internal memory buffer of the dataset structure. The size"]
    #[doc = " of the internal memory buffer is randomx_dataset_item_count() * RANDOMX_DATASET_ITEM_SIZE."]
    #[doc = ""]
    #[doc = " @param dataset is a pointer to a previously allocated randomx_dataset structure. Must not be NULL."]
    #[doc = ""]
    #[doc = " @return Pointer to the internal memory buffer of the dataset structure."]
    pub fn randomx_get_dataset_memory(dataset: *mut randomx_dataset)
        -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Releases all memory occupied by the randomx_dataset structure."]
    #[doc = ""]
    #[doc = " @param dataset is a pointer to a previously allocated randomx_dataset structure."]
    pub fn randomx_release_dataset(dataset: *mut randomx_dataset);
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
    #[doc = " Reinitializes a virtual machine with a new Cache. This function should be called anytime"]
    #[doc = " the Cache is reinitialized with a new key. Does nothing if called with a Cache containing"]
    #[doc = " the same key value as already set."]
    #[doc = ""]
    #[doc = " @param machine is a pointer to a randomx_vm structure that was initialized"]
    #[doc = "        without RANDOMX_FLAG_FULL_MEM. Must not be NULL."]
    #[doc = " @param cache is a pointer to an initialized randomx_cache structure. Must not be NULL."]
    pub fn randomx_vm_set_cache(machine: *mut randomx_vm, cache: *mut randomx_cache);
}
extern "C" {
    #[doc = " Reinitializes a virtual machine with a new Dataset."]
    #[doc = ""]
    #[doc = " @param machine is a pointer to a randomx_vm structure that was initialized"]
    #[doc = "        with RANDOMX_FLAG_FULL_MEM. Must not be NULL."]
    #[doc = " @param dataset is a pointer to an initialized randomx_dataset structure. Must not be NULL."]
    pub fn randomx_vm_set_dataset(machine: *mut randomx_vm, dataset: *mut randomx_dataset);
}
extern "C" {
    #[doc = " Releases all memory occupied by the randomx_vm structure."]
    #[doc = ""]
    #[doc = " @param machine is a pointer to a previously created randomx_vm structure."]
    pub fn randomx_destroy_vm(machine: *mut randomx_vm);
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
extern "C" {
    #[doc = " Set of functions used to calculate multiple RandomX hashes more efficiently."]
    #[doc = " randomx_calculate_hash_first will begin a hash calculation."]
    #[doc = " randomx_calculate_hash_next  will output the hash value of the previous input"]
    #[doc = "                              and begin the calculation of the next hash."]
    #[doc = " randomx_calculate_hash_last  will output the hash value of the previous input."]
    #[doc = ""]
    #[doc = " WARNING: These functions may alter the floating point rounding mode of the calling thread."]
    #[doc = ""]
    #[doc = " @param machine is a pointer to a randomx_vm structure. Must not be NULL."]
    #[doc = " @param input is a pointer to memory to be hashed. Must not be NULL."]
    #[doc = " @param inputSize is the number of bytes to be hashed."]
    #[doc = " @param nextInput is a pointer to memory to be hashed for the next hash. Must not be NULL."]
    #[doc = " @param nextInputSize is the number of bytes to be hashed for the next hash."]
    #[doc = " @param output is a pointer to memory where the hash will be stored. Must not"]
    #[doc = "        be NULL and at least RANDOMX_HASH_SIZE bytes must be available for writing."]
    pub fn randomx_calculate_hash_first(
        machine: *mut randomx_vm,
        input: *const ::std::os::raw::c_void,
        inputSize: size_t,
    );
}
extern "C" {
    pub fn randomx_calculate_hash_next(
        machine: *mut randomx_vm,
        nextInput: *const ::std::os::raw::c_void,
        nextInputSize: size_t,
        output: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn randomx_calculate_hash_last(
        machine: *mut randomx_vm,
        output: *mut ::std::os::raw::c_void,
    );
}
