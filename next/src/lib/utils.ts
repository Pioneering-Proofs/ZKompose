import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { waitForTransactionReceipt } from 'wagmi/actions';

import { getConfig } from "@/wagmi";
import { Address } from "viem";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const waitTx = (txHash: string) => waitForTransactionReceipt(getConfig(), { hash: txHash as Address });

export const tierToText = (tier?: number) => {
  switch (tier) {
    case 0:
      return "diamond";
    case 1:
      return "platinum";
    case 2:
      return "gold";
    case 3:
      return "silver";
    case 4:
      return "bronze";
    default:
      return "";
  }
}