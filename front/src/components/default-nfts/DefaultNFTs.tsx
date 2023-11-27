import { gasToSpend } from "utils";
import { ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { web3FromSource } from "@polkadot/extension-dapp";
import { useApi, useAccount, useAlert } from "@gear-js/react-hooks";
import { CollectionCard } from "components/collection-card";
import { NFT_CONTRACT, MAIN_CONTRACT } from "consts";
import { Card, RegisterButton } from "components";
import process from "process";
import { AnyJson } from "@polkadot/types/types";
import { Button } from "@gear-js/ui";

function DefaultNfts() {
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();
  const [defaultsNFTs, setDefaultsNFTs] = useState<any>([]);
  const [canMint, setCanMint] = useState(true);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const mintDefaultNft = async (nftId: number) => {
    if (!api) return;

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { MintCard: [nftId] },
      0,
      false,
      mainContractMetadata
    );

    if (!account) {
      alert.error("Account not available to sign");
      return;
    }

    const message: any = {
      destination: MAIN_CONTRACT.PROGRAM_ID, // programId
      payload: { MintCard: [nftId] }, // Add your data
      gasLimit: gasToSpend(gas),
      value: 0,
    };


    const transferExtrinsic = await api.message.send(message, mainContractMetadata);

    const injector = await web3FromSource(account.meta.source);

    transferExtrinsic
      .signAndSend(
        account?.decodedAddress,
        { signer: injector.signer },
        ({ status }) => {
          if (status.isInBlock) {
            console.log(
              `Completed at block hash #${status.asInBlock.toString()}`
            );
            alert.success(`Block hash #${status.asInBlock.toString()}`);
          } else {
            console.log(`Current status: ${status.type}`);
            if (status.type === "Finalized") {
              alert.success(status.type);
            }
          }
        }
      )
      .catch((error: any) => {
        console.log(":( transaction failed", error);
      });

  }

  const setDefaultsNfts = async () => {
    if (!api) return;

    const stateResult1 = await api
      .programState
      .read({ 
        programId: MAIN_CONTRACT.PROGRAM_ID, 
        payload: { UserCanMintDefaultsNFTs: account?.decodedAddress ?? "0x0" } 
      }, mainContractMetadata);
    
    const stateFormated1: any = stateResult1.toJSON();
    const userCanMint: boolean = stateFormated1.userCanMintDefaultsNfts;
    
    setCanMint(userCanMint);

    if (!canMint) return;
 
    const stateResult2 = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { NFTsPurchasedByUser: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated2: any = stateResult2.toJSON();

    const mintedNfts: [number] = stateFormated2.purchasedNfts;

    const stateResult3 = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { DefaultsNFTS: null } }, mainContractMetadata);
    
    const stateFormated3: any = stateResult3.toJSON();
    
    const defaultsNfts: [any] = stateFormated3.defaultsNFTs;

    const defaultsNftsToMint = defaultsNfts?.filter((nft: any) => {
      const nftId: number = nft.saleId;
      return !(mintedNfts?.includes(nftId) ?? true);
    }) ?? [];

    setDefaultsNFTs(defaultsNftsToMint);
  };

  setDefaultsNfts();

  return (
    canMint ? (
    <div>
      {
        defaultsNFTs.map((nft: any) => 
          <Card 
            image={nft.media}
            title={nft.name}
            type={nft.description.toLowerCase()}
            value={nft.reference}
            price={6}
            key={nft.saleId}
          >
            <Button text="Mint" onClick={() => {
              const saleId = Number(nft.saleId.toString());
              mintDefaultNft(saleId);
            }} />
          </Card>
        )
      }
    </div>
    ) : (
      <div />
    )
  );
}

export { DefaultNfts };
