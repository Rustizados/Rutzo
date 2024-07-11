import { useAccount, useApi, useAlert, useVoucher, useBalanceFormat } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata, GearKeyring } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, VOUCHER_MIN_LIMIT, seed } from "@/app/consts";
import { gasToSpend, sleepReact } from "@/app/utils";
import { useState } from "react";
import { SvgLoader } from "../loaders";
import useVoucherUtils from "@/hooks/useVoucherUtils";

function PlayButton({ onJoiningGame, onPressed=(x: boolean)=>{}, tokenId }: any) {
  // const { isVoucherExists, voucherBalance } = useVoucher(MAIN_CONTRACT.PROGRAM_ID);
  // const { getFormattedBalanceValue } = useBalanceFormat();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const [loadingSignature, setLoadingSignature] = useState(false);
  const alert = useAlert();
  const { 
    voucherExists,
    voucherExpired,
    renewVoucherOneHour,
    accountVoucherId,
    addTwoTokensToVoucher,
    voucherBalance
  } = useVoucherUtils();


  const signer = async () => {
    if (!account || !accounts || !api) return;

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      if (!account) {
        return;
      }

      // const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

      const voucherAlreadyExists = await voucherExists(account.decodedAddress);

      if (!voucherAlreadyExists) {
        alert.error("Voucher does not exist!");
        return;
      }

      setLoadingSignature(true);
      onPressed(true);

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


      // ahi
      // estado de contrato
      //

      const gasMainContract = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { PlayGame: [tokenId]},
        0,
        false,
        mainContractMetadata
      );

      const { signer } = await web3FromSource(account.meta.source);

      const transferExtrinsic = api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { PlayGame: [tokenId] },
        gasLimit: gasToSpend(gasMainContract),
        value: 0,
        prepaid: true,
        account: account.decodedAddress
      }, mainContractMetadata);

      const voucherTx = api.voucher.call(voucherId, { SendMessage: transferExtrinsic });
      let alertLoaderId: any = null;

      try {
        await voucherTx
        .signAndSend(
          account?.decodedAddress,
          { signer },
          ({ status }) => {
            if (!alertLoaderId) {
              alertLoaderId = alert.loading("preparing game");
            }
            if (status.isInBlock) {
              onPressed(true);
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                alert.remove(alertLoaderId);
                alert.success(status.type);
                setLoadingSignature(false);
                onPressed(false);
                onJoiningGame();
              }
            }
          }
        )
      } catch(error: any) {
        console.log(":( transaction failed", error);
        onPressed(false);
        setLoadingSignature(false);
      }
    } else {
      alert.error("Account not available to sign");
    }
  };

  // return <Button text="Play"  onClick={signer} />;

  return !loadingSignature
    ? <Button text="Play" onClick={signer} color="gradient" />
    : <SvgLoader /> //<Spinner animation="border" variant="success" />;
}
export { PlayButton };





// if (isVoucherExists && voucherBalance) {
      //   const voucherTotalBalance = Number(getFormattedBalanceValue(voucherBalance.toString()).toFixed());
      //   if (voucherTotalBalance < VOUCHER_MIN_LIMIT) {
      //     const addingTVarasAlertId = alert.loading("Adding TVaras to the voucher");
      //     const mainContractVoucher = api.voucher.issue(
      //       account?.decodedAddress ?? "0x00",
      //       MAIN_CONTRACT.PROGRAM_ID,
      //       2_000_000_000_000
      //     );
      //     const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");
      //     let addedVarasToVoucher = false;
      //     try {
      //       await mainContractVoucher.extrinsic.signAndSend(
      //         keyring,
      //         async (event) => {
      //           const eventData = event.toHuman();
      //           const { status }: any = eventData;
      //           if (Object.keys(status)[0] === "Finalized") addedVarasToVoucher = true;
      //         }
      //       );
      //     } catch (error: any) {
      //       console.error(`${error.name}: ${error.message}`);
      //       return
      //     }
      //     /* eslint-disable no-await-in-loop */
      //     while (!addedVarasToVoucher) {
      //       await sleepReact(500);
      //     }
      //     alert.remove(addingTVarasAlertId);
      //     alert.success("Added TVaras");
      //   }
      // }
