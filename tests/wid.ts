import { web3, BN } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Common } from "./common";
import { IdGatewayProgram } from "./id_gateway_program";
import { IdRegistryProgram } from "./id_registry_program";

describe("WID (Wild Card ID) Registry", () => {
  let connection: web3.Connection;
  let custody1: web3.Keypair;
  let custody2: web3.Keypair;
  let custody3: web3.Keypair;
  let recovery1: web3.Keypair;
  let recovery2: web3.Keypair;
  let admin: web3.Keypair;
  let fuzzer: web3.Keypair;
  before(async () => {
    console.log("Setting up test environment...");
    connection = new web3.Connection("http://localhost:8899", "confirmed");
    admin = IdRegistryProgram.getAdminKeypair();
    custody1 = web3.Keypair.generate();
    custody2 = web3.Keypair.generate();
    custody3 = web3.Keypair.generate();
    recovery1 = web3.Keypair.generate();
    recovery2 = web3.Keypair.generate();
    fuzzer = web3.Keypair.generate();
    await Common.airdrop(connection, admin.publicKey);
    console.log("Admin: ", admin.publicKey);
    console.log("Test environment set up successfully.");
  });

  it("should initialize the gateway correctly", async () => {
    console.log("Initializing gateway...");
    const success = await IdRegistryProgram.initialize_gateway();
    assertWithLog(success, "Succesfully complete transaction");
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
        IdGatewayProgram.getInstance().programId.toBase58(),
      "ID Gateway should match the Gateway program"
    );
    assertWithLog(
      gatewayData.owner.toBase58() === admin.publicKey.toBase58(),
      "Owner should match the admin public key"
    );

    printAccountData("ID Registry Gateway", gatewayData);
    console.log("Gateway initialized successfully.");
  });
  it("should not let register a new WID by raw invokation", async () => {
    console.log("Registering new WID WITHOUT GATEWAY...");
    const success = await IdRegistryProgram.registerRaw(
      custody1.publicKey,
      recovery1.publicKey
    );
    assertWithLog(
      success == false,
      "Registration without gateway passage MUST FAIL"
    );
  });
  it("should register a new WID correctly", async () => {
    console.log("Registering new WID via gateway...");
    // const succ = await IdGatewayProgram.registerViaGateway(
    //   custody.publicKey,
    //   recovery.publicKey,
    //   fuzzer.publicKey
    // );
    // assertWithLog(
    //   succ == false,
    //   "Registry via gateway should fail if given incorrect registry program ID"
    // );
    const success = await IdGatewayProgram.registerViaGateway(
      custody1.publicKey,
      recovery1.publicKey,
      IdRegistryProgram.getInstance().programId
    );
    assertWithLog(success == true, "Registry via gateway should succeed");

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
      widData.custody.toBase58() === custody1.publicKey.toBase58(),
      "Custody public key should match the registered one"
    );
    assertWithLog(
      widData.recovery.toBase58() === recovery1.publicKey.toBase58(),
      "Recovery public key should match the registered one"
    );

    printAccountData("ID Registry Gateway", gatewayData);
    printAccountData("WID Account", widData);
    console.log("WID registered successfully with ID:", widData.wid.toString());
  });

  it("should transfer custody correctly", async () => {
    console.log("Transferring custody...");
    const wcAddress = IdRegistryProgram.wid_address(new BN(1));
    await IdRegistryProgram.transfer(wcAddress, custody1, custody2.publicKey);

    const widData = await IdRegistryProgram.program().account.widAccount.fetch(
      wcAddress
    );

    assertWithLog(
      widData.custody.toBase58() !== custody1.publicKey.toBase58(),
      "Custody should have changed from the original"
    );
    assertWithLog(
      widData.custody.toBase58() === custody2.publicKey.toBase58(),
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
