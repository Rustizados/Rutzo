import {
  useAccount,
  useApi,
  useAlert,
  useBalanceFormat,
} from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { useState } from "react";
import { AccountsModal } from "../layout/header/account-info/accounts-modal";
import { ReactComponent as userSVG } from "@/assets/images/icons/login.svg";
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
    accountVoucherId,
  } = useVoucherUtils();

  const registerUser = async (voucherId: string) => {
    if (!account || !api || !accounts) {
      console.error('Account, API o cuentas no disponibles');
      return;
    }

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      const { signer } = await web3FromSource(account.meta.source);

      const gas = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { Register: account.decodedAddress },
        0,
        true,
        mainContractMetadata
      );

      console.log('Calculated Gas:', gas);

      const transferExtrinsic = await api.message.send(
        {
          destination: MAIN_CONTRACT.PROGRAM_ID,
          payload: { Register: account.decodedAddress  },
          gasLimit: gasToSpend(gas),
          value: 0,
          prepaid: true,
          account: account.decodedAddress,
        },
        mainContractMetadata
      );

      console.log('Transfer Extrinsic:', transferExtrinsic);

      const voucherTx = api.voucher.call(voucherId, {
        SendMessage: transferExtrinsic,
      });

      try {
        await voucherTx.signAndSend(
          account?.decodedAddress,
          { signer },
          ({ status, events }) => {
            console.log(`Current status: ${status.type}`);
            if (status.isInBlock) {
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);

              onRegister();
            } else if (status.type === "Finalized") {
              alert.success(status.type);
              setUserIsSigning(false);
            }
          }
        );
      } catch (error: any) {
        console.log(":( transaction failed", error);
        setUserIsSigning(false);
      }
    } else {
      alert.error("Account not available to sign");
      setUserIsSigning(false);
    }
  };

  const setMainContractVoucher = async () => {
    if (!api || !account) {
      console.error('API o cuenta no disponible');
      return;
    }

    if (await voucherExists(account.decodedAddress)) {
      console.log("Voucher already exists");

      const voucherId = await accountVoucherId(account.decodedAddress);

      if (await voucherExpired(voucherId)) {
        console.log("Voucher expired");
        await renewVoucherOneHour(voucherId);
      }

      await registerUser(voucherId);
      return;
    }

    console.log("Voucher does not exist");

    try {
      const voucherId = await createNewVoucher(account.decodedAddress);
      await registerUser(voucherId);
    } catch (error) {
      console.log("Error creating voucher");
    }
  };

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
    !userIsSigning ? (
      <Button
        text="Register"
        onClick={signer}
        className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl"
      />
    ) : (
      <SvgLoader />
    )
  ) : (
    <>
      <Button
        icon={userSVG}
        text="Sign in"
        onClick={openModal}
        className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl"
      />
      {isModalOpen && <AccountsModal accounts={accounts} close={closeModal} />}
    </>
  );
}

export { RegisterButton };