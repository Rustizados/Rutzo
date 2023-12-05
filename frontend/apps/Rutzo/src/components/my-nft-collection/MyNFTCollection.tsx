import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { CollectionCard } from "../collection-card";
import { NFT_CONTRACT, MAIN_CONTRACT } from "@/app/consts";


function MyNFTCollection() {
  const { api } = useApi();
  const { account } = useAccount();
  const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
  const [tokensLoaded, setTokensLoaded] = useState(false);

  // Add your programID
  const programIDNFT = NFT_CONTRACT.PROGRAM_ID;

  const nftMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const getMyNFT = () => {
    if (!api || tokensLoaded) return;
    api.programState
      .read({ programId: programIDNFT, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftMetadata)
      .then((result) => {

        const nftStateFormated: any = result.toJSON();
      
        const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

        setTokensForOwnerState(tokensForOwner);

      })
      .catch(({ message }: Error) => {
        console.log(message);
      });
    
    setTokensLoaded(true);
  };

  getMyNFT();

  return (
    <div>
      {tokensForOwnerState.length > 0 ? (
        tokensForOwnerState.map((element: any) => {
          const [nftId, elemento] = element;
          return <CollectionCard 
            image={elemento.media}
            title={elemento.name}
            type={elemento.description.toLowerCase()}
            value={elemento.reference}
            key={nftId}
          />;
        })
      ) : (
        <h1>You don't have NFTS</h1>
      )}
    </div>
  );
}

export { MyNFTCollection };
