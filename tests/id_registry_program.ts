import {
  AnchorProvider,
  Program,
  setProvider,
  web3,
  Wallet,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import { Common, KP, PK } from "./common";
import { IdRegistry } from "../target/types/id_registry";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { IdGatewayProgram } from "./id_gateway_program";
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
      const admin = Common.getAdminKeypair();
      this._wallet = wallet;
      this._program = program;
      this._admin = admin;
      this._programId = program.programId;
    }
  }
  static get registryGatewayPda(): web3.PublicKey {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registry_gateway")],
      this.getInstance().programId
    )[0];
  }
  static async fetchIdRegistryGatewayAccount() {
    return await this.getInstance().program.account.idRegistryGateway.fetch(
      this.registryGatewayPda
    );
  }
  static widAddress(wid: BN): web3.PublicKey {
    const widBuffer = wid.toArrayLike(Buffer, "le", 8);
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wid_account_seed"), widBuffer],
      this.getInstance().programId
    )[0];
  }
  static async initializeGateway(idGatewayProgram: PK): Promise<boolean> {
    try {
      const keys = await this.getInstance()
        .program.methods.initializeGateway()
        .accountsStrict({
          gatewayProgram: idGatewayProgram,
          owner: Common.admin.publicKey,
          registryGateway: this.registryGatewayPda,
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

  // This should fail everytime invoked, only gateway should be able to cpi to this
  static async registerRaw(custody: PK, recovery: PK): Promise<boolean> {
    try {
      const { idCounter } =
        await this.getInstance().program.account.idRegistryGateway.fetch(
          this.registryGatewayPda
        );
      const wid = idCounter.add(new BN(1));
      const keys = await this.getInstance()
        .program.methods.register()
        .accountsStrict({
          registryGateway: this.registryGatewayPda,
          custodyAccount: custody,
          recoveryAccount: recovery,
          payer: Common.wallet.publicKey,
          widAccount: this.widAddress(wid),
          instructionSysvar: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([Common.wallet.payer])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      console.log(error.errorLogs);
      return false;
    }
  }
  static async transfer(widAddress: PK, custody: KP, newCustody: PK) {
    try {
      const keys = await this.getInstance()
        .program.methods.transfer()
        .accounts({
          newCustody,
          signer: custody.publicKey,
          widAccount: widAddress,
        })
        .signers([custody])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      console.log(error.errorLogs);
      return false;
    }
  }
  static async fetchWidAccount(wid: BN) {
    return await this.getInstance().program.account.widAccount.fetch(
      this.widAddress(wid)
    );
  }
}
