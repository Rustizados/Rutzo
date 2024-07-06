import { useState } from "react";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { ProgramMetadata } from "@gear-js/api";
import { UserEmptyAccount, RegisterButton, BoardGame, MyNFTCollection } from "@/components";
import {Link} from 'react-router-dom'
import { BoardGame2 } from "@/components/board-game/BoardGame2";
import { NotEnoughCards } from "../play/NotEnoughCards";

function Game() {
  const { api } = useApi();
  const { account } = useAccount();
  const [isRegister, setIsRegister] = useState(false);
  const [userNftsNumber, setUserNftsNumber] = useState(0);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const setData = async () => {    
    if (!api) return;

    const stateResult = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    try {
      const nftStateResult = await api
        .programState
        .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftContractMetadata);
      
      const nftStateFormated: any = nftStateResult.toJSON();
      
      const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

      const totalNfts = tokensForOwner.length;

      setUserNftsNumber(totalNfts);
    } catch (error) {
      console.log(error);
      setUserNftsNumber(0);
    }
  };

  setData();

  return (
    <div>
      <Link to={'/match'}>Go to Match</Link>
      {
        isRegister ? (
          <div>
            {
              userNftsNumber > 2 ? (
                <BoardGame2 />
              ) : (
                <NotEnoughCards />
              )
            }
          </div>
        ) : (
          <UserEmptyAccount/>
        )
      }
    </div>
  );
}

export { Game };
