"use client";
import React from "react";

import { Player as PlayerType } from "@/lib/types";
import { tierToText } from "@/lib/utils";

export interface PlayerProps {
  className?: string;
  isSelected?: boolean;
  player?: PlayerType;
  onClick?: () => void;
}

export const Player: React.FC<PlayerProps> = ({ player, onClick, isSelected, className }) => {
  const tierName = tierToText(player?.tier);
  return (
    <div className={`player ${tierName} ${isSelected && 'border border-blue-500'} ${className}`} onClick={onClick}>
      <img src={player?.image} />
      <div className="stat">{player?.overall_rating}</div>
      {player?.jersey_number && <p className="jersey-num">No. {player?.jersey_number}</p>}
      <p className="name">{player?.name}</p>
      {/* <select onChange={(e) => onPositionChange(e.target.value)}>
        <option value="goal_tender">Goal Tender</option>
        <option value="defense">Defense</option>
        <option value="mid">Mid</option>
        <option value="offense">Offense</option>
      </select> */}
    </div>
  );
};
