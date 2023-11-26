import { ReactComponent as ShoppingCart } from "assets/images/shopping_cart.svg";
import { ReactComponent as GameController } from "assets/images/game_controller.svg";
import { RegisterButton,MyNFTCollection } from "components";
import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "consts";
import { useState } from "react";
import { UserEmptyAccount } from "./UserEmptyAccount";
import "./Collection.scss";

function Play() {
  const [userDoRegister, setUserDoRegister] = useState(false);
  const [hasEnoughCards, setHasEnoughCards] = useState(false);
  const [numberOfNfts, setNumberOfNfts] = useState(0);
  const [isRegister, setIsRegister] = useState(false);
  const { api } = useApi();
  const { account } = useAccount();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const setData = async () => {    
    const stateResult = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    try {
      const nftStateResult = await api
        .programState
        .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: "" }, nftContractMetadata);
      const nftStateFormated: any = nftStateResult.toJSON();
      console.log(nftStateFormated);
      
      const tokensForOwner: any = nftStateFormated.token.tokensForOwner ?? "";
      const userNfts = tokensForOwner.find((user: any) => user[0] === account?.decodedAddress);
      const totalNfts = userNfts[1].length;
      if (userNfts && totalNfts > 2) {
        setHasEnoughCards(true);
        setNumberOfNfts(totalNfts);
      } else {
        console.log("No se encontroal usuario!!");
        setHasEnoughCards(false);
        setNumberOfNfts(0);
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
                      <p className="alert">Go to the store to get some nfts!</p>
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
            <p className="alert">Register to obtain free cards</p>
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