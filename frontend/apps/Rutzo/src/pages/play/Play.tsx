import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import { RegisterButton, MyNFTCollection, UserEmptyAccount } from "@/components";
import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { useState } from "react";
import "./Collection.scss";

function Play() {
  const { api } = useApi();
  const { account } = useAccount();
  const [hasEnoughCards, setHasEnoughCards] = useState(false);
  const [numberOfNfts, setNumberOfNfts] = useState(0);
  const [isRegister, setIsRegister] = useState(false);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const setData = async () => {   
    if (!account || !api) return; 
    
    const stateResult = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    try {
      const nftStateResult = await api
        .programState
        .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, ProgramMetadata.from(NFT_CONTRACT.METADATA));
      
      const nftStateFormated: any = nftStateResult.toJSON();
      
      const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

      const totalNfts = tokensForOwner.length;

      setNumberOfNfts(totalNfts);

      if (totalNfts > 2) {
        setHasEnoughCards(true);
      } else {
        setHasEnoughCards(false);
      }
    } catch (error) {
      console.log(error);
      setHasEnoughCards(false);
    }

  };

  setData();

  return (
    <div className="play-title">
      { 
        isRegister ? (
          <div>
            {hasEnoughCards ? (
              <div className="alert">
                <h1>Your NFT collection</h1>
                <MyNFTCollection />
                <br />
                <div className="playcontainer">
                  <a href="/game">
                    <GameController />
                    PLAY
                  </a>
                </div>
              </div>
            ) : (
              <div>
                {
                  numberOfNfts > 0 ? (
                    <div className="alert">
                      <h1>You don&apos;t have enough NFTs</h1>
                      <MyNFTCollection />
                      <br />
                      <div className="playcontainer">
                        <div className="playcontainer">
                          <a href="/marketplace">
                            <ShoppingCart />
                            MARKETPLACE
                          </a>
                        </div>
                      </div>
                    </div>
                  ) : (
                    <UserEmptyAccount>
                      <p className="alert">Go to the marketplace and get some cool NFTs!</p>
                      <div className="playcontainer">
                        <a href="/marketplace" className="alert">
                          MARKETPLACE
                        </a>
                      </div>
                    </UserEmptyAccount>
                  )
                }
              </div> 
            )}
          </div>
        ) : (
          <UserEmptyAccount>
            <p className="alert">Register to get free cards</p>
            <RegisterButton onRegister={setData} />
          </UserEmptyAccount>
        )
      }
    </div>
  );
}

export { Play };


// return (
//   <div className="play-title">
//     <MyNFTCollection />
//   </div>
// );