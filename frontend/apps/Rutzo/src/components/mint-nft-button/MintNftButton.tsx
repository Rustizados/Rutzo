import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT } from "@/app/consts";
import { HumanGasCalculated } from "@/types";
import { AnyJson } from "@polkadot/types/types";
import { gasToSpend } from "@/app/utils";

interface MintNftButtonProps {
  onMint: any;
  defaultNftId: number
}

function MintNftButton({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();

  // Add your programID
  const programIDNFT = MAIN_CONTRACT.PROGRAM_ID;

  // Add your metadata.txt
  const meta = MAIN_CONTRACT.METADATA;

  const metadata = ProgramMetadata.from(meta);

  const signer = async () => {
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

    if (!api || !accounts) return;

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { Register: null },
      0,
      false,
      mainContractMetadata
    );


    const message: any = {
      destination: programIDNFT, // programId
      payload: { Register: null }, // Add your data
      gasLimit: gasToSpend(gas),
      value: 0,
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

      const transferExtrinsic = await api.message.send(message, metadata);

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
        .catch((error: any) => {
          console.log(":( transaction failed", error);
        });
    } else {
      alert.error("Account not available to sign");
    }
  };

  return <Button text="Register" onClick={signer} className="alert" />;
}
export { MintNftButton };
