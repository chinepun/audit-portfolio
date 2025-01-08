import * as anchor from "@project-serum/anchor";
// import { createSyncNativeInstruction } from "@solana/spl-token";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import fs from "fs";

export const createKeypairFromFile = async (
    provider: anchor.AnchorProvider,
    filepath
  ) => {
    const path = require("path");
    const file = fs.readFileSync(path.resolve(__dirname, filepath));
    // let fileTxt = readFileSync("./_users/mint.json", { encoding: 'utf-8' });
    let kepairJson = JSON.parse(file);
    let buffers_8 = Uint8Array.from(kepairJson);
    let token_keypair = anchor.web3.Keypair.fromSecretKey(buffers_8);
  
    
    const latestBlockHash = await provider.connection.getLatestBlockhash();
      
    const airdropSignature = await provider.connection.requestAirdrop(
      token_keypair.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,    
    });
  
  
    // console.log("token keypair = ", token_keypair.publicKey.toString());
    return token_keypair;
  } 


  export const createKeypair = async (
    provider: anchor.AnchorProvider
  ) => {
      const keypair = new anchor.web3.Keypair();
      const latestBlockHash = await provider.connection.getLatestBlockhash();
      
      const airdropSignature = await provider.connection.requestAirdrop(
        keypair.publicKey,
        10 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: airdropSignature,    
      });
      return keypair;
  };