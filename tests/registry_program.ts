import {
  AnchorProvider,
  Program,
  setProvider,
  web3,
  Wallet,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import { IdRegistry } from "../target/types/id_registry";
import IDL from "../target/idl/id_registry.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
type PK = web3.PublicKey;
type KP = web3.Keypair;
type con = web3.Connection;
export class IdRegistryProgram {
  private static instance: IdRegistryProgram;
  static readonly programId: web3.PublicKey = new web3.PublicKey(
    "ECFPDX4ux9wpsrt7KAhGmv3LkXwrgHabLHWUpdKtfBVV"
  );
  static readonly admin: web3.PublicKey = new web3.PublicKey(
    "BEs6Lh6NbDVtxt3FPKVkLN9fz22Byk6EvEp4GRUW7mqm"
  );
  private _program: Program<IdRegistry> | null = null;
  private _wallet: Wallet | null = null;
  private constructor() {}
  static getInstance(): IdRegistryProgram {
    if (!IdRegistryProgram.instance) {
      IdRegistryProgram.instance = new IdRegistryProgram();
    }
    return IdRegistryProgram.instance;
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
  private initializeProgram(): void {
    if (!this._program) {
      let provider = AnchorProvider.env();
      setProvider(provider);
      const program = workspace.IdRegistry as Program<IdRegistry>;
      const wallet = provider.wallet as Wallet;
      this._wallet = wallet;
      this._program = program;
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
      IdRegistryProgram.programId
    )[0];
  }
  static wcid_address(wcid: BN): web3.PublicKey {
    const wcidBuffer = wcid.toArrayLike(Buffer, "le", 8);
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wcid_account_seed"), wcidBuffer],
      IdRegistryProgram.programId
    )[0];
  }
  static async initialize_gateway(idGateway: web3.PublicKey) {
    try {
      const keys = await IdRegistryProgram.getInstance()
        .program.methods.initializeGateway()
        .accountsStrict({
          idGateway: idGateway,
          owner: this.admin,
          registryGateway: this.registry_gateway_pda,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([this.wallet().payer])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return keys.pubkeys;
    } catch (error) {
      console.log(error.logs);
    }
  }
  static async register(
    idGatewayKeypair: web3.Keypair,
    custody: PK,
    recovery: PK
  ) {
    try {
      const { idCounter } =
        await this.program().account.idRegistryGateway.fetch(
          this.registry_gateway_pda
        );
      const wcid = idCounter.add(new BN(1));
      const keys = await IdRegistryProgram.getInstance()
        .program.methods.register()
        .accounts({
          registryGateway: this.registry_gateway_pda,
          custodyAccount: custody,
          recoveryAccount: recovery,
          signer: idGatewayKeypair.publicKey,
          wcidAccount: this.wcid_address(wcid),
        })
        .signers([this.wallet().payer, idGatewayKeypair])
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
          wcidAccount: wcAddress,
        })
        .signers([custody])
        .rpcAndKeys({ commitment: "confirmed" });
      console.log(keys.signature);
      return keys.pubkeys;
    } catch (error) {
      console.log(error.logs);
    }
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
}
