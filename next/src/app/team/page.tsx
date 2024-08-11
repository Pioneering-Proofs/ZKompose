import { Player } from "@/components/player";
import { getPlayersByTokenIds } from "@/lib/tokens";

/**
 * v0 by Vercel.
 * @see https://v0.dev/t/EavtehaJuak
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
export default async function Team() {

  const userPlayerTokenIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
  const players = await getPlayersByTokenIds(userPlayerTokenIds);

  return (
    <div className="container">
      <div className="flex flex-col">
        <h2 className="text-2xl font-bold tracking-tight">Your Players</h2>
        <p className="text-muted-foreground">
          Your current team of players.
        </p>

        <div className="flex flex-row flex-wrap gap-4">
          {
            players.length > 0 ? players.map((player) => {
              return (
                <Player key={`user-player-${player.token_id}`} player={player} />
              )
            }) : <p>No players found</p>
          }
        </div>
      </div>
    </div>
  );
}
