'use client';
import React, { useState } from "react";

import { Player as PlayerType, Roster } from "@/lib/types";
import { Player } from "@/components/player";
import { Field } from "@/components/Field";
import { Button } from "@/components/ui/button";

export interface TeamSelectorProps {
  players: PlayerType[]
}

const defaultRoster: Roster = {
  defense: [],
  mid: [],
  offense: [],
}


export const TeamSelector: React.FC<TeamSelectorProps> = ({ players: playersInit }) => {
  const [players, setPlayers] = useState<PlayerType[]>(playersInit);
  const [roster, setRoster] = useState<Roster>(defaultRoster);
  const [selectedPlayer, setSelectedPlayer] = useState<PlayerType | null>(null);

  const onSelectPosition = (position: string, index?: number) => {
    if (!selectedPlayer) return;

    if (position === 'goal_tender') {
      setRoster({ ...roster, goal_tender: selectedPlayer });
    } else {
      const newRoster = { ...roster };
      // @ts-ignore
      newRoster[position][index] = selectedPlayer;
      setRoster(newRoster);
    }
    setPlayers(players.filter((player) => player.token_id !== selectedPlayer.token_id));
  }

  return (
    <div className="flex flex-col">

      <div className="flex flex-row gap-4 items-start">
        <div className="flex flex-col items-center gap-4 w-2/3">
          <Field roster={roster} name="name" onSelectPosition={onSelectPosition} />
          <Button onClick={() => setSelectedPlayer(null)}>Clear Selection</Button>
        </div>
        <div className="flex flex-row flex-wrap gap-4 w-1/3">
          {players.length > 0 ? (
            players.map((player) => {
              return (
                <Player
                  key={`user-player-${player.token_id}`}
                  className="min-w-24"
                  isSelected={selectedPlayer?.token_id === player.token_id}
                  onClick={() => setSelectedPlayer(player)}
                  player={player}
                />
              );
            })
          ) : (
            <p>No players found</p>
          )}
        </div>
      </div>
    </div>

  )
}