import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, NFT_CONTRACT } from "consts";
import { gasToSpend } from "utils";

function PlayButton({ onJoiningGame, onError=()=>{}, tokenId }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const signer = async () => {

    if (!account || !accounts || !api) return;

    const gasMainContract = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { PlayGame: [tokenId] },
      0,
      false,
      mainContractMetadata
    );

    console.log("Gas gastado para main contract: ", gasToSpend(gasMainContract));
    console.log("NFT QUE SE USARA: ", tokenId);
    

    const messageMainContract: any = {
      destination: MAIN_CONTRACT.PROGRAM_ID, // programId
      payload: { PlayGame: [tokenId] }, // Add your data
      gasLimit: gasToSpend(gasMainContract),
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

      const injector = await web3FromSource(account.meta.source);

      const transferExtrinsicMain = await api.message.send(messageMainContract, mainContractMetadata);

      try {
        await transferExtrinsicMain
        .signAndSend(
          account?.decodedAddress,
          { signer: injector.signer },
          ({ status }) => {
            if (status.isInBlock) {
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                console.log("Se termino el proceso de main contract =========");
                onJoiningGame();
                alert.success(status.type);
              }
            }
          }
        )
      } catch(error) {
        console.log(":( transaction failed", error);
        onError();
      }

    } else {
      alert.error("Account not available to sign");
    }
  };

  return <Button text="Play" onClick={signer} />;
}
export { PlayButton };
