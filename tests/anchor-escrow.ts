import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorEscrow } from '../target/types/anchor_escrow';
import { PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { assert } from "chai";

describe('anchor-escrow', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  let mintA = null;
  let depositTokenAccountA = null;
  let vault_account_pda = null;
  let vault_account_bump = null;
  let vault_authority_pda = null;

  const depositAmount = 500;

  const merkleTreeAccount = anchor.web3.Keypair.generate();
  const anchorMetadataAccount = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const depositorMainAccount = anchor.web3.Keypair.generate();

  it("Initialize program state", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 10000000000),
      "confirmed"
    );

    // Fund Main Accounts
    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: depositorMainAccount.publicKey,
            lamports: 1000000000,
          }),
        );
        return tx;
      })(),
      [payer]
    );

    mintA = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    depositTokenAccountA = await mintA.createAccount(depositorMainAccount.publicKey);
    await mintA.mintTo(
      depositTokenAccountA,
      mintAuthority.publicKey,
      [mintAuthority],
      depositAmount
    );

    let _depositTokenAccountA = await mintA.getAccountInfo(depositTokenAccountA);

    assert.ok(_depositTokenAccountA.amount.toNumber() == depositAmount);
  });

  it("Initialize merkle tree", async () => {
    const [_vault_account_pda, _vault_account_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("token-seed"))],
      program.programId
    );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("escrow"))],
      program.programId
    );
    vault_authority_pda = _vault_authority_pda;

    await program.rpc.initialize(
      vault_account_bump,
      new anchor.BN(depositAmount),
      30,
      {
        accounts: {
          initializer: depositorMainAccount.publicKey,
          vaultAccount: vault_account_pda,
          mint: mintA.publicKey,
          depositTokenAccount: depositTokenAccountA,
          anchorMetadata: anchorMetadataAccount.publicKey,
          merkleTreeAccount: merkleTreeAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [
          await program.account.merkleTreeAccount.createInstruction(merkleTreeAccount),
        ],
        signers: [merkleTreeAccount, anchorMetadataAccount, depositorMainAccount],
      }
    );

    let _vault = await mintA.getAccountInfo(vault_account_pda);

    let _merkleTreeAccount = await program.account.merkleTreeAccount.fetch(
      merkleTreeAccount.publicKey
    );

    let _anchorMetadataAccount = await program.account.anchorMetadata.fetch(
      anchorMetadataAccount.publicKey
    );

    // Check that the new owner is the PDA.
    assert.ok(_vault.owner.equals(vault_authority_pda));

    // Check that the values in the escrow account match what we expect.
    assert.ok(_merkleTreeAccount.initializerKey.equals(depositorMainAccount.publicKey));
    assert.equal(_merkleTreeAccount.levels, 30);
    assert.equal(_merkleTreeAccount.currentRootIndex, 0);
    assert.equal(_merkleTreeAccount.nextIndex, 0);

    assert.ok(_anchorMetadataAccount.initializerKey.equals(depositorMainAccount.publicKey));
    assert.ok(_anchorMetadataAccount.depositAmount.toNumber() == depositAmount);
    assert.ok(_anchorMetadataAccount.depositTokenAccount.equals(depositTokenAccountA));
  });

  it("DepositInto merkle tree", async () => {
    const [_vault_account_pda, _vault_account_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("token-seed"))],
      program.programId
    );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("escrow"))],
      program.programId
    );
    vault_authority_pda = _vault_authority_pda;

    await program.rpc.deposit(
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      {
        accounts: {
          depositor: depositorMainAccount.publicKey,
          vaultAccount: vault_account_pda,
          depositTokenAccount: depositTokenAccountA,
          anchorMetadata: anchorMetadataAccount.publicKey,
          merkleTreeAccount: merkleTreeAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [depositorMainAccount],
      }
    );

    let _vault = await mintA.getAccountInfo(vault_account_pda);

    let _merkleTreeAccount = await program.account.merkleTreeAccount.fetch(
      merkleTreeAccount.publicKey
    );

    let _anchorMetadataAccount = await program.account.anchorMetadata.fetch(
      anchorMetadataAccount.publicKey
    );

    assert.equal(_anchorMetadataAccount.depositCount.toString(), '1');

  });
});
