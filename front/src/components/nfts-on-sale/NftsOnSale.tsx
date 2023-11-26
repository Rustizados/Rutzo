import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { CollectionCard } from "components/collection-card";
import { NFT_CONTRACT, MAIN_CONTRACT } from "consts";
import process from "process";
import { AnyJson } from "@polkadot/types/types";

// non-essential extra component
function InfoNFT({ name, description, media, reference }: any) {
  return (
    <CollectionCard
      image={media}
      title={name}
      type={description.toLowerCase()}
      value={reference}
    />
  );
}

export function NftsOnSale() {
  const { api } = useApi();
  const { account } = useAccount();
  const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);

  const alldatanfts: any[] = [];

  const mynftscollection: any[] = [];

  // Add your programID
  const programIDNFT = NFT_CONTRACT.PROGRAM_ID;

  // Add your metadata.txt
  const meta = NFT_CONTRACT.METADATA;

  const metadata = ProgramMetadata.from(meta);

  const currentaccount = account?.address;



  const getMyNFT = () => {
    api.programState
      .read({ programId: programIDNFT, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, metadata)
      .then((result) => {

        const nftStateFormated: any = result.toJSON();
      
        const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

        setTokensForOwnerState(tokensForOwner);

      })
      .catch(({ message }: Error) => {
        console.log(message);
      });
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
          />;
        })
      ) : (
        <h1>No NFTs</h1>
      )}
    </div>
  );
}