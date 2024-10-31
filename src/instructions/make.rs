use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};


pub fn make (
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    let [
        maker,
        escrow,
        _system_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    assert!(escrow.is_signer());

    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key()
    }

    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8; 104])
            = *(data.as_ptr() as *const [u8; 104])
    }

    Ok(())
}