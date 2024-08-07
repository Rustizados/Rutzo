import { useState } from "react";
import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { ProgramMetadata } from "@gear-js/api";
import {
  UserEmptyAccount,
  BoardGame,
  MyNFTCollection,
} from "@/components";
import { NotRegistered } from "../play/NotRegistered";
import { NotEnoughCards } from "../play/NotEnoughCards";

function Select() {
  const { api } = useApi();
  const { account } = useAccount();
  const [isRegister, setIsRegister] = useState(false);
  const [userNftsNumber, setUserNftsNumber] = useState(0);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const setData = async () => {
    if (!api) return;

    const stateResult = await api.programState.read(
      {
        programId: MAIN_CONTRACT.PROGRAM_ID,
        payload: { UserIsRegister: account?.decodedAddress ?? "0x0" },
      },
      mainContractMetadata
    );

    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    try {
      const nftStateResult = await api.programState.read(
        {
          programId: NFT_CONTRACT.PROGRAM_ID,
          payload: { tokensForOwner: account?.decodedAddress ?? "0x0" },
        },
        nftContractMetadata
      );

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
      {isRegister ? (
        <div>
          {userNftsNumber > 2 ? (
            <div className="mx-32">
              <h1 className="title text-4xl font-extrabold dark:text-white">
                Choose your cards
              </h1>
              <BoardGame />
            </div>
          ) : (
            <div>
              {userNftsNumber > 0 ? (
                <NotEnoughCards />
              ) : (
                <UserEmptyAccount />
              )}
            </div>
          )}
        </div>
      ) : (
        <NotRegistered />
      )}
    </div>
  );
}

export { Select };
