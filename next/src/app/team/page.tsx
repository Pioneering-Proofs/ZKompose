import { getPlayersByTokenIds } from "@/lib/tokens";
import { TeamSelector } from "../teamSelector";

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

  return (
    <div className="container">
      <div className="flex flex-col gap-4">
        <div className="flex flex-col">
          <h2 className="text-2xl font-bold tracking-tight">Your Players</h2>
          <p className="text-muted-foreground">Your current team of players.</p>
        </div>

        <TeamSelector players={players} />
      </div>
    </div>
  );
}
