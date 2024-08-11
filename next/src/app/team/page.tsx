"use client";
import { Player } from "@/components/player";
import { getPlayersByTokenIds } from "@/lib/tokens";
import { Field } from "@/components/Field";
import { Roster } from "@/lib/types";
import { useState } from "react";

/**
 * v0 by Vercel.
 * @see https://v0.dev/t/EavtehaJuak
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
export default async function Team() {
  const userPlayerTokenIds = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
  ];
  const players = await getPlayersByTokenIds(userPlayerTokenIds);

  const roster: Roster = {
    goal_tender: players[0],
    defense: players.slice(1, 4),
    mid: players.slice(4, 8),
    offense: players.slice(8, 15),
  };

  const [teamRoster, setTeamRoster] = useState<Roster>(roster);

  return (
    <div className="container">
      <div className="flex flex-col">
        <h2 className="text-2xl font-bold tracking-tight">Your Players</h2>
        <p className="text-muted-foreground">Your current team of players.</p>

        <div className="flex flex-row flex-wrap gap-4">
          {players.length > 0 ? (
            players.map((player) => {
              return (
                <Player
                  key={`user-player-${player.token_id}`}
                  player={player}
                />
              );
            })
          ) : (
            <p>No players found</p>
          )}
        </div>
      </div>
      <Field roster={teamRoster} name="name" />
    </div>
  );
}
