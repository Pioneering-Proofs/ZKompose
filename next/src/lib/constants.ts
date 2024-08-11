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
      return BigInt(0.0005 * ETH);
    case Tier.Platinum:
      return BigInt(0.0004 * ETH);
    case Tier.Gold:
      return BigInt(0.0003 * ETH);
    case Tier.Silver:
      return BigInt(0.0002 * ETH);
    case Tier.Bronze:
      return BigInt(0.0001 * ETH);
  }
};

export const PLAYERS_CONTRACT_ADDRESS = "0xd3F6e84F932BC3D0E71C97b094149FB08385d90E";

export const PLAYERS_JSON_CID = "QmSNmFFgwPdhB6miCe2JuUhTFT969eAH4Cya16na8GNVP9"
export const PLAYERS_SVG_CID = "Qmf7E9Zg8uVQ5p34QvdMFtLGHDiPXtTL5j26gFX4euXj3P"