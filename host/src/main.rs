// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::Executor;
use risc0_zkvm::ExecutorEnv;
use risc0_zkvm::sha::Digest;

use host_guest_interface::Interface;

use std::time::Instant;

fn main() {
    println!("host: start");
    let input = Interface::new("some key".to_string(), 0);

    println!("host: before executor env");
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&input).unwrap())
        .session_limit(usize::MAX)
        .build();

    println!("host: executor from elf");
    let prove_start = Instant::now();
    let mut executor = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();
    println!("host: executor created");
    let session = executor.run().unwrap();
    let prove_time = prove_start.elapsed();
    println!("host: session created in {}", prove_time);
    let receipt = session.prove().unwrap();

    let verify_start = Instant::now();
    receipt.verify(<[u32; 8] as Into<Digest>>::into(METHOD_NAME_ID)).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
    let verify_time = verify_start.elapsed();
    println!("host: verify succedded in {}", verify_time);
}
