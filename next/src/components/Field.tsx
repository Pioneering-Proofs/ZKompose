"use client";
import { Position, Roster } from "@/lib/types";
import React, { useState } from "react";
import { Player } from "./player";

export interface TeamProps {
  roster: Roster;
  name: string;
  onSelectPosition: (position: Position, index?: number) => void;
}

export const Field: React.FC<TeamProps> = ({ roster, name, onSelectPosition }) => {

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
          <Player player={roster?.goal_tender} onClick={() => onSelectPosition(Position.goal_tender)} />
        </div>
        <div id="defense" className="player-group">
          <Player player={roster?.defense[0]} onClick={() => onSelectPosition(Position.defense, 0)} />
          <Player player={roster?.defense[1]} onClick={() => onSelectPosition(Position.defense, 1)} />
          <Player player={roster?.defense[2]} onClick={() => onSelectPosition(Position.defense, 2)} />
          <Player player={roster?.defense[3]} onClick={() => onSelectPosition(Position.defense, 3)} />
        </div>
        <div id="mid" className="player-group">
          <Player player={roster?.mid[0]} onClick={() => onSelectPosition(Position.mid, 0)} />
          <Player player={roster?.mid[1]} onClick={() => onSelectPosition(Position.mid, 1)} />
          <Player player={roster?.mid[2]} onClick={() => onSelectPosition(Position.mid, 2)} />
        </div>
        <div id="offense" className="player-group">
          <Player player={roster?.offense[0]} onClick={() => onSelectPosition(Position.offense, 0)} />
          <Player player={roster?.offense[1]} onClick={() => onSelectPosition(Position.offense, 1)} />
          <Player player={roster?.offense[2]} onClick={() => onSelectPosition(Position.offense, 2)} />
        </div>
      </div>
    </div>
  );
};