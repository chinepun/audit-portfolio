import { assert } from "chai";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { min } from "bn.js";
import { BridgeTokenFactory } from "../target/types/bridge_token_factory";
// import BridgeTokenFactoryJSON from "../target/idl/bridge_token_factory.json";
import { rpc } from "@project-serum/anchor/dist/cjs/utils";
import fs from "fs";
import { Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, createAssociatedTokenAccountInstruction, createInitializeAccount3Instruction, createInitializeAccountInstruction, createMint, getAccount, getAccountLenForMint, getMint, getOrCreateAssociatedTokenAccount, mintTo, mintToChecked, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { it } from "mocha";
import {
    deriveAddress,
    getPostMessageCpiAccounts,
  } from "@certusone/wormhole-sdk/lib/cjs/solana";
import { createKeypair, createKeypairFromFile } from "./utils";

describe("admin tests", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider =  anchor.getProvider() as anchor.AnchorProvider;
  
    const omni = new anchor.web3.PublicKey("omn5mh1bHBt5U6JotBSzj1tQpUizwfcLAQVPfJiCnpZ");
    const wormhole = new anchor.web3.PublicKey("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth")

    const program = anchor.workspace.BridgeTokenFactory as Program<BridgeTokenFactory>;

    it ("Admin can initialize",async () => {

        const payer = await createKeypair(provider);
        const admin = await Keypair.generate();
        const deployed_program = await createKeypairFromFile(provider, "../deploy/omni.json");

        const config = await PublicKey.findProgramAddressSync(
            [Buffer.from("config")],
            program.programId
        )[0];

        const authority = await PublicKey.findProgramAddressSync(
            [Buffer.from("authority")],
            program.programId
        )[0];

        const sol_vault = await PublicKey.findProgramAddressSync(
            [Buffer.from("sol_vault")],
            program.programId
        )[0];
        const wormholeCpi = getPostMessageCpiAccounts(
            program.programId,
            wormhole,
            payer.publicKey,
            deriveAddress([Buffer.from("custom_message")], program.programId)
          );
         
        console.log("wormhole bridge ", wormholeCpi.wormholeBridge.toString());
        
        const wormhole_bridge = await PublicKey.findProgramAddressSync(
            [Buffer.from("Bridge")],
            wormhole
        )[0]; 
        // console.log("simulated Bridge = ", wormhole_bridge.toString());

        const wormhole_fee_collector = await PublicKey.findProgramAddressSync(
            [Buffer.from("fee_collector")],
            wormhole
        )[0];
        console.log("Fee collector = ", wormholeCpi.wormholeFeeCollector.toString());

        const wormhole_sequence = await PublicKey.findProgramAddressSync(
            [Buffer.from("Sequence"), config.toBuffer()],
            wormhole
        )[0];
        console.log("Sequence = ", wormhole_sequence.toString());
        const wormhole_message = await Keypair.generate();


        
        let before_amount_sol_vault = await provider.connection.getAccountInfo(sol_vault);

        console.log("before_amount_sol_vault", before_amount_sol_vault?.lamports);
        try {
            await program.methods.initialize(
                admin.publicKey,
                Buffer.alloc(64, "some string to occupy ")
            )
            .accounts({
                config: config,
                authority,
                solVault: sol_vault,
                wormholeBridge: wormhole_bridge,
                wormholeFeeCollector: wormhole_fee_collector,
                wormholeSequence: wormhole_sequence,
                wormholeMessage: wormhole_message.publicKey,
                payer: payer.publicKey,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                // wormholeBridge: wormhole_bridge,
                // wormholeFeeCollector: wormhole_fee_collector,
                // wormholeSequence: wormhole_sequence,
                // wormholeMessage: wormhole_message.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                wormholeProgram: wormhole,
                program: deployed_program.publicKey,
            }).signers([wormhole_message, payer, ]).rpc();
    
        } catch (error) {
            console.log("Err => ", error);
        }


        const token = "WUSD Near"
        const mint = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("wrapped_mint"), 
                Buffer.from(anchor.utils.bytes.utf8.encode(token)),
            ],
            program.programId
        )[0];
        const ETH_KEYPAIR = await createKeypair(provider);
        const mint_authority = await createKeypair(provider);
        const ETH_mint = await createMint(
            provider.connection,
            ETH_KEYPAIR,
            mint_authority.publicKey,
            mint_authority.publicKey,
            0,
          ); 
        console.log("mint = ", mint.toString());
        const metaplex_program = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        const metadata = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"), 
                new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
                mint.toBuffer(),
            ],
            metaplex_program
        )[0];
        try {
           
            let data =                
            {
                payload:  {
                    token: token,
                    name: "Test Token",
                    symbol: "TST",
                    decimals: 6
                },
                signature: Buffer.alloc(65, "some string to occupy "),
            }
            let new_message = await Keypair.generate();
            const deploy_message = await Keypair.generate();
            await program.methods.deployToken(
                data
            ).accounts({
                authority: authority,
                mint: mint,
                metadata: metadata,
                wormhole: {
                    config: config,
                    bridge: wormhole_bridge,
                    feeCollector: wormhole_fee_collector,
                    sequence: wormhole_sequence,
                    message: new_message.publicKey,
                    payer: payer.publicKey,
                    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                    wormholeProgram: wormhole,
                    systemProgram: SystemProgram.programId,
                }
            }).signers([new_message, payer]).rpc();
        } catch (error) {
            console.log("Err => ", error);
        }
console.log("Deploy Token success")
        let vault = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"), 
                mint.toBuffer(),
            ],
            program.programId
        )[0];
        let ETH_vault = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"), 
                ETH_mint.toBuffer(),
            ],
            program.programId
        )[0];
        const ETH_metadata = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"), 
                metaplex_program.toBuffer(),
                ETH_mint.toBuffer(),
            ],
            metaplex_program
        )[0];

    console.log("Minted to Vault")
    // mint to from
    let payer_ETH_Account = await getOrCreateAssociatedTokenAccount(provider.connection, payer, ETH_mint, payer.publicKey);

    let from = await mintTo(
        provider.connection,
        ETH_KEYPAIR,
        ETH_mint,
        payer_ETH_Account.address,
        mint_authority,
        1000000000,
    );

console.log("Transferinh")
// const special_mint_airdrop = await mintToChecked(
//     provider.connection,
//     mint_authority,
//     special_mint,
//     creator_special_mint_account.address,
//     mint_authority,
//     10 * WSOL_AMOUNT,
//     MINT_DECIMALS
//   ); 

    // try {
    //     let new_message = await Keypair.generate();
    //     await program.methods.logMetadata().accounts({
    //         authority: authority,
    //         mint: ETH_mint,
    //         metadata: ETH_metadata,
    //         vault: ETH_vault,
    //         wormhole: {
    //             config: config,
    //             bridge: wormhole_bridge,
    //             feeCollector: wormhole_fee_collector,
    //             sequence: wormhole_sequence,
    //             message: new_message.publicKey,
    //             payer: payer.publicKey,
    //             clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
    //             rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    //             wormholeProgram: wormhole,
    //             systemProgram: SystemProgram.programId,
    //         },
    //         systemProgram: anchor.web3.SystemProgram.programId,
    //         tokenProgram: TOKEN_PROGRAM_ID,
    //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //     }).signers([new_message, payer]).rpc();

    //     new_message = await Keypair.generate();
    //     await program.methods.logMetadata().accounts({
    //         authority: authority,
    //         mint: mint,
    //         metadata: metadata,
    //         vault: vault,
    //         wormhole: {
    //             config: config,
    //             bridge: wormhole_bridge,
    //             feeCollector: wormhole_fee_collector,
    //             sequence: wormhole_sequence,
    //             message: new_message.publicKey,
    //             payer: payer.publicKey,
    //             clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
    //             rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    //             wormholeProgram: wormhole,
    //             systemProgram: SystemProgram.programId,
    //         },
    //         systemProgram: anchor.web3.SystemProgram.programId,
    //         tokenProgram: TOKEN_PROGRAM_ID,
    //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //     }).signers([new_message, payer]).rpc();

    // } catch (error) {
    //     console.log("Log MEtadata Err => ", error);
    // }
console.log("metadata looging successfil;", )
        // try {
                       
        //     let payload =  {
        //         amount: new anchor.BN(5000),
        //         recipient: "Some ",
        //         fee: new anchor.BN(5000),
        //         native_fee: new anchor.BN(5000),
        //     }
        //     let new_message = await Keypair.generate();
        //     await program.methods.initTransfer({
        //         amount: payload.amount,
        //         recipient: payload.recipient,
        //         fee: payload.fee,
        //         nativeFee: payload.native_fee
        //     }).accounts({
        //         authority,
        //         mint: ETH_mint,
        //         // metadata,
        //         vault: ETH_vault,
        //         from: payer_ETH_Account.address,
        //         user: payer.publicKey,
        //         wormhole: {
        //             config: config,
        //             bridge: wormhole_bridge,
        //             feeCollector: wormhole_fee_collector,
        //             sequence: wormhole_sequence,
        //             message: new_message.publicKey,
        //             payer: payer.publicKey,
        //             clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        //             rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        //             wormholeProgram: wormhole,
        //             systemProgram: SystemProgram.programId,
        //         },
        //         systemProgram: anchor.web3.SystemProgram.programId,
        //         tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        //         tokenMetadataProgram: metaplex_program
        //     }).signers([new_message, payer]).rpc();
        // } catch (error) {
        //     console.log("Error ", error)
        // }
        console.log("Init transfer passed", );

        try {
            let num = "1"
            let value = new anchor.BN("18446744073709551615");
            let payload_transfer =  {
                amount: value,
                recipient: "Some ",
                fee: new anchor.BN(0),
                nativeFee: new anchor.BN(2),
            }
            let new_message = await Keypair.generate();

            await program.methods.initTransferSol({
                amount: payload_transfer.amount,
                recipient: payload_transfer.recipient,
                fee: payload_transfer.fee,
                nativeFee: payload_transfer.nativeFee,
            }).accounts({
                authority,
                sol_vault,
                user: payer.publicKey,
                wormhole: {
                    config: config,
                    bridge: wormhole_bridge,
                    feeCollector: wormhole_fee_collector,
                    sequence: wormhole_sequence,
                    message: new_message.publicKey,
                    payer: payer.publicKey,
                    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                    wormholeProgram: wormhole,
                    systemProgram: SystemProgram.programId,
                },
            }).signers([new_message, payer]).rpc();
        } catch (error) {
            console.log("err ", error)
        }



try {
    
                       
    let payload =  {
        amount: new anchor.BN(1),
        recipient: "Some ",
        fee: new anchor.BN(5000),
        native_fee: new anchor.BN(1),
    }
    let new_message = await Keypair.generate();
    console.log("user = ", authority.toString());
    const keys = [
        { pubkey: authority, isSigner: false, isWritable: false },
        { pubkey: mint, isSigner: false, isWritable: true },
        { pubkey: payer_ETH_Account.address, isSigner: false, isWritable: true },
        { pubkey: ETH_vault, isSigner: false, isWritable: true },
        { pubkey: sol_vault, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: config, isSigner: false, isWritable: false },
        { pubkey: wormhole_bridge, isSigner: false, isWritable: true },
        { pubkey: wormhole_fee_collector, isSigner: false, isWritable: true },
        { pubkey: wormhole_sequence, isSigner: false, isWritable: true },
        { pubkey: new_message.publicKey, isSigner: true, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: anchor.web3.SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: anchor.web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: wormhole, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ];
    // await program.methods.initTransfer({
    //     amount: payload.amount,
    //     recipient: payload.recipient,
    //     fee: payload.fee,
    //     nativeFee: payload.native_fee
    // }).accounts({
    //     authority,
    //     mint: ETH_mint,
    //     metadata: ETH_metadata,
    //     vault: ETH_vault,
    //     from: payer_ETH_Account.address,
    //     user: payer.publicKey,
    //     wormhole: {
    //         config: config,
    //         bridge: wormhole_bridge,
    //         feeCollector: wormhole_fee_collector,
    //         sequence: wormhole_sequence,
    //         message: new_message.publicKey,
    //         payer: payer.publicKey,
    //         clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
    //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    //         wormholeProgram: wormhole,
    //         systemProgram: SystemProgram.programId,
    //     },
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //     tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    //     tokenMetadataProgram: metaplex_program
    // }).signers([new_message, payer]).rpc();
} catch (error) {
    console.log("Error ", error)
}
    })
  });