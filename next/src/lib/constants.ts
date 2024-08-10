const ETH = 1e18;

export enum Tier {
  Diamond = 0,
  Platinum = 1,
  Gold = 2,
  Silver = 3,
  Bronze = 4,
}
export enum TierText {
  Diamond = "Diamond",
  Platinum = "Platinum",
  Gold = "Gold",
  Silver = "Silver",
  Bronze = "Bronze",
}

export const tierPricer = (tier: Tier): bigint => {
  switch (tier) {
    case Tier.Diamond:
      return BigInt(1 * ETH);
    case Tier.Platinum:
      return BigInt(0.5 * ETH);
    case Tier.Gold:
      return BigInt(0.25 * ETH);
    case Tier.Silver:
      return BigInt(0.1 * ETH);
    case Tier.Bronze:
      return BigInt(0.05 * ETH);
  }
};
