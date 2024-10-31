use pinocchio::{
    account_info::AccountInfo, 
    instruction::{Seed, Signer}, 
    program_error::ProgramError, 
    pubkey::Pubkey, 
    ProgramResult,
};
use pinocchio_token::{
    instructions::{CloseAccount, Transfer}, 
    state::TokenAccount,
};

use crate::state::Escrow;


pub fn refund(
    accounts: &[AccountInfo],
    bump: [u8; 1]
) -> ProgramResult {
    let [
        maker,
        maker_ta_a,
        escrow,
        vault,
        authority,
        _token_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    assert!(maker.is_signer());

    let escrow_account = Escrow::from_account_info(escrow);
    assert_eq!(&escrow_account.maker(), maker.key());

    let seeds = [Seed::from(escrow.key().as_ref()), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    let amount = TokenAccount::from_account_info_unchecked(vault).amount();

    Transfer {
        from: vault,
        to: maker_ta_a,
        authority,
        amount
    }.invoke_signed(&signer)?;

    CloseAccount {
        account: vault,
        destination: maker,
        authority,
    }.invoke_signed(&signer)?;

    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        escrow.assign(&Pubkey::default());

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}