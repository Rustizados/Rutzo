import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import {
  GearKeyring,
  ProgramMetadata,
} from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { useState } from "react";

function RegisterButton({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const [voucherCreated, setVoucherCreated] = useState(false);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  // Datos de cuenta del administrador donde se efectuaran los pagos en los contratos
  // de los nfts y del main contract
  const mnemonic =
    "strong orchard plastic arena pyramid lobster lonely rich stomach label clog rubber";
  const { seed } = GearKeyring.generateSeed(mnemonic);

  // Function to register user
  const registerUser = async () => {
    if (!account || !api || !accounts) return;

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { Register: null },
      0,
      true,
      mainContractMetadata
    );

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      const { signer } = await web3FromSource(account.meta.source);

      const transferExtrinsic = api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { Register: null },
        gasLimit: gasToSpend(gas),
        value: 0,
        prepaid: true,
        account: account.decodedAddress
      }, mainContractMetadata);

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

              if (onRegister) {
                onRegister();
              }
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
    } else {
      alert.error("Account not available to sign");
    }
  }


  // Function to create voucher to main contract
  const setMainContractVoucher = async () => {
    if (!api) return;
    // Se genera el "issue" para crear el voucher para el usuario
    // En este caso, para el main contract
    const mainContractVoucher = api.voucher.issue(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      13000000000000
      // 20000000000000
    );

    const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

    // Se firma el voucher con la cuenta del administrador para el main Contract

    try {
      await mainContractVoucher.extrinsic.signAndSend(
        keyring,
        async (event) => {
          console.log(event.toHuman());
        }
      );
    } catch (error: any) {
      console.error(`${error.name}: ${error.message}`);
    }

    let voucherExists = false;
    /* eslint-disable no-await-in-loop */
    while (!voucherExists) {
      voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account?.decodedAddress ?? "0x00");
    }

    await registerUser();
  }



  const signer = async () => {
    if (!account || !accounts || !api) return;
    await setMainContractVoucher();
  };

  return <Button text="Register" onClick={signer} /> // <Button text="Register" onClick={signer} className="alert" />;
}



export { RegisterButton };