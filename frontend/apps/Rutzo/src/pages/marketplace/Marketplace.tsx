import { MAIN_CONTRACT } from "@/app/consts";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { useState } from "react";
import { ReactComponent as Banner } from "@/assets/images/marketplace.svg";
import { DefaultNfts, RegisterButton, NftsOnSale } from "@/components";
import { ProgramMetadata } from "@gear-js/api";
import "./Marketplace.scss";


function Marketplace() {
  const { api } = useApi();
  const { account } = useAccount();
  const [isRegister, setIsRegister] = useState(false);
  const [totalNftsToMint, setTotalNftsToMint] = useState(3);

  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const setData = async () => {    
    if (!api) return;

    const stateResult = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    const stateResult2 = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { NFTsPurchasedByUser: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated2: any = stateResult2.toJSON();

    const mintedNfts: [number] = stateFormated2.purchasedNfts;

    setTotalNftsToMint(3 - (mintedNfts?.length ?? 0))

  };

  setData();

  return (
    <div className="text-center">
      <Banner style={{ width: "50%", alignSelf: "center", padding: 0 }} />
      <h2 style={{ marginBottom: 65 }}>
        Get ready for the battle with some cool NFTs
      </h2>
      {
        isRegister ? (
          <>
            {
              totalNftsToMint > 0 && 
              <h2 style={{ marginBottom: 30 }}>
                Free NFTs, pick {totalNftsToMint}!
              </h2>
            }
            <div>
              <div className="cards-container">
                <DefaultNfts onMinted={setData}/>
                <br />
                { totalNftsToMint !== 0 && <h2 style={{ marginBottom: 30, marginTop: 10 }}>
                    NFTs for sale
                  </h2>}
                <NftsOnSale />
              </div>
            </div>
          </>
        ) : (
          <>
            <h2 style={{ marginBottom: 30 }}>
              To get nfts you must register!
            </h2>
            <div className="empty_container">
              <RegisterButton onRegister={setData} />
            </div>
          </>
        )
      }
    </div>
  );
}

export { Marketplace };
