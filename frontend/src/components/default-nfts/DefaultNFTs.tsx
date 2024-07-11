import { gasToSpend } from "@/app/utils";
import { ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { web3FromSource } from "@polkadot/extension-dapp";
import { useApi, useAccount, useAlert, TemplateAlertOptions } from "@gear-js/react-hooks";
import { MAIN_CONTRACT } from "@/app/consts";
import { Card } from "../card/Card";
import { Button } from "@gear-js/ui";
import { SvgLoader } from "../loaders"; 
import useVoucherUtils from "@/hooks/useVoucherUtils";

interface DefaultNftsProos {
  onMinted?: any;
}

function DefaultNfts({ onMinted }: DefaultNftsProos) {
  const { api } = useApi();
  const { account } = useAccount();
  const [defaultsNFTs, setDefaultsNFTs] = useState<any[]>([]);
  const [canMint, setCanMint] = useState(true);
  const [minting, setMinting] = useState(false);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const {
    voucherExists,
    voucherExpired,
    renewVoucherOneHour,
    accountVoucherId,
    addTwoTokensToVoucher,
    voucherBalance
  } = useVoucherUtils();
  const alert = useAlert();

  const mintDefaultNft = async (nftId: number) => {
    if (!api || !account) return;

    try {
      const voucherAlreadyExists = await voucherExists(account.decodedAddress);

      if (!voucherAlreadyExists) {
        alert.error("Voucher does not exist!");
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
        account.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { MintCard: [nftId] },
        0,
        false,
        mainContractMetadata
      );

      const { signer } = await web3FromSource(account.meta.source);

      const transferExtrinsic = api.message.send(
        {
          destination: MAIN_CONTRACT.PROGRAM_ID,
          payload: { MintCard: [nftId] },
          gasLimit: gasToSpend(gas),
          value: 0,
          prepaid: true,
          account: account.decodedAddress,
        },
        mainContractMetadata
      );

      const voucherTx = api.voucher.call(voucherId, { SendMessage: transferExtrinsic });
      let alertLoaderId: any = null;

      await voucherTx.signAndSend(account.decodedAddress, { signer } , ({ status, events }: { status: any, events: any[] }) => {
        if (!alertLoaderId) {
          const alertOptions: TemplateAlertOptions = {
            title: "Rutzo action",
          };
          alertLoaderId = alert.loading("processing mint", alertOptions);
        }
        if (status.isInBlock) {
          console.log(`Completed at block hash #${status.asInBlock.toString()}`);
          alert.success(`Block hash #${status.asInBlock.toString()}`);
        } else {
          console.log(`Current status: ${status.type}`);
          if (status.isFinalized) {
            if (onMinted) {
              onMinted();
            }
            alert.remove(alertLoaderId);
            alert.success(status.type);
            setMinting(false);
          }
        }
      });
    } catch (error: any) {
      console.error("An error occurred:", error);
      alert.error("An unexpected error occurred. Please try again.");
      setMinting(false);
    }
  };

  const setDefaultsNfts = async () => {
    if (!api || !account) return;

    if (minting) {
      return;
    }

    const stateResult1 = await api.programState.read(
      {
        programId: MAIN_CONTRACT.PROGRAM_ID,
        payload: { UserCanMintDefaultsNFTs: account.decodedAddress ?? "0x0" },
      },
      mainContractMetadata
    );

    const stateFormated1: any = stateResult1.toJSON();
    const userCanMint: boolean = stateFormated1.userCanMintDefaultsNfts;

    setCanMint(userCanMint);

    if (!userCanMint) return;

    const stateResult2 = await api.programState.read(
      { programId: MAIN_CONTRACT.PROGRAM_ID, payload: { NFTsPurchasedByUser: account.decodedAddress ?? "0x0" } },
      mainContractMetadata
    );

    const stateFormated2: any = stateResult2.toJSON();
    const mintedNfts: number[] = stateFormated2.purchasedNfts;

    const stateResult3 = await api.programState.read(
      { programId: MAIN_CONTRACT.PROGRAM_ID, payload: { DefaultsNFTS: null } },
      mainContractMetadata
    );

    const stateFormated3: any = stateResult3.toJSON();
    const defaultsNfts: any[] = stateFormated3.defaultsNFTs;

    const defaultsNftsToMint = defaultsNfts?.filter((nft: any) => {
      const nftId: number = nft.saleId;
      return !(mintedNfts?.includes(nftId) ?? true);
    }) ?? [];

    setDefaultsNFTs(defaultsNftsToMint);
  };

  setDefaultsNfts();

  return canMint ? (
    <div>
      {defaultsNFTs.map((nft: any) => (
        <Card
          image={nft.media}
          title={nft.name}
          type={nft.description.toLowerCase()}
          value={nft.reference}
          price={nft.price}
          key={nft.saleId}
          children={
            !minting ? (
              <Button
                text="Mint"
                className="bg-green-500"
                onClick={() => {
                  const saleId = Number(nft.saleId.toString());
                  setMinting(true);
                  mintDefaultNft(saleId);
                }}
              />
            ) : (
              <SvgLoader />
            )
          }
        />
      ))}
    </div>
  ) : (
    <div />
  );
}

export { DefaultNfts };
