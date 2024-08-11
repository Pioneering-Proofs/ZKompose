"use client";
import { Roster } from "@/lib/types";
import React, { useState } from "react";
import { Player } from "./player";

export interface TeamProps {
  roster: Roster;
  name: string;
}

export const Field: React.FC<TeamProps> = ({ roster, name }) => {
  const [teamRoster, setTeamRoster] = useState<Roster | null>(roster);

  const handlePositionChange = (
    playerId: number | undefined,
    newPosition: string
  ) => {
    const newRoster = { ...teamRoster };
    const player =
      newRoster?.goal_tender?.token_id === playerId
        ? newRoster.goal_tender
        : newRoster?.defense?.find((player) => player.token_id === playerId)
          ? newRoster?.defense.find((player) => player.token_id === playerId)
          : newRoster?.mid?.find((player) => player.token_id === playerId)
            ? newRoster?.mid.find((player) => player.token_id === playerId)
            : newRoster?.offense?.find(
                (player) => player.token_id === playerId
              );
    if (player) {
      setTeamRoster(newRoster);
    }
  };

  return (
    <div className="relative flex flex-col w-full h-full">
      <img
        className="relative top-0 left-0 w-full h-full"
        src="./field.svg"
        alt="Soccer Field"
      />
      <div
        id="players"
        className="absolute top-0 left-0 w-full h-4/5 flex flex-col justify-around items-center"
      >
        <div id="goal_tender" className="player-group">
          <Player
            player={teamRoster?.goal_tender}
            onPositionChange={(position) =>
              handlePositionChange(teamRoster?.goal_tender.token_id, position)
            }
          />
        </div>
        <div id="defense" className="player-group">
          {teamRoster?.defense.map((player, index) => (
            <Player
              key={index}
              player={player}
              onPositionChange={(position) =>
                handlePositionChange(player.token_id, position)
              }
            />
          ))}
        </div>
        <div id="mid" className="player-group">
          {teamRoster?.mid.map((player, index) => (
            <Player
              key={index}
              player={player}
              onPositionChange={(position) =>
                handlePositionChange(player.token_id, position)
              }
            />
          ))}
        </div>
        <div id="offense" className="player-group">
          {teamRoster?.offense.map((player, index) => (
            <Player
              key={index}
              player={player}
              onPositionChange={(position) =>
                handlePositionChange(player.token_id, position)
              }
            />
          ))}
        </div>
      </div>
    </div>
  );
};
