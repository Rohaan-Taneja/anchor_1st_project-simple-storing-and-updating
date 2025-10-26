use anchor_lang::prelude::*;

// This is the unique public key of your deployed program.
declare_id!("6bYQk58SMWhwX6ch8Eac1u4V4LEH9muHD3X1hxFfLeBv");

// this macro tells that this is the main smart contract and this is the staritng pointof the contract
// all the function defined in it are instructions which can be called by the users
// converts it into raw solana type of contract
#[program]
pub mod calculator_rohan {
    use super::*;

    pub fn initalize_calculator(ctx : Context<Initalize_calculator>) -> Result<()> {
        let new_data_account = &mut ctx.accounts.new_aacount;
        new_data_account.value = 0;
        new_data_account.Owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn update_by_2(ctx : Context<update_data>)-> Result<()>{
        // taking mut reference of the data account owned by this contract 
        let data_account = &mut ctx.accounts.data_account;
        data_account.value = 2 ;

        Ok(())
    }
}

// serialization / deserialization , data_Account_owner details / 8 bytes identifier for the this struct
#[account]
pub struct accountData {
    value: u32,
    Owner: Pubkey,
}

#[derive(Accounts)]
pub struct Initalize_calculator<'info> {
    #[account(init , payer = owner , space = 8 + 4 + 32)]
    pub new_aacount: Account<'info, accountData>,
    #[account(mut)]
    // contract is saying that this account is mutable , measn we can update its data or lamports
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct update_data<'info> {
    #[account(mut)]
    pub signer : Signer<'info> ,
    #[account(mut)]
    pub data_account : Account<'info ,  accountData>
}
