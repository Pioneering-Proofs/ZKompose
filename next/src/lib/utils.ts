import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { waitForTransactionReceipt } from 'wagmi/actions';

import { getConfig } from "@/wagmi";
import { Address } from "viem";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const waitTx = (txHash: string) => waitForTransactionReceipt(getConfig(), { hash: txHash as Address });
