import React from "react";

export interface TeamCardProps {
  team: {
    name: string;
    score: number;
    tier: string;
    wins: number;
    losses: number;
    winRate: string;
    streak: string;
  }
}

export const TeamCard: React.FC<TeamCardProps> = ({ team }) => {

  return (
    <div
      className="bg-card rounded-lg shadow-lg overflow-hidden"
    >
      <div className="h-40 bg-[url('/placeholder.svg')] bg-cover bg-center" />
      <div className="p-4 flex flex-col items-center gap-2">
        <h3 className="text-xl font-bold">{team.name}</h3>
        <div className="grid grid-cols-2 gap-4 w-full">
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.score}</div>
            <div className="text-muted-foreground text-sm">Score</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.tier}</div>
            <div className="text-muted-foreground text-sm">Tier</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.wins}</div>
            <div className="text-muted-foreground text-sm">Wins</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.losses}</div>
            <div className="text-muted-foreground text-sm">Losses</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.winRate}</div>
            <div className="text-muted-foreground text-sm">Win Rate</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold">{team.streak}</div>
            <div className="text-muted-foreground text-sm">Streak</div>
          </div>
        </div>
      </div>
    </div>
  )
}
