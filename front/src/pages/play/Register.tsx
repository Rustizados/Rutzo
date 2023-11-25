import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT } from "consts";

function Register({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();

  // Add your programID
  const programIDNFT = MAIN_CONTRACT.PROGRAM_ID; // "0x2cd2eefd93196e8adcb38eef6b40c476478fa9f0806f7a6c617fe02520381b1f"

  // Add your metadata.txt
  const meta = MAIN_CONTRACT.METADATA;

  const metadata = ProgramMetadata.from(meta);

  const message: any = {
    destination: programIDNFT, // programId
    payload: { Register }, // Add your data
    gasLimit: 2099819245,
    value: 0,
  };

  const signer = async () => {
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

      // const injector = await web3FromSource(accounts[0].meta.source);
      const injector = await web3FromSource(account.meta.source);

      transferExtrinsic
        .signAndSend(
          // accounts[0].address,
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

    const unsub = api.gearEvents.subscribeToGearEvent(
      "UserMessageSent",
      ({
        data: {
          message: { id, source, destination, payload, value },
        },
      }) => {
        console.log(`
        messageId: ${id.toHex()}
        source: ${source.toHex()}
        payload: ${payload.toHuman()}
        `);
      }
    );
  };

  return <Button text="Register" onClick={signer} className="alert" />;
}
export { Register };
