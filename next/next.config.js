/** @type {import('next').NextConfig} */
const nextConfig = {};

module.exports = nextConfig;

/**
 * const privKeyBuff = createHash("sha256").update(data).digest();
    const pubKey = secp256k1.publicKeyCreate(privKeyBuff, false); // uncompressed key
    const pubKeyHex = Buffer.from(pubKey).toString('hex');
    const privKey = privKeyBuff.toString('hex');
    const message = "we out here";

    console.log('pubKeyHex :>> ', pubKeyHex);

    const signature = EthCrypto.sign(
      privKey,
      EthCrypto.hash.keccak256(message)
    );
    const payload = {
      message,
      signature
    };
    const encrypted = await EthCrypto.encryptWithPublicKey(
      pubKeyHex, // by encrypting with bobs publicKey, only bob can decrypt the payload with his privateKey
      JSON.stringify(payload) // we have to stringify the payload before we can encrypt it
    );
    const encryptedString = EthCrypto.cipher.stringify(encrypted);
    const encryptedObject = EthCrypto.cipher.parse(encryptedString);
    const decrypted = await EthCrypto.decryptWithPrivateKey(
      privKey,
      encryptedObject
    );
    const decryptedPayload = JSON.parse(decrypted);
 */
