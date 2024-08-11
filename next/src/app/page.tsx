import { Cards } from "@/components/Cards";
import { getPlayersByTokenIds } from "@/lib/tokens";

export default async function Home() {

  const userPlayerTokenIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
  const players = await getPlayersByTokenIds(userPlayerTokenIds);

  return (
    <main>

      <Cards userPlayers={players} />
    </main>
  );
}
