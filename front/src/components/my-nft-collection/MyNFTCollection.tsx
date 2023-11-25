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

function MyNFTCollection() {
  const { api } = useApi();
  const { account } = useAccount();

  const [allnfts, setAllnfts] = useState<any | undefined>([]);
  const [allmynft, setAllmynft] = useState<any | undefined>();
  const [existNFT, setExistNFT] = useState<any | undefined>(true);

  const [tokensForOwnerState, setTokensForOwnerState] = useState<
    any | undefined
  >([]);

  const [fullState, setFullState] = useState<any | undefined>({});

  const [mynftcollection, setMynftcollection] = useState<any | undefined>([]);

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
      .read({ programId: programIDNFT, payload: "" }, metadata)
      .then((result) => {
        setFullState(result.toJSON());

        console.log(fullState);

        const tokensForOwner: any = fullState.token.tokensForOwner ?? "";

        const tokenMetadataById: any = fullState.token.tokenMetadataById ?? "";

        tokenMetadataById.map((item: any) => alldatanfts.push(item[1]));

        setAllnfts(alldatanfts);

        setTokensForOwnerState(tokensForOwner);

        tokensForOwnerState.map((objeto: any) =>          
          objeto[0] === decodeAddress(currentaccount ?? "")
            ? setAllmynft(objeto[1])
            : console.log("No NFT")
        );

        allmynft.forEach((posicion: any) => {
          if (posicion >= 0 && posicion < allnfts.length) {
            mynftscollection.push(allnfts[posicion]);
          }
        });

        setMynftcollection(mynftscollection);
      })
      .catch(({ message }: Error) => console.log(message));
  };

  getMyNFT();

  return (
    <div>
      {existNFT ? (
        mynftcollection.map((elemento: any) => (
          <CollectionCard 
            image={elemento.media}
            title={elemento.name}
            type={elemento.description.toLowerCase()}
            value={elemento.reference}
          />
         /*
          <InfoNFT
            name={elemento.name}
            description={elemento.description}
            media={elemento.media}
            reference={elemento.reference}
          />
          */
        ))
      ) : (
        <h1>No NFTs</h1>
      )}
    </div>
  );
}

export { MyNFTCollection };
