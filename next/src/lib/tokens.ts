import { PLAYERS_JSON_CID, PLAYERS_SVG_CID } from "./constants"
import { Player } from "./types";

export const getPlayerByTokenId = async (tokenId: number): Promise<Player | null> => {
  // const url = `https://ipfs.io/ipfs/${PLAYERS_JSON_CID}/${tokenId}.json`;
  const url = `https://tan-hidden-gull-949.mypinata.cloud/ipfs/${PLAYERS_JSON_CID}/${tokenId}.json?pinataGatewayToken=G55brD2rpj1SpsguZn4ayE-IV35guvqsDM0q9abMAAiVvqFTcUr4Gk3pD9ZzoYQz`;
  // const url = `https://127.0.0.1:8080/ipfs/${PLAYERS_JSON_CID}/${tokenId}.json`;
  console.log('FETCHING url :>> ', url);
  const response = await fetch(url, {
    mode: 'cors',
    headers: {
      'Access-Control-Allow-Origin': '*'
    }
  });

  if (!response.ok) {
    return null;
  }
  const json = await response.json();

  return {
    ...json,
    token_id: tokenId,
    image: `https://tan-hidden-gull-949.mypinata.cloud/ipfs/${PLAYERS_SVG_CID}/${tokenId}.svg?pinataGatewayToken=G55brD2rpj1SpsguZn4ayE-IV35guvqsDM0q9abMAAiVvqFTcUr4Gk3pD9ZzoYQz`,
  };
}

export const getPlayersByTokenIds = async (tokenIds: number[]): Promise<Player[]> => {
  const players = await Promise.all(tokenIds.map(getPlayerByTokenId));
  return players.filter((player): player is Player => player !== null);
}