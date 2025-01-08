use trident_client::fuzzing::*;


mod instructions;
use instructions::{Initialize, DeployToken, InitTransfer, InitTransferSol, FinalizeTransfer, FinalizeTransferSol, LogMetadata};

mod fuzz_instructions;
mod constants;
use fuzz_instructions::FuzzInstruction;

struct InstructionsSequence;

use bridge_token_factory::entry as entry_bridge_token_factory;
use bridge_token_factory::ID as PROGRAM_ID_BRIDGE_TOKEN_FACTORY;
const PROGRAM_NAME_BRIDGE_TOKEN_FACTORY: &str = "bridge_token_factory";
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// use fuzz_instructions::{InitializeFn, UpdateFn, WithdrawFn};
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(Initialize, LogMetadata);
    middle_sequence!(InitTransfer, InitTransferSol, DeployToken);
    post_sequence!(FinalizeTransfer, FinalizeTransferSol);
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {

    let fuzzing_program_bridge_token_factory = FuzzingProgram::new(
        PROGRAM_NAME_BRIDGE_TOKEN_FACTORY,
        &PROGRAM_ID_BRIDGE_TOKEN_FACTORY,
        processor!(convert_entry!(entry_bridge_token_factory)),
    );

    let mut client = ProgramTestClientBlocking::new(&[fuzzing_program_bridge_token_factory], config).unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
