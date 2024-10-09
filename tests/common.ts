import { web3 } from "@coral-xyz/anchor";
import path from "path";
import fs from "fs";
export type KP = web3.Keypair;
export type PK = web3.PublicKey;
export class Common {
  static getAdminKeypair(): KP {
    const adminJsonPath = path.join(__dirname, "..", "admin.json");
    const adminJsonContent = fs.readFileSync(adminJsonPath, "utf8");
    const adminKeypairData = JSON.parse(adminJsonContent);
    const secretKey = Uint8Array.from(adminKeypairData);
    return web3.Keypair.fromSecretKey(secretKey);
  }
  static async createAndAirdropKeypair(
    connection: web3.Connection,
    lamports: number = 1000000000
  ): Promise<web3.Keypair> {
    const keypair = new web3.Keypair();
    const airdropSignature = await connection.requestAirdrop(
      keypair.publicKey,
      lamports
    );
    await connection.confirmTransaction(airdropSignature, "confirmed");
    return keypair;
  }
  static async airdrop(
    connection: web3.Connection,
    address: PK,
    lamports: number = 1000000000
  ) {
    const airdropSignature = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(airdropSignature, "confirmed");
  }
}
