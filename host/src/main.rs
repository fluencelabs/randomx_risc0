// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::sha::Digest;
use risc0_zkvm::Executor;
use risc0_zkvm::ExecutorEnv;
use risc0_zkvm::FileSegmentRef;

use host_guest_interface::Interface;
use tempfile::tempdir;

use std::time::Instant;

fn main() {
    println!("host: start");
    let input = Interface::new("aa some key".to_string(), 1);

    println!("host: before executor env");
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&input).unwrap())
        .session_limit(Some(usize::MAX))
        .build()
        .unwrap();

    println!("host: executor from elf");
    let segment_dir = tempdir().unwrap();
    let session_start = Instant::now();
    let mut executor = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();
    println!("host: executor created");
    let session = executor.run_with_callback(|segment| {
        Ok(Box::new(
                FileSegmentRef::new(
                    &segment,
                    &segment_dir.path(),
                    )?
                ))
    }).unwrap();
    let session_time = session_start.elapsed();
    println!("host: session created in {:?}", session_time);

    let prove_start = Instant::now();
    let receipt = session.prove().unwrap();
    let prove_time = prove_start.elapsed();
    println!("host: prove time is {:?}", prove_time);

    let verify_start = Instant::now();
    receipt.verify(<[u32; 8] as Into<Digest>>::into(METHOD_NAME_ID)).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
    let verify_time = verify_start.elapsed();
    println!("host: verify succeeded in {:?}", verify_time);
}
