export interface Team {
  roster: Roster;
  name: string;
  logo: string;
  team_rating: number;
}

export interface Roster {
  goal_tender?: Player;
  defense: Player[];
  mid: Player[];
  offense: Player[];
}

export interface Player {
  token_id: number;
  cid: string;
  image: string;
  name: string;
  tier: number;
  overall_rating: number;
  skills: Skills;
  skill_multiplier: number;
  jersey_number: number;
}

export interface Skills {
  speed: number;
  shooting: number;
  passing: number;
  dribbling: number;
  defense: number;
  physical: number;
  goal_tending: number;
}