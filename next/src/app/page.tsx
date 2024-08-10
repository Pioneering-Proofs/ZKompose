"use client";

import { useAccount, useConnect, useDisconnect } from "wagmi";
import Navbar from "../components/Navbar";
import Cards from "@/components/Cards";

function App() {
  const { address, status, chainId } = useAccount();
  const { connectors, connect, error } = useConnect();
  const { disconnect } = useDisconnect();

  return (
    <>
      <div>
        <h2 className="text-xl">Account</h2>

        <div>
          status: {status}
          <br />
          address: {address}
          <br />
          chainId: {chainId}
        </div>

        {status === "connected" && (
          <button type="button" onClick={() => disconnect()}>
            Disconnect
          </button>
        )}
      </div>

      <div>
        <h2>Connect</h2>
        {connectors.map((connector) => (
          <button
            key={connector.uid}
            onClick={() => connect({ connector })}
            type="button"
          >
            {connector.name}
          </button>
        ))}
        <div>{status}</div>
        <div>{error?.message}</div>
      </div>

      <Navbar />
      <Cards address={address} />
    </>
  );
}

export default App;