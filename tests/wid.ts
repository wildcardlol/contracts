import { web3, BN } from "@coral-xyz/anchor";
import { assert } from "chai";
import { IdRegistryProgram } from "./registry_program";
describe("wid", () => {
  let connection: web3.Connection;
  let idGateway: web3.Keypair;
  let custody: web3.Keypair;
  let custody1: web3.Keypair;
  let recovery: web3.Keypair;
  let recovery2: web3.Keypair;

  it("setup", async () => {
    connection = new web3.Connection("http://localhost:8899", "confirmed");
    idGateway = await IdRegistryProgram.createAndAirdropKeypair(connection);
    custody = await IdRegistryProgram.createAndAirdropKeypair(connection);
    custody1 = await IdRegistryProgram.createAndAirdropKeypair(connection);
    recovery = await IdRegistryProgram.createAndAirdropKeypair(connection);
    recovery2 = await IdRegistryProgram.createAndAirdropKeypair(connection);
  });
  it("initialize_gateway", async () => {
    await IdRegistryProgram.initialize_gateway(idGateway.publicKey);
    const {
      gatewayFrozen,
      idCounter,
      idGateway: idg,
      owner,
    } = await IdRegistryProgram.program().account.idRegistryGateway.fetch(
      IdRegistryProgram.registry_gateway_pda
    );
    assert(gatewayFrozen == false, "Invalid Initialization");
    assert(idCounter.toString() == "0", "Invalid Initialization");
    assert(
      idg.toBase58() == idGateway.publicKey.toBase58(),
      "Invalid Id Gateway"
    );
    assert(
      owner.toBase58() == IdRegistryProgram.admin.toBase58(),
      "Invalid Owner"
    );
  });
  it("Register", async () => {
    await IdRegistryProgram.register(
      idGateway,
      custody.publicKey,
      recovery.publicKey
    );
    const { idCounter } =
      await IdRegistryProgram.program().account.idRegistryGateway.fetch(
        IdRegistryProgram.registry_gateway_pda
      );
    assert(idCounter.toString() == "1", "Id counter did not increase");
    const {
      wcid,
      custody: c,
      recovery: r,
    } = await IdRegistryProgram.program().account.wcidAccount.fetch(
      IdRegistryProgram.wcid_address(idCounter)
    );
    assert(wcid.toString() == "1", "Wcid not set");
    assert(custody.publicKey.toBase58() == c.toBase58(), "Invalid custody");
    assert(recovery.publicKey.toBase58() == r.toBase58(), "Invalid recovery");
  });
  it("Transfer", async () => {
    const wcAddress = IdRegistryProgram.wcid_address(new BN(1));
    await IdRegistryProgram.transfer(wcAddress, custody, custody1.publicKey);
    const { custody: c } =
      await IdRegistryProgram.program().account.wcidAccount.fetch(wcAddress);
    assert(
      custody.publicKey.toBase58() != c.toBase58(),
      "Custody not transferred"
    );
    assert(
      custody1.publicKey.toBase58() == c.toBase58(),
      "Custody not transferred to expected account"
    );
  });
});
