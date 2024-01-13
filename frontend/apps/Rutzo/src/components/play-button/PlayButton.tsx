import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { ProgramMetadata } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";

function PlayButton({ onJoiningGame, onPressed=(x: boolean)=>{}, tokenId }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

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

      onPressed(true);

      console.log("SE JUGARA CON EL NFT: ", tokenId);
      

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
                console.log("Se termino el proceso de main contract =========");
                alert.remove(alertLoaderId);
                alert.success(status.type);
                onPressed(false);
                onJoiningGame();
              }
            }
          }
        )
      } catch(error: any) {
        console.log(":( transaction failed", error);
        onPressed(false);
      }
    } else {
      alert.error("Account not available to sign");
    }
  };

  return <Button text="Play" onClick={signer} />;
}
export { PlayButton };
