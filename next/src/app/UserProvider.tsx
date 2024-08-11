import { createContext, useState } from "react";
import { useAccountEffect, useSignMessage, useAccount } from "wagmi";
import { createHash } from "crypto";

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
  const [key, setKey] = useState<Buffer | null>(null);
  const UserContext = createContext({ key });
  const { signMessage } = useSignMessage();

  const { address, status } = useAccount();
  let isNewConnection = true;


  const setKeys = async (signData: string) => {
    try {
      const privKey = createHash("sha256").update(signData).digest();
      window.localStorage.setItem("privKey", privKey.toString());
      setKey(privKey);
      return privKey;
    } catch (e) {
      console.error("Failed to create private key: ", e);
    }
  };

  const getLocalKey = async () => {
    try {
      const privKeyString = window.localStorage.getItem("privKey");
      if (privKeyString) {
        const privKey = Buffer.from(privKeyString, "hex");
        setKey(privKey);
        return privKey;
      }
    } catch (e) {
      console.error("Failed to get private key from local storage: ", e);
    }
  };

  useAccountEffect({
    async onConnect(data) {
      const signData = "session data" + { data };
      console.log("signData: ", signData);

      console.log("address: ", address);
      if (!address) {
        signMessage({ message: "You are connecting your wallet to the app" });
        isNewConnection = false;
        if (window.localStorage.getItem("privKey") === null) {
          const privKey = await setKeys(signData);

          console.log("new privKey: ", privKey);
        } else {
          try {
            const privKey = await getLocalKey();
            console.log("existing privKey: ", privKey);
            return privKey;
          } catch (e) {
            console.error("Failed to get private key from local storage: ", e);
          }

        }
      }
    },
    async onDisconnect() {
      window.localStorage.removeItem("privKey");
      setKey(null);
    },
  });
  return (
    <UserContext.Provider value={{ key }}>{children}</UserContext.Provider>
  );
};
