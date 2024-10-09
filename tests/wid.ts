import { web3, BN } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Common } from "./common";
import { IdGatewayProgram } from "./id_gateway_program";
import { IdRegistryProgram } from "./id_registry_program";
import { SignerGatewayProgram } from "./signer_gateway";
import { SignerRegistryProgram } from "./signer_registry";

describe("WID (Wild Card ID) Registry", () => {
  let connection: web3.Connection;
  let custody1: web3.Keypair;
  let custody2: web3.Keypair;
  let custody3: web3.Keypair;
  let recovery1: web3.Keypair;
  let recovery2: web3.Keypair;
  let admin: web3.Keypair;
  let fuzzer: web3.Keypair;
  let signerKey1: web3.Keypair;
  let signerKey2: web3.Keypair;
  before(async () => {
    console.log("Setting up test environment...");
    connection = new web3.Connection("http://localhost:8899", "confirmed");
    admin = Common.getAdminKeypair();
    custody1 = web3.Keypair.generate();
    custody2 = web3.Keypair.generate();
    custody3 = web3.Keypair.generate();
    recovery1 = web3.Keypair.generate();
    recovery2 = web3.Keypair.generate();
    fuzzer = web3.Keypair.generate();
    signerKey1 = web3.Keypair.generate();
    signerKey2 = web3.Keypair.generate();
    await Common.airdrop(connection, admin.publicKey);
    console.log("Admin: ", admin.publicKey);
    console.log("Test environment set up successfully.");
  });

  it("should initialize the id gateway correctly", async () => {
    console.log("Initializing ID gateway...");
    const success = await IdRegistryProgram.initializeGateway(
      IdGatewayProgram.getInstance().programId
    );
    assertWithLog(success, "Succesfully complete transaction");
    const gatewayData = await IdRegistryProgram.fetchIdRegistryGatewayAccount();
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
    console.log(
      "Id Gateway initialized successfully inside Id Registry Gateway State."
    );
  });
  it("should initialize the key gateway correctly", async () => {
    console.log("Initializing Key gateway...");
    const success = await SignerRegistryProgram.initializeKeyGateway(
      SignerGatewayProgram.getInstance().programId,
      1000,
      5
    );
    const instance = SignerRegistryProgram.getInstance();
    assertWithLog(success, "Succesfully complete transaction");
    const gatewayData = await instance.program.account.keyRegistryGateway.fetch(
      SignerRegistryProgram.keyGatewayPda
    );
    assertWithLog(
      gatewayData.keyGatewayFrozen === false,
      "Key Gateway should not be frozen initially"
    );
    assertWithLog(
      gatewayData.keyGatewayProgram.toBase58() ===
        SignerGatewayProgram.getInstance().programId.toBase58(),
      "Key Gateway Program should match the Signer Gateway program"
    );
    assertWithLog(
      gatewayData.owner.toBase58() === admin.publicKey.toBase58(),
      "Owner should match the admin public key"
    );
    assertWithLog(
      1000 == gatewayData.maxKeysPerId,
      "Maxs keys per ID should match"
    );
    assertWithLog(5 == gatewayData.maxFlags, "Maxs flags should match");

    printAccountData("KEY Registry Gateway", gatewayData);
    console.log(
      "Signer Gateway initialized successfully in Signer Registry Gateway State"
    );
  });
  it("should not let register a new WID by raw invokation", async () => {
    console.log("Registering new WID WITHOUT GATEWAY...");
    const success = await IdRegistryProgram.registerRaw(
      custody1.publicKey,
      recovery1.publicKey
    );
    assertWithLog(
      success == false,
      "Registration without gateway passage MUST FAIL❗️"
    );
  });
  it("should register a new WID correctly", async () => {
    console.log("Registering new WID via gateway...");
    const success = await IdGatewayProgram.registerViaGateway(
      custody1.publicKey,
      recovery1.publicKey,
      IdRegistryProgram.getInstance().programId
    );
    assertWithLog(success == true, "Registry via gateway should succeed");

    const gatewayData = await IdRegistryProgram.fetchIdRegistryGatewayAccount();
    assertWithLog(
      gatewayData.idCounter.toString() === "1",
      "ID counter should increment to 1"
    );

    const widData = await IdRegistryProgram.fetchWidAccount(
      gatewayData.idCounter
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
    const widAddress = IdRegistryProgram.widAddress(new BN(1));
    let success = await IdRegistryProgram.transfer(
      widAddress,
      fuzzer,
      custody2.publicKey
    );
    assertWithLog(
      success == false,
      "Transfer should fail when signed by unauthorized custody❗️"
    );
    success = await IdRegistryProgram.transfer(
      widAddress,
      custody1,
      custody2.publicKey
    );
    assertWithLog(
      success == true,
      "Transfer should succeed when signed by authorized custody"
    );
    const widData = await IdRegistryProgram.fetchWidAccount(new BN(1));
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

  it("Should not let add signer via raw invokation", async () => {
    console.log("Adding signer WITHOUT GATEWAY...");
    const widAccount = await IdRegistryProgram.fetchWidAccount(new BN(1));
    const success = await SignerRegistryProgram.addSignerRaw(
      custody2,
      widAccount,
      1,
      signerKey1.publicKey.toBuffer(),
      [true, true, false, false, false],
      true
    );
    assertWithLog(
      success == false,
      "Adding signer key without gateway MUST FAIL❗️"
    );
  });
  it("Add add a new signer correctly", async () => {
    console.log("Adding signer WITH GATEWAY...");
    const widAccountOld = await IdRegistryProgram.fetchWidAccount(new BN(1));
    const success = await SignerGatewayProgram.addSignerViaGateway(
      custody2,
      widAccountOld,
      1,
      signerKey1.publicKey.toBuffer(),
      [true, true, false, false, false],
      true
    );
    assertWithLog(
      success == true,
      "Adding signer key with gateway MUST Succeed"
    );
    const widAccountNew = await IdRegistryProgram.fetchWidAccount(new BN(1));
    assertWithLog(
      widAccountNew.keyCounter == 1,
      "Key Counter inside WID account must increase to 1"
    );
    const keyAccount = await SignerRegistryProgram.fetchKeyAccount(
      widAccountOld
    );
    assertWithLog(keyAccount.isAdmin == true, "Key account should be admin");
    assertWithLog(
      keyAccount.parentKeyId == 0,
      "Parent key Id must be 0 for custody added keys"
    );
    assertWithLog(
      keyAccount.key.value.toString() ==
        signerKey1.publicKey.toBuffer().toString() &&
        keyAccount.key.keyType.toString() == "1",
      "Key Type and Buffer should match"
    );
    assertWithLog(keyAccount.keyId == 1, "Key Id should be 1");
    printAccountData("Key Account", keyAccount);
    printAccountData("Updated Wid Account", widAccountNew);
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
