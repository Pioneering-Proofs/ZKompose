"use client";

import Link from "next/link";
import { useAccount, useConnect, useDisconnect } from "wagmi";
import Navbar from "../components/Navbar";

function App() {
  const account = useAccount();
  const { connectors, connect, status, error } = useConnect();
  const { disconnect } = useDisconnect();

  return (
    <>
      <div>
        <h2 className="text-xl">Account</h2>

        <div>
          status: {account.status}
          <br />
          addresses: {JSON.stringify(account.addresses)}
          <br />
          chainId: {account.chainId}
        </div>

        {account.status === "connected" && (
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
      <section className="grid grid-cols-2 gap-4 p-4 md:grid-cols-4 md:p-6">
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 1"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 1</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 2"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 2</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 3"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 3</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 4"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 4</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
      </section>
    </>
  );
}

export default App;
