import { useAccount, useApi, useAlert, useBalanceFormat } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, seed } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { useState } from "react";
import { AccountsModal } from "../layout/header/account-info/accounts-modal";
import { ReactComponent as userSVG } from  '@/assets/images/icons/login.svg';
import { SvgLoader } from "../loaders";

import useVoucherUtils from "@/hooks/useVoucherUtils";

function RegisterButton({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const [userIsSigning, setUserIsSigning] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const { 
    createNewVoucher, 
    voucherExpired,
    renewVoucherOneHour,
    voucherExists,
    accountVoucherId
  } = useVoucherUtils();


  // const voucherBalance = useVoucherBalanceDeprecated(MAIN_CONTRACT.PROGRAM_ID, "0x523dda1e177405c8d2a17b9fdb61e757f8b7a9fe01c281ff1329f5a38721a755");

  const { getFormattedBalanceValue } = useBalanceFormat();


  // Function to register user
  const registerUser = async (voucherId: string) => {
    if (!account || !api || !accounts) return;

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {

      const { signer } = await web3FromSource(account.meta.source);

      const gas = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { Register: null },
        0,
        true,
        mainContractMetadata
      );

      const transferExtrinsic = api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { Register: null },
        gasLimit: gasToSpend(gas),
        value: 0,
        prepaid: true,
        account: account.decodedAddress
      }, mainContractMetadata);

      const voucherTx = api.voucher.call(voucherId, { SendMessage: transferExtrinsic });

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

              if (onRegister) {
                onRegister();
              }
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                alert.success(status.type);
                setUserIsSigning(false);
              }
            }
          }
        )
      } catch(error: any) {
        console.log(":( transaction failed", error);
        setUserIsSigning(false);
      }
    } else {
      alert.error("Account not available to sign");
      setUserIsSigning(false);
    }
  }


  // Function to create voucher to main contract
  const setMainContractVoucher = async () => {
    if (!api || !account) return;

    if (await voucherExists()) {
      console.log("Voucher already exists");

      const voucherId = await accountVoucherId();

      if (await voucherExpired(voucherId)) {
        console.log("Voucher expired");
        await renewVoucherOneHour(voucherId);
      }

      await registerUser(voucherId);

      return;
    }

    console.log("Voucher does not exists");

    try {
      const voucherId = await createNewVoucher();
      await registerUser(voucherId);
    } catch (error) {
      console.log("Error creating voucher");
    }
  }

  const signer = async () => {
    console.log("signer");
    if (!account || !accounts || !api) return;
    setUserIsSigning(true);
    await setMainContractVoucher();
  };

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  return account ? (
    !userIsSigning
      ? <Button text="Register" onClick={signer} />
      : <SvgLoader />
  ) : (
    <>
      <Button icon={userSVG} text="Sign in" onClick={openModal} />
      {isModalOpen && <AccountsModal accounts={accounts} close={closeModal} />}
    </>
  );

}

export { RegisterButton };








// const mainContractVoucher = await api.voucher.issue(
//       account?.decodedAddress ?? "0x00",
//       11_000_000_000_000, // 11 TVaras
//       1_200, // An hour in blocks
//       [MAIN_CONTRACT.PROGRAM_ID]
//     );

//     console.log("Se hizo el issue del voucher");
//     console.log(`Con id: ${mainContractVoucher.voucherId}`);
    

//     // const mainContractVoucher = api.voucher.issue(
//     //   account?.decodedAddress ?? "0x00",
//     //   MAIN_CONTRACT.PROGRAM_ID,
//     //   13_000_000_000_000
//     //   // 18_000_000_000_000
//     //   // 10_000_000_000_000
//     // );

//     const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

//     // Se firma el voucher con la cuenta del administrador para el main Contract

//     try {
//       console.log("Se firmara el voucher!");
//       await mainContractVoucher.extrinsic.signAndSend(
//         keyring,
//         async (event) => {
//           console.log(event.toHuman()); 
//           const extrinsicJSON: any = event.toHuman();
//           if (extrinsicJSON && extrinsicJSON.status !== "Ready") {
//             const objectKey = Object.keys(extrinsicJSON.status)[0];
//             if (objectKey === "Finalized") {
//               console.log("Ya se finalizo de crear el voucher");
//               await registerUser(mainContractVoucher.voucherId);
//             }
//           }
//         }
//       );
//     } catch (error: any) {
//       console.error(`${error.name}: ${error.message}`);
//     }


//     // return;
  
//     // /* eslint-disable no-await-in-loop */
//     // while (!voucherExists) {
//     //   // voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account?.decodedAddress ?? "0x00");
//     // }

//     // console.log("VOUCHER EXISTS");


//     // await registerUser();
