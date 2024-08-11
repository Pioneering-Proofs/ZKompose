"use client";
import { Player as PlayerType } from "@/lib/types";
import { tierToText } from "@/lib/utils";
import React from "react";

export interface PlayerProps {
  player?: PlayerType;
  onPositionChange: (position: string) => void;
}
const onPositionChange = (position: string) => {
  console.log(position);
};

export const Player: React.FC<PlayerProps> = ({ player }) => {
  const tierName = tierToText(player?.tier);
  return (
    <div className={`player ${tierName}`}>
      <img src={player?.image} />
      <div className="stat">{player?.overall_rating}</div>
      <p className="jersey-num">No. {player?.jersey_number}</p>
      <p className="name">{player?.name}</p>
      <select onChange={(e) => onPositionChange(e.target.value)}>
        <option value="goal_tender">Goal Tender</option>
        <option value="defense">Defense</option>
        <option value="mid">Mid</option>
        <option value="offense">Offense</option>
      </select>
    </div>
  );
};
