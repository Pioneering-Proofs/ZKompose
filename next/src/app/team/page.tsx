/**
 * v0 by Vercel.
 * @see https://v0.dev/t/EavtehaJuak
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
export default function temas() {
  const teams = [
    {
      name: "Team A",
      score: 1250,
      tier: "Gold",
      wins: 24,
      losses: 12,
      winRate: "45%",
      streak: "+200",
    },
    {
      name: "Team B",
      score: 950,
      tier: "Silver",
      wins: 18,
      losses: 16,
      winRate: "53%",
      streak: "+50",
    },
    {
      name: "Team C",
      score: 1100,
      tier: "Silver",
      wins: 20,
      losses: 14,
      winRate: "58%",
      streak: "+100",
    },
  ];
  return (
    <div className="bg-background text-foreground min-h-screen flex flex-col items-center justify-center p-4 md:p-8">
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 w-full max-w-5xl">
        {teams.map((team, index) => (
          <div
            key={index}
            className="bg-card rounded-lg shadow-lg overflow-hidden"
          >
            <div className="h-40 bg-[url('/placeholder.svg')] bg-cover bg-center" />
            <div className="p-4 flex flex-col items-center gap-2">
              <h3 className="text-xl font-bold">{team.name}</h3>
              <div className="grid grid-cols-2 gap-4 w-full">
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.score}</div>
                  <div className="text-muted-foreground text-sm">Score</div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.tier}</div>
                  <div className="text-muted-foreground text-sm">Tier</div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.wins}</div>
                  <div className="text-muted-foreground text-sm">Wins</div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.losses}</div>
                  <div className="text-muted-foreground text-sm">Losses</div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.winRate}</div>
                  <div className="text-muted-foreground text-sm">Win Rate</div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-2xl font-bold">{team.streak}</div>
                  <div className="text-muted-foreground text-sm">Streak</div>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
