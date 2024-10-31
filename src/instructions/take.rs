use pinocchio::{
    account_info::AccountInfo, 
    instruction::{Seed, Signer}, 
    program_error::ProgramError, 
    ProgramResult
};

use pinocchio_token::{
    instructions::{CloseAccount, Transfer}, 
    state::TokenAccount
};

use crate::state::Escrow;

pub fn take(
    accounts: &[AccountInfo], 
    bump: [u8; 1]
) -> ProgramResult {

    let [
        taker,
        taker_ta_a,
        taker_ta_b,
        maker_ta_b,
        escrow,
        vault,
        authority,
        _token_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    let escrow_account = Escrow::from_account_info(escrow);

    assert_eq!(maker_ta_b.key(), &escrow_account.maker_ta_b());

    assert_eq!(
        &TokenAccount::from_account_info(vault).mint(),
        &escrow_account.mint_a()
    );

    Transfer {
        from: taker_ta_b,
        to: maker_ta_b,
        authority: taker,
        amount: escrow_account.amount_b(),
    }.invoke()?;

    let seeds = [
        Seed::from(escrow.key().as_ref()), 
        Seed::from(&bump)
    ];
    let signer = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: taker_ta_a,
        authority,
        amount: TokenAccount::from_account_info(vault).amount(),
    }.invoke_signed(&signer.clone())?;

    CloseAccount {
        account: vault,
        destination: taker,
        authority,
    }.invoke_signed(&signer.clone())?;

    unsafe {
        *taker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}