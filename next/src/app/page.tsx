"use client";

import { useAccount, useConnect, useDisconnect } from "wagmi";
import Navbar from "../components/Navbar";
import Cards from "@/components/Cards";
import { UserProvider } from "./UserProvider";

import { Button } from "@/components/ui/button";

function App() {
  const { address, status, chainId } = useAccount();
  const { connectors, connect, error } = useConnect();
  const { disconnect } = useDisconnect();

  const account = useAccount();
  return (
    <UserProvider>
      <div className="ml-auto flex gap-2">
        {account.status === "connected" && (
          <Button variant={"outline"} onClick={() => disconnect()}>
            Disconnect
          </Button>
        )}
        {connectors.map((connector) => (
          <Button
            key={connector.uid}
            onClick={() => connect({ connector })}
            type="button"
          >
            {connector.name}
          </Button>
        ))}
      </div>

      <Cards address={address} />
    </UserProvider>
  );
}

export default App;
