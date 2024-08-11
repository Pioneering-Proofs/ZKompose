import { createContext, useState } from "react";
import { useAccountEffect, useSignMessage, useAccount, useSwitchChain } from "wagmi";
import { createHash } from "crypto";
import { sepolia } from "viem/chains";

export const UserContext = createContext<Buffer | null>(null);

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
  const [key, setKey] = useState<Buffer | null>(null);
  const { signMessageAsync } = useSignMessage();
  const { chainId } = useAccount();
  const { switchChainAsync } = useSwitchChain();

  const setKeys = async (signData: string) => {
    try {
      const privKey = createHash("sha256").update(signData).digest();
      window.localStorage.setItem("privKey", privKey.toString("hex"));
      setKey(privKey);
      return privKey;
    } catch (e) {
      console.error("Failed to create private key: ", e);
    }
  };

  const getLocalKey = async () => {
    try {
      const privKeyString = getPrivKey();
      if (!privKeyString) {
        return
      }
      const privKey = Buffer.from(privKeyString, "hex");
      setKey(privKey);
      return privKey;
    } catch (e) {
      console.error("Failed to get private key from local storage: ", e);
    }
  };

  const getPrivKey = () => {
    return window.localStorage.getItem("privKey");
  }

  const correctChain = async () => {
    if (chainId !== sepolia.id) {
      await switchChainAsync({ chainId: sepolia.id });
    }
  }

  useAccountEffect({
    async onConnect(data) {
      await correctChain();
      if (getPrivKey()) {
        getLocalKey();
        return;
      }

      const signData = await signMessageAsync({ account: data.address, message: "You certify that you own this wallet" });
      const privKey = await setKeys(signData);

      console.log("new privKey: ", privKey);
    },
    async onDisconnect() {
      window.localStorage.removeItem("privKey");
      setKey(null);
    },
  });

  return (
    <UserContext.Provider value={key}>{children}</UserContext.Provider>
  );
};
