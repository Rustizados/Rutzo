import { decodeAddress, ProgramMetadata, VoucherIssued } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAccount, useAlert, useBalance, useBalanceFormat } from "@gear-js/react-hooks";
import { Card } from "../card/Card";
import { NFT_CONTRACT, MAIN_CONTRACT, ONE_TVARA_VALUE } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Button } from "@gear-js/ui";
import process from "process";
import { AnyJson, AnyNumber } from "@polkadot/types/types";
import { u128 } from "@polkadot/types";

export function NftsOnSale() {
  const { api, isApiReady } = useApi();
  const { account, accounts } = useAccount();
  const { balance } = useBalance(account?.address);
  const { getFormattedBalance } = useBalanceFormat();
  const alert = useAlert();
  const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
  const [nftsPrices, setNftsPrices] = useState<any>([])
  const formattedBalance = isApiReady && balance ? getFormattedBalance(balance) : undefined;

  const nftMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);
  const mainMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const butNft = async (tokenId: number, price: AnyNumber) => {
    if (!account || !api) {
      alert.error("Account not available to sign");
      return;
    }

    console.log("NFT QUE SE VA A COMPRAR: ", tokenId);
    

    // convert value into a valid value in Vara
    const nftParcialPrice = Number(price.toString());

    let finalPrice: AnyNumber;
    if (nftParcialPrice < 10) {
      finalPrice = (nftParcialPrice + 10) * ONE_TVARA_VALUE;
    } else {
      finalPrice = nftParcialPrice * ONE_TVARA_VALUE;
    }
  
    console.log("To mint: ", finalPrice.toString());
    

    if (Number(formattedBalance?.value)  < Number(price.toString())) {
      alert.error(`insufficient funds: ${formattedBalance?.value} < ${price.toString()}`);
      return;
    } 

    const localaccount = account?.address;
    const isVisibleAccount = accounts?.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      if (!account) {
        return;
      }

      const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

      if (!voucherExists) {
        alert.error("voucher does not exist!");
        return;
      }

      console.log("AVR SE HARA EL RESPECTIVO CALCULO DE GAS");
    
      const gas = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { BuyNFT: [tokenId] },
        finalPrice,
        false,
        mainMetadata
      );

      console.log("Gas spend: ", gasToSpend(gas));
      
      console.log("MANDANDO A COMPRAR EL NFT");

      const { signer } = await web3FromSource(account.meta.source);

      const transferExtrinsic = api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { BuyNFT: [tokenId] },
        gasLimit: gasToSpend(gas),
        value: finalPrice,
        prepaid: true,
        account: account.decodedAddress
      }, mainMetadata);

      const voucherTx = api.voucher.call({ SendMessage: transferExtrinsic });

      try {
        await voucherTx
        .signAndSend(
          account?.decodedAddress,
          { signer },
          ({ status, events }) => {
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
      } catch(error: any) {
        console.log(":( transaction failed", error);
      }

      console.log("TERMINADO EN LA COMPRA DEL NFT!");


      // const transferExtrinsic = await api.message.send(message, mainMetadata);

      // console.log("SE TERMINO DE CREAR EL EXTRINSIC PARA EL MENSAJE");
      

      // const injector = await web3FromSource(account.meta.source);

      // console.log("MANDANDO MENSAJEEEEE");
      
      // transferExtrinsic
      //   .signAndSend(
      //     account?.decodedAddress,
      //     { signer: injector.signer },
      //     ({ status }) => {
      //       if (status.isInBlock) {
      //         console.log(
      //           `Completed at block hash #${status.asInBlock.toString()}`
      //         );
      //         alert.success(`Block hash #${status.asInBlock.toString()}`);
      //       } else {
      //         console.log(`Current status: ${status.type}`);
      //         if (status.type === "Finalized") {
      //           alert.success(status.type);
      //         }
      //       }
      //     }
      //   )
      //   .catch((error: any) => {
      //     console.log(":( transaction failed", error);
      //   });
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
              butNft(nftId, nftPriceData.value as u128);
            }} />
          </Card>;
        })
      ) : (
        <h1>No NFTs</h1>
      )}
    </div>
  );
}