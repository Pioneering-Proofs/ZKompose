import { Player as PlayerType } from "@/lib/types";
import { tierToText } from "@/lib/utils";
import React from "react";

export interface PlayerProps {
  player?: PlayerType;
}

export const Player: React.FC<PlayerProps> = ({ player }) => {
  const tierName = tierToText(player?.tier);
  return (
    <div className={`player ${tierName}`}>
      <img src={player?.image} />
      <div className="stat">{player?.overall_rating}</div>
      <p className="jersey-num">No. {player?.jersey_number}</p>
      <p className="name">{player?.name}</p>
    </div>
  );
};