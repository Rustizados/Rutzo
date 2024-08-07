import { ProgramMetadata, GearKeyring } from "@gear-js/api";
import { useState, useEffect } from "react";
import { useApi, useAccount, useAlert, useBalance, useBalanceFormat, useVoucher } from "@gear-js/react-hooks";
import { Card } from "../card/Card";
import { NFT_CONTRACT, MAIN_CONTRACT, ONE_TVARA_VALUE, VOUCHER_MIN_LIMIT, seed } from "@/app/consts";
import { gasToSpend, sleepReact } from "@/app/utils";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Button } from "@gear-js/ui";
import { AnyNumber } from "@polkadot/types/types";
import { u128 } from "@polkadot/types";
import { SvgLoader } from "../loaders"; 
import useVoucherUtils from "@/hooks/useVoucherUtils";

interface DefaultNftsProos {
  onSaled?: any;
  type: string;
}

export function NftsOnSale({onSaled, type}: DefaultNftsProos) {
  //const { isVoucherExists, voucherBalance } = useVoucher(MAIN_CONTRACT.PROGRAM_ID);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { api, isApiReady } = useApi();
  const { account, accounts } = useAccount();
  const { balance } = useBalance(account?.address);
  const { getFormattedBalance } = useBalanceFormat();
  const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
  const [nftsPrices, setNftsPrices] = useState<any>([]);
  const [buyingNFT, setBuyingNFT] = useState(false);
  const formattedBalance = isApiReady && balance ? getFormattedBalance(balance) : undefined;
  const alert = useAlert();
  const { 
    voucherExists,
    voucherExpired,
    renewVoucherOneHour,
    accountVoucherId,
    addTwoTokensToVoucher,
    voucherBalance
  } = useVoucherUtils();

  const nftMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);
  const mainMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  

  const butNft = async (tokenId: number, price: AnyNumber) => {
    if (!account || !api) {
      alert.error("Account not available to sign");
      return;
    }

    // convert value into a valid value in Vara
    const nftParcialPrice = Number(price.toString());

    let finalPrice: AnyNumber;
    if (nftParcialPrice < 10) {
      finalPrice = (nftParcialPrice + 10) * ONE_TVARA_VALUE;
    } else {
      finalPrice = nftParcialPrice * ONE_TVARA_VALUE;
    }

    if (Number(formattedBalance?.value)  < Number(price.toString())) {
      alert.error(`insufficient funds: ${formattedBalance?.value} < ${price.toString()}`);
      return;
    } 

    const localaccount = account?.address;
    const isVisibleAccount = accounts?.some(
      (visibleAccount: { address: string }) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      if (!account) {
        return;
      }

      // const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);
      const voucherAlreadyExists = await voucherExists(account.decodedAddress);

      if (!voucherAlreadyExists) {
        alert.error("voucher does not exist!");
        return;
      }

      const voucherId = await accountVoucherId(account.decodedAddress);

      if (await voucherExpired(voucherId)) {
        console.log("Voucher expired");
        await renewVoucherOneHour(voucherId);
      }

      const accountVoucherBalance = await voucherBalance(voucherId);

      if (accountVoucherBalance < 11) {
        console.log("Voucher does not have enough tokens");
        await addTwoTokensToVoucher(voucherId);
      }

      const gas = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { BuyNFT: [tokenId] },
        finalPrice,
        false,
        mainMetadata
      );

      console.log("Gas spend: ", gasToSpend(gas));

      const { signer } = await web3FromSource(account.meta.source);

      const transferExtrinsic = api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { BuyNFT: [tokenId] },
        gasLimit: gasToSpend(gas),
        value: finalPrice,
        prepaid: true,
        account: account.decodedAddress
      }, mainMetadata);

      const voucherTx = api.voucher.call(voucherId, { SendMessage: transferExtrinsic });

      let alertLoaderId: any = null;

      try {
        await voucherTx
        .signAndSend(
          account?.decodedAddress,
          { signer },
            ({ status, events }: { status: any, events: any }) => {
            if (!alertLoaderId) {
              alertLoaderId = alert.loading("processing purchase");
            }
            if (status.isInBlock) {
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                if (onSaled) {
                  onSaled();
                }
                alert.remove(alertLoaderId);
                alert.success(status.type);
                setData();
                setBuyingNFT(false);
              }
            }
          }
        )
      } catch(error: any) {
        console.log(":( transaction failed", error);
        if (alertLoaderId) alert.remove(alertLoaderId);
        setBuyingNFT(false);
      }
    } else {
      alert.error("Account not available to sign");
    }
  }

  const setData = async () => {
    if (!api) return;

    if (buyingNFT) return;

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

    if (nftsOnSaleFormated.length !== tokensForOwner.length) {
      setData();
      return;
    }

    setNftsPrices(nftsOnSaleFormated);
    setTokensForOwnerState(tokensForOwner);
  }

  setData();

  return (
    <div>
  {tokensForOwnerState.length > 0 ? (
    <>
      {tokensForOwnerState.map((element: any) => {
        const [nftId, elemento] = element;

        const nftPriceData = nftsPrices.find((nftPrice: any) => nftId === nftPrice.tokenId);

        return (
          elemento.description.toLowerCase() === type || type === "all" ? (
            <Card 
              image={elemento.media}
              title={elemento.name}
              type={elemento.description.toLowerCase()}
              value={elemento.reference}
              price={nftPriceData.value}
              key={nftId}
              children={
                !buyingNFT ? (
                  <Button text={`$${nftPriceData.value} TVara`} onClick={() => {
                    setBuyingNFT(true);
                    butNft(nftId, nftPriceData.value as u128);
                  }} />
                ) : (
                  // <Spinner animation="border" variant="success" />
                  <SvgLoader />
                )
              }
            />
          ) : null
        );
      })}
      {tokensForOwnerState.every((element: any) => {
        const [nftId, elemento] = element;
        return elemento.description.toLowerCase() !== type && type !== "all";
      }) && (
        <p className="text-xl">
          There are no NFTs of this type
        </p>
      )}
    </>
  ) : (
    <h1>No NFTs</h1>
  )}
</div>


  );
}
