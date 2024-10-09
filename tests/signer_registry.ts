import {
  AnchorProvider,
  Program,
  setProvider,
  web3,
  Wallet,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import { Common, KP, PK, WidAccount } from "./common";
import { SignerRegistry } from "../target/types/signer_registry";
import { IdRegistryProgram } from "./id_registry_program";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
export class SignerRegistryProgram {
  private static instance: SignerRegistryProgram;
  private _programId: web3.PublicKey | null = null;
  private _program: Program<SignerRegistry> | null = null;
  private _wallet: Wallet | null = null;
  private _admin: KP | null = null;
  private constructor() {}
  static getInstance(): SignerRegistryProgram {
    if (!SignerRegistryProgram.instance) {
      SignerRegistryProgram.instance = new SignerRegistryProgram();
    }
    return SignerRegistryProgram.instance;
  }
  get programId(): PK {
    if (!this._program) {
      this.initializeProgram();
    }
    return this._programId!;
  }
  get program(): Program<SignerRegistry> {
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
      const program = workspace.SignerRegistry as Program<SignerRegistry>;
      const wallet = provider.wallet as Wallet;
      const admin = Common.getAdminKeypair();
      this._wallet = wallet;
      this._program = program;
      this._admin = admin;
      this._programId = program.programId;
    }
  }
  static get keyGatewayPda() {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("key_gateway")],
      this.getInstance().programId
    )[0];
  }
  static keyAccountAddress(widAccount: WidAccount): web3.PublicKey {
    const widBuffer = widAccount.wid.toArrayLike(Buffer, "le", 8);
    const keyCounterBuffer = Buffer.alloc(2);
    keyCounterBuffer.writeUInt16LE(widAccount.keyCounter + 1);
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("key_account"), widBuffer, keyCounterBuffer],
      this.getInstance().programId
    )[0];
  }
  static async initializeKeyGateway(
    keyGatewayProgram: PK,
    maxKeysPerId: number,
    maxFlags: number
  ): Promise<boolean> {
    try {
      const keys = await this.getInstance()
        .program.methods.initializeGateway(maxKeysPerId, maxFlags)
        .accountsStrict({
          owner: Common.admin.publicKey,
          idRegistryProgram: IdRegistryProgram.getInstance().programId,
          keyGatewayProgram,
          keyGatewayState: this.keyGatewayPda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([Common.wallet.payer, Common.admin])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      console.log(error.errorLogs);
      return false;
    }
  }
  // This should fail everytime invoked, only signer gateway should be able to cpi to this
  static async addSignerRaw(
    custody: KP,
    widAccount: WidAccount,
    keyType: number,
    keyValue: Buffer,
    flags: Array<boolean>,
    isAdmin: boolean
  ): Promise<boolean> {
    try {
      const keys = await this.getInstance()
        .program.methods.add({ keyType, value: keyValue }, flags, isAdmin)
        .accountsStrict({
          keyAccount: this.keyAccountAddress(widAccount),
          keyGatewayState: this.keyGatewayPda,
          idRegistryProgram: IdRegistryProgram.getInstance().programId,
          custody: custody.publicKey,
          payer: Common.wallet.publicKey,
          widAccount: IdRegistryProgram.widAddress(widAccount.wid),
          instructionSysvar: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([Common.wallet.payer, custody])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      console.log(error.errorLogs);
      return false;
    }
  }
  static async fetchKeyAccount(widAccount: WidAccount) {
    return await this.getInstance().program.account.keyAccount.fetch(
      this.keyAccountAddress(widAccount)
    );
  }
}
