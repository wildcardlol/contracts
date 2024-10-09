import {
  AnchorProvider,
  Program,
  setProvider,
  Wallet,
  web3,
  workspace,
} from "@coral-xyz/anchor";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { SignerGateway } from "../target/types/signer_gateway";
import { Common, KP, PK, WidAccount } from "./common";
import { IdRegistryProgram } from "./id_registry_program";
import { SignerRegistryProgram } from "./signer_registry";
export class SignerGatewayProgram {
  private static instance: SignerGatewayProgram;
  private _programId: web3.PublicKey | null = null;
  private _program: Program<SignerGateway> | null = null;
  private _wallet: Wallet | null = null;
  private _admin: KP | null = null;
  private constructor() {}
  static getInstance(): SignerGatewayProgram {
    if (!SignerGatewayProgram.instance) {
      SignerGatewayProgram.instance = new SignerGatewayProgram();
    }
    return SignerGatewayProgram.instance;
  }
  get programId(): PK {
    if (!this._program) {
      this.initializeProgram();
    }
    return this._programId!;
  }
  get program(): Program<SignerGateway> {
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
      const program = workspace.SignerGateway as Program<SignerGateway>;
      const wallet = provider.wallet as Wallet;
      const admin = Common.getAdminKeypair();
      this._wallet = wallet;
      this._program = program;
      this._admin = admin;
      this._programId = program.programId;
    }
  }
  static async addSignerViaGateway(
    custody: KP,
    widAccount: WidAccount,
    keyType: number,
    keyValue: Buffer,
    flags: Array<boolean>,
    isAdmin: boolean
  ) {
    try {
      const instance = this.getInstance();
      const keys = await instance.program.methods
        .add({ keyType, value: keyValue }, flags, isAdmin)
        .accountsStrict({
          idRegistryProgram: IdRegistryProgram.getInstance().programId,
          keyGatewayState: SignerRegistryProgram.keyGatewayPda,
          keyRegistryProgram: SignerRegistryProgram.getInstance().programId,
          custody: custody.publicKey,
          payer: instance.wallet.publicKey,
          keyAccount: SignerRegistryProgram.keyAccountAddress(widAccount),
          widAccount: IdRegistryProgram.widAddress(widAccount.wid),
          instructionSysvar: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([instance.wallet.payer, custody])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      console.log(error);
      return false;
    }
  }
}
