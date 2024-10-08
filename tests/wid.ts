import { web3, BN } from "@coral-xyz/anchor";
import { assert } from "chai";
import { IdRegistryProgram } from "./id_registry_program";

describe("WID (Wild Card ID) Registry", () => {
  let connection: web3.Connection;
  let idGateway: web3.Keypair;
  let custody: web3.Keypair;
  let custody1: web3.Keypair;
  let recovery: web3.Keypair;
  let recovery2: web3.Keypair;
  let admin: web3.Keypair;
  before(async () => {
    console.log("Setting up test environment...");
    connection = new web3.Connection("http://localhost:8899", "confirmed");
    admin = IdRegistryProgram.getAdminKeypair();
    idGateway = web3.Keypair.generate();
    custody = await IdRegistryProgram.createAndAirdropKeypair(connection);
    custody1 = await IdRegistryProgram.createAndAirdropKeypair(connection);
    recovery = await IdRegistryProgram.createAndAirdropKeypair(connection);
    recovery2 = await IdRegistryProgram.createAndAirdropKeypair(connection);
    await IdRegistryProgram.airdrop(connection, admin.publicKey);
    console.log("Admin: ", admin.publicKey);
    console.log("Test environment set up successfully.");
  });

  it("should initialize the gateway correctly", async () => {
    console.log("Initializing gateway...");
    await IdRegistryProgram.initialize_gateway(idGateway.publicKey);

    const gatewayData =
      await IdRegistryProgram.program().account.idRegistryGateway.fetch(
        IdRegistryProgram.registry_gateway_pda
      );

    assertWithLog(
      gatewayData.idGatewayFrozen === false,
      "Gateway should not be frozen initially"
    );
    assertWithLog(
      gatewayData.idCounter.toString() === "0",
      "ID counter should start at 0"
    );
    assertWithLog(
      gatewayData.idGatewayProgram.toBase58() ===
        idGateway.publicKey.toBase58(),
      "ID Gateway should match the provided public key"
    );
    assertWithLog(
      gatewayData.owner.toBase58() === admin.publicKey.toBase58(),
      "Owner should match the admin public key"
    );

    printAccountData("ID Registry Gateway", gatewayData);
    console.log("Gateway initialized successfully.");
  });

  it("should register a new WID correctly", async () => {
    console.log("Registering new WID...");
    await IdRegistryProgram.register(custody.publicKey, recovery.publicKey);

    const gatewayData =
      await IdRegistryProgram.program().account.idRegistryGateway.fetch(
        IdRegistryProgram.registry_gateway_pda
      );
    assertWithLog(
      gatewayData.idCounter.toString() === "1",
      "ID counter should increment to 1"
    );

    const widData = await IdRegistryProgram.program().account.widAccount.fetch(
      IdRegistryProgram.wid_address(gatewayData.idCounter)
    );

    assertWithLog(widData.wid.toString() === "1", "WID should be set to 1");
    assertWithLog(
      widData.custody.toBase58() === custody.publicKey.toBase58(),
      "Custody public key should match the registered one"
    );
    assertWithLog(
      widData.recovery.toBase58() === recovery.publicKey.toBase58(),
      "Recovery public key should match the registered one"
    );

    printAccountData("ID Registry Gateway", gatewayData);
    printAccountData("WID Account", widData);
    console.log("WID registered successfully with ID:", widData.wid.toString());
  });

  it("should transfer custody correctly", async () => {
    console.log("Transferring custody...");
    const wcAddress = IdRegistryProgram.wid_address(new BN(1));
    await IdRegistryProgram.transfer(wcAddress, custody, custody1.publicKey);

    const widData = await IdRegistryProgram.program().account.widAccount.fetch(
      wcAddress
    );

    assertWithLog(
      widData.custody.toBase58() !== custody.publicKey.toBase58(),
      "Custody should have changed from the original"
    );
    assertWithLog(
      widData.custody.toBase58() === custody1.publicKey.toBase58(),
      "New custody should match the transferred account"
    );

    printAccountData("Updated WID Account", widData);
    console.log(
      "Custody transferred successfully to:",
      custody1.publicKey.toBase58()
    );
  });
});

function assertWithLog(condition: boolean, message: string) {
  if (condition) {
    console.log(`✅ Assertion passed: ${message}`);
  } else {
    console.error(`❌ Assertion failed: ${message}`);
  }
  assert(condition, message);
}

function printAccountData(accountName: string, data: any) {
  console.log(`\n${accountName} Account Data:`);
  console.log(JSON.stringify(data, null, 2));
}
