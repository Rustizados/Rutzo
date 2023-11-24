import { NFT_CONTRACT } from "consts";
import * as process from "process";
import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { CollectionCard } from "components/collection-card";
import { ReactComponent as ShoppingCart } from "assets/images/shopping_cart.svg";
import { ReactComponent as GameController } from "assets/images/game_controller.svg";
import { EmptyCollection } from "./EmptyCollection";
import "./EmptyCollection.scss";
import "./Collection.scss";

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
  const [existNFT, setExistNFT] = useState<boolean>(true);

  const [tokensForOwnerState, setTokensForOwnerState] = useState<
    any | undefined
  >([]);

  const [fullState, setFullState] = useState<any | undefined>({});

  const [mynftcollection, setMynftcollection] = useState<any | undefined>([]);

  const alldatanfts: any[] = [];

  const mynftscollection: any[] = [];

  // Add your programID // NFT
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

        const tokensForOwner: any = fullState.token.tokensForOwner ?? "";

        const tokenMetadataById: any = fullState.token.tokenMetadataById ?? "";

        tokenMetadataById.map((item: any) => alldatanfts.push(item[1]));

        setAllnfts(alldatanfts);

        setTokensForOwnerState(tokensForOwner);

        let foundNFT = false;

        tokensForOwnerState.forEach((objeto: any) => {
          if (objeto[0] === decodeAddress(currentaccount ?? "")) {
            foundNFT = true;
            setAllmynft(objeto[1]);
          }
        });

        setExistNFT(foundNFT);

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
        <div>
          {mynftcollection.length >= 3 ? (
            <div className="alert">
              <h1>Your NFT collection</h1>
              {mynftcollection.map((elemento: any) => (
                <InfoNFT
                  name={elemento.name}
                  description={elemento.description}
                  media={elemento.media}
                  reference={elemento.reference}
                />
              ))}
              <br />
              <div className="playcontainer">
                <a href="/game">
                  <GameController />
                  PLAY
                </a>
              </div>
            </div>
          ) : (
            <div className="alert">
              <h1>You don&apos;t have enough NFTs</h1>
              {mynftcollection.map((elemento: any) => (
                <InfoNFT
                  name={elemento.name}
                  description={elemento.description}
                  media={elemento.media}
                  reference={elemento.reference}
                />
              ))}
              <br />
              <div className="playcontainer">
                <a href="/marketplace">
                  <ShoppingCart />
                  MARKETPLACE
                </a>
              </div>
            </div>
          )}
        </div>
      ) : (
        <EmptyCollection />
      )}
    </div>
  );
}

export { MyNFTCollection };
