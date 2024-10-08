import {
  AnchorProvider,
  Program,
  setProvider,
  web3,
  Wallet,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import fs from "fs";
import path from "path";
import { IdRegistry } from "../target/types/id_registry";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
type PK = web3.PublicKey;
type KP = web3.Keypair;
type con = web3.Connection;
export class IdRegistryProgram {
  private static instance: IdRegistryProgram;
  private _programId: web3.PublicKey | null = null;
  private _program: Program<IdRegistry> | null = null;
  private _wallet: Wallet | null = null;
  private _admin: KP | null = null;
  private constructor() {}
  static getInstance(): IdRegistryProgram {
    if (!IdRegistryProgram.instance) {
      IdRegistryProgram.instance = new IdRegistryProgram();
    }
    return IdRegistryProgram.instance;
  }
  get programId(): PK {
    if (!this._program) {
      this.initializeProgram();
    }
    return this._programId!;
  }
  get program(): Program<IdRegistry> {
    if (!this._program) {
      this.initializeProgram();
    }
    return this._program!;
  }
  get wallet(): Wallet {
    if (!this._program || !this._wallet) {
      this.initializeProgram();
    }
    return this._wallet;
  }
  get admin(): KP {
    if (!this._program || !this._admin) {
      this.initializeProgram();
    }
    return this._admin;
  }
  private initializeProgram(): void {
    if (!this._program) {
      let provider = AnchorProvider.env();
      setProvider(provider);
      const program = workspace.IdRegistry as Program<IdRegistry>;
      const wallet = provider.wallet as Wallet;
      const admin = IdRegistryProgram.getAdminKeypair();
      this._wallet = wallet;
      this._program = program;
      this._admin = admin;
      this._programId = program.programId;
    }
  }
  static program() {
    return IdRegistryProgram.getInstance().program;
  }
  static wallet() {
    return IdRegistryProgram.getInstance().wallet;
  }
  static get registry_gateway_pda(): web3.PublicKey {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registry_gateway")],
      IdRegistryProgram.getInstance().programId
    )[0];
  }
  static wid_address(wid: BN): web3.PublicKey {
    const widBuffer = wid.toArrayLike(Buffer, "le", 8);
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wid_account_seed"), widBuffer],
      IdRegistryProgram.getInstance().programId
    )[0];
  }
  static async initialize_gateway(gatewayProgram: web3.PublicKey) {
    try {
      const keys = await IdRegistryProgram.getInstance()
        .program.methods.initializeGateway()
        .accountsStrict({
          gatewayProgram,
          owner: IdRegistryProgram.getInstance().admin.publicKey,
          registryGateway: this.registry_gateway_pda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([this.wallet().payer, IdRegistryProgram.getInstance().admin])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return keys.pubkeys;
    } catch (error) {
      console.log(error.logs);
    }
  }
  static async register(custody: PK, recovery: PK) {
    try {
      const { idCounter } =
        await this.program().account.idRegistryGateway.fetch(
          this.registry_gateway_pda
        );
      const wid = idCounter.add(new BN(1));
      const keys = await IdRegistryProgram.getInstance()
        .program.methods.register()
        .accounts({
          registryGateway: this.registry_gateway_pda,
          custodyAccount: custody,
          recoveryAccount: recovery,
          payer: this.wallet().publicKey,
          widAccount: this.wid_address(wid),
        })
        .signers([this.wallet().payer])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return keys.pubkeys;
    } catch (error) {
      console.log(error.logs);
    }
  }
  static async transfer(wcAddress: PK, custody: KP, newCustody: PK) {
    try {
      const keys = await IdRegistryProgram.getInstance()
        .program.methods.transfer()
        .accounts({
          newCustody,
          signer: custody.publicKey,
          widAccount: wcAddress,
        })
        .signers([custody])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return keys.pubkeys;
    } catch (error) {
      console.log(error.logs);
    }
  }
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
