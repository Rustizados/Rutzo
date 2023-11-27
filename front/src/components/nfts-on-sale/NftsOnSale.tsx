import { decodeAddress, ProgramMetadata, VoucherIssued } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAccount, useAlert } from "@gear-js/react-hooks";
import { Card } from "components/card/Card";
import { NFT_CONTRACT, MAIN_CONTRACT } from "consts";
import { gasToSpend } from "utils";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Button } from "@gear-js/ui";
import process from "process";
import { AnyJson, AnyNumber } from "@polkadot/types/types";
import { u128 } from "@polkadot/types";

export function NftsOnSale() {
  const { api } = useApi();
  const { account, accounts } = useAccount();
  const alert = useAlert();
  const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
  const [nftsPrices, setNftsPrices] = useState<any>([])

  const nftMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);
  const mainMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const butNft = async (tokenId: number, price: AnyNumber) => {
    if (!account) {
      alert.error("Account not available to sign");
      return;
    }

    if (Number(account.balance.value)  < Number(price.toString())) {
      alert.error(`insufficient funds: ${account.balance.value} < ${price.toString()}`);
      return;
    }
    
    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { BuyNFT: [tokenId] },
      0,
      false,
      mainMetadata
    );

    const message: any = {
      destination: MAIN_CONTRACT.PROGRAM_ID, // programId
      payload: { BuyNFT: [tokenId] }, // Add your data
      gasLimit: gasToSpend(gas),
      value: price,  // Aqui es donde pasa el error, se manda el valor de 3
                     // checando el valor de este en el estado, pero, no permite 
                     // mandar menos de 10 TVaras
    };

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      if (!account) {
        return;
      }

      // const transferExtrinsic1 = await api.message.send({
      //   destination: MAIN_CONTRACT.PROGRAM_ID,
      //   payload: { Register: null },
      //   gasLimit: gasToSpend(gas),
      //   value: 0,
      //   prepaid: true,
      //   account: account.decodedAddress
      // }, mainMetadata);


      const transferExtrinsic = await api.message.send(message, mainMetadata);

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
    } else {
      alert.error("Account not available to sign");
    }
  }

  const setData = async () => {
    if (!api) return;



    const stateNft = await api
      .programState
      .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: MAIN_CONTRACT.PROGRAM_ID } }, nftMetadata);
    const nftStateFormated: any = stateNft.toJSON();
    
    const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

    const mainState = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { NFTsOnSale: null } }, mainMetadata);
    const mainStateFormated: any = mainState.toJSON();
    
    const nftsOnSaleFormated: [any] = mainStateFormated.nfTsOnSale ?? [];

    setNftsPrices(nftsOnSaleFormated);
    setTokensForOwnerState(tokensForOwner);
  }

  setData();

  return (
    <div>
      {tokensForOwnerState.length > 0 ? (
        tokensForOwnerState.map((element: any) => {
          const [nftId, elemento] = element;
          const nftPriceData = nftsPrices.find((nftPrice: any) => nftId === nftPrice.tokenId);
          return <Card 
            image={elemento.media}
            title={elemento.name}
            type={elemento.description.toLowerCase()}
            value={elemento.reference}
            price={nftPriceData.value}
            key={nftId}
          >
            <Button text={`buy for ${nftPriceData.value} TVara`} onClick={() => {
              butNft(nftPriceData.nftPriceData, nftPriceData.value as u128);
            }} />
          </Card>;
        })
      ) : (
        <h1>No NFTs</h1>
      )}
    </div>
  );
}