import { NFT_CONTRACT } from "consts";
import { ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { RegisterButton } from "components";
import { useApi } from "@gear-js/react-hooks";
import { Card } from "../../components/card/Card";

function InfoNFT({ name, description, media, reference }: any) {
  return (
    <Card
      image={media}
      title={name}
      type={description.toLowerCase()}
      value={reference}
      price={reference}
    >
      <RegisterButton />
    </Card>
  );
}

function AllNFTCollection() {
  const { api } = useApi();
  const [allnfts, setAllnfts] = useState<any | undefined>([]);
  const [fullState, setFullState] = useState<any | undefined>({});

  const alldatanfts: any[] = [];

  const programIDNFT = NFT_CONTRACT.PROGRAM_ID;

  const meta = NFT_CONTRACT.METADATA;

  const metadata = ProgramMetadata.from(meta);

  const getMyNFT = () => {
    api.programState
      .read({ programId: programIDNFT, payload: "" }, metadata)
      .then((result) => {
        setFullState(result.toJSON());

        const tokenMetadataById: any = fullState.token.tokenMetadataById ?? "";

        tokenMetadataById.map((item: any) => alldatanfts.push(item[1]));

        setAllnfts(alldatanfts);
      })
      .catch(({ message }: Error) => console.log(message));
  };

  getMyNFT();

  return (
    <>
      {allnfts.map((elemento: any) => (
        <InfoNFT
          name={elemento.name}
          description={elemento.description}
          media={elemento.media}
          reference={elemento.reference}
        />
      ))}
    </>
  );
}

export { AllNFTCollection };
