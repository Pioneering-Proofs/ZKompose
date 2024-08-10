const ETH = 1e18;

enum Tier {
  Diamond = 0,
  Platinum = 1,
  Gold = 2,
  Silver = 3,
  Bronze = 4,
}

const tierPricer = (tier: Tier) => {
  switch (tier) {
    case Tier.Diamond:
      return 1 * ETH;
    case Tier.Platinum:
      return 0.5 * ETH;
    case Tier.Gold:
      return 0.25 * ETH;
    case Tier.Silver:
      return 0.1 * ETH;
    case Tier.Bronze:
      return 0.05 * ETH;
  }
};

export { Tier, tierPricer };
