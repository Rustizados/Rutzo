import { useState, useEffect } from "react";
import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { ProgramMetadata } from "@gear-js/api";
import { UserEmptyAccount, RegisterButton, BoardGame, MyNFTCollection } from "@/components";

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
      {
        isRegister ? (
          <div>
            {
              userNftsNumber > 2 ? (
                <BoardGame />
              ) : (
                <div>
                  {
                    userNftsNumber > 0 ? (
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
              )
            }
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

export { Game };
