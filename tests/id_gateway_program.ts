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
import { IdGateway } from "../target/types/id_gateway";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { IdRegistryProgram } from "./id_registry_program";
import { Common } from "./common";
type PK = web3.PublicKey;
type KP = web3.Keypair;
type con = web3.Connection;
export class IdGatewayProgram {
  private static instance: IdGatewayProgram;
  private _programId: web3.PublicKey | null = null;
  private _program: Program<IdGateway> | null = null;
  private _wallet: Wallet | null = null;
  private _admin: KP | null = null;
  private constructor() {}
  static getInstance(): IdGatewayProgram {
    if (!IdGatewayProgram.instance) {
      IdGatewayProgram.instance = new IdGatewayProgram();
    }
    return IdGatewayProgram.instance;
  }
  get programId(): PK {
    if (!this._program) {
      this.initializeProgram();
    }
    return this._programId!;
  }
  get program(): Program<IdGateway> {
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
      const program = workspace.IdGateway as Program<IdGateway>;
      const wallet = provider.wallet as Wallet;
      const admin = Common.getAdminKeypair();
      this._wallet = wallet;
      this._program = program;
      this._admin = admin;
      this._programId = program.programId;
    }
  }
  static program() {
    return IdGatewayProgram.getInstance().program;
  }
  static wallet() {
    return IdGatewayProgram.getInstance().wallet;
  }
  static async registerViaGateway(
    custodyAddress: PK,
    recoveryAddress: PK,
    registryProgram: PK
  ) {
    try {
      const { idCounter } =
        await this.program().account.idRegistryGateway.fetch(
          IdRegistryProgram.registry_gateway_pda
        );
      const wid = idCounter.add(new BN(1));
      const instance = IdGatewayProgram.getInstance();
      const keys = await instance.program.methods
        .register()
        .accountsStrict({
          custodyAccount: custodyAddress,
          payer: instance.wallet.publicKey,
          recoveryAccount: recoveryAddress,
          registryProgram,
          widAccount: IdRegistryProgram.wid_address(wid),
          instructionSysvar: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          registryGateway: IdRegistryProgram.registry_gateway_pda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([instance.wallet.payer])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return true;
    } catch (error) {
      if (!error.logs) {
        throw new Error("Unexpected error");
      }
      console.log(error.logs);
      return false;
    }
  }
}
