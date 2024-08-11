'use client';
/**
 * v0 by Vercel.
 * @see https://v0.dev/t/DHoPpuIM9Li
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
import React, { useContext, useState } from "react";
import Link from "next/link";

import { useAccount, useWriteContract } from "wagmi";
import { ethers } from "ethers";

import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { PLAYERS_CONTRACT_ADDRESS, Tier, tierPricer, TierText } from "@/lib/constants";
import { PLAYERS_ABI } from "@/contracts/abi";
import { toast } from "sonner";
import { waitTx } from "@/lib/utils";
import { UserContext } from "@/app/UserProvider";
import { Player as PlayerType } from "@/lib/types";
import { Player } from "./player";

const plans = [
  {
    tier: Tier.Diamond,
    name: TierText.Diamond,
    price: tierPricer(Tier.Diamond),
  },
  {
    tier: Tier.Platinum,
    name: TierText.Platinum,
    price: tierPricer(Tier.Platinum),
  },
  { tier: Tier.Gold, name: TierText.Gold, price: tierPricer(Tier.Gold) },
  { tier: Tier.Silver, name: TierText.Silver, price: tierPricer(Tier.Silver) },
  { tier: Tier.Bronze, name: TierText.Bronze, price: tierPricer(Tier.Bronze) },
];

export interface CardsProps {
  userPlayers: PlayerType[];
}

export const Cards: React.FC<CardsProps> = ({ userPlayers }) => {
  // const [userPlayers, setUserPlayers] = useState<PlayerType[]>([]);
  // Set up contract interaction using Wagmi
  const { address } = useAccount();
  const [tier, setTier] = useState<Tier>();
  const { writeContractAsync, isPending } = useWriteContract();
  const smthn = useContext(UserContext);

  // const handleSignature = async (tier: Tier) => {
  //   setTier(tier);
  //   const message = `Purchase order for ${address}`;
  //   const signature = await signMessageAsync({ message });
  //   await handlePurchase(signature);
  // };

  const handlePurchase = async (tier: Tier) => {
    const id = toast.loading("Processing purchase...");
    try {
      console.log('tier :>> ', tier);

      const hash = await writeContractAsync({
        abi: PLAYERS_ABI,
        address: PLAYERS_CONTRACT_ADDRESS,
        functionName: "requestPack",
        args: [tier],
        value: tierPricer(tier),
      });
      console.log("hash", hash);
      const team = await waitTx(hash);
      console.log("team", team);
      toast.success("Pack purchased successfully", { id });
    } catch (error) {
      toast.error("Failed to purchase pack", { id });
      console.log("Failed to purchase pack", error);
    }
  };

  return (
    <div className="bg-muted py-12 md:py-24">
      <div className="container grid gap-6 px-4 md:gap-8 md:px-6">
        <div className="flex flex-col items-center gap-4 md:flex-row md:justify-between">
          <div className="grid gap-1">
            <h2 className="text-2xl font-bold tracking-tight">Pricing</h2>
            <p className="text-muted-foreground">
              Select a team tier to recieve your first 15 players.
            </p>
          </div>
          <Link
            href="#"
            className="inline-flex h-10 items-center justify-center rounded-md bg-primary px-6 text-sm font-medium text-primary-foreground shadow transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
            prefetch={false}
          >
            Shop Now
          </Link>
        </div>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-5">
          {plans.map((plan) => (
            <Card key={plan.name} className="bg-[#e5e7eb]">
              <CardHeader className="bg-[#e5e7eb]">
                <CardTitle>{plan.name}</CardTitle>
              </CardHeader>
              <CardContent className="p-6">
                <div className="space-y-2">
                  <h3 className="text-4xl font-bold">
                    {ethers.formatEther(plan.price)} ETH
                  </h3>
                </div>
              </CardContent>
              <CardFooter>
                <Button
                  disabled={isPending || !address}
                  onClick={() => handlePurchase(plan.tier)}
                >
                  Select
                </Button>
              </CardFooter>
            </Card>
          ))}
        </div>

        <div className="flex flex-col">
          <h2 className="text-2xl font-bold tracking-tight">Your Players</h2>
          <p className="text-muted-foreground">
            Your current team of players.
          </p>

          <div className="flex flex-row flex-wrap gap-4">
            {
              userPlayers.length > 0 ? userPlayers.map((player) => {
                return (
                  <Player key={`user-player-${player.token_id}`} player={player} />
                )
              }) : <p>No players found</p>
            }
          </div>
        </div>
      </div>
    </div>
  );
}
