import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";

function PlayButton({ onJoiningGame, onPressed=()=>{}, tokenId }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

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

      const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

      if (!voucherExists) {
        alert.error("voucher does not exist!");
        return;
      }

      const gasMainContract = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        { PlayGame: [tokenId] },
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

      const voucherTx = api.voucher.call({ SendMessage: transferExtrinsic });

      try {
        await voucherTx
        .signAndSend(
          account?.decodedAddress,
          { signer },
          ({ status }) => {
            if (status.isInBlock) {
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);
              if (onPressed) {
                onPressed();
              }
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                onJoiningGame();
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
  };

  return <Button text="Play" onClick={signer} />;
}
export { PlayButton };
