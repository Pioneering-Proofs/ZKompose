import React from "react";
import { Roster } from "@/lib/types";
const players = await getPlayersByTokenIds(userPlayerTokenIds);

const TeamRoster = () => {
  const roster: Roster = {
    goal_tender: players[0],
    defense: players.slice(1, 4),
    mid: players.slice(4, 8),
    offense: players.slice(8, 15),
  };

  const [teamRoster, setTeamRoster] = useState<Roster>(roster);
  return <div>TeamRoster</div>;
};

export default TeamRoster;
