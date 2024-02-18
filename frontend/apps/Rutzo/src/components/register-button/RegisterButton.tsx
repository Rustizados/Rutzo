import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import {
  GearKeyring,
  ProgramMetadata,
} from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, seed } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { useState } from "react";
import { AccountsModal } from "../layout/header/account-info/accounts-modal";
import Spinner from 'react-bootstrap/Spinner';
import { ReactComponent as userSVG } from  '@/assets/images/icons/login.svg';

function RegisterButton({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const [userIsSigning, setUserIsSigning] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  // Function to register user
  const registerUser = async () => {
    if (!account || !api || !accounts) return;

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

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
    // Se genera el "issue" para crear el voucher para el usuario
    // En este caso, para el main contract

    setUserIsSigning(true);

    const voucherAlreadyExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

    if (voucherAlreadyExists) {
      console.log("Voucher already exists");
      await registerUser();
      return;
    }

    const mainContractVoucher = api.voucher.issue(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      13_000_000_000_000
      // 18_000_000_000_000
      // 10_000_000_000_000
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
    console.log("signer");
    if (!account || !accounts || !api) return;
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
      ? <Button text="Register" onClick={signer} className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl"/>
      : <Spinner animation="border" variant="success" />
  ) : (
    <>
      <Button icon={userSVG} text="Sign in" onClick={openModal} />
      {isModalOpen && <AccountsModal accounts={accounts} close={closeModal} />}
    </>
  );

}



export { RegisterButton };
