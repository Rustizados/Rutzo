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

    console.log(nftContractMetadata.getTypeDef(17));

    const gasNftContract = await api.program.calculateGas.handle(
        account?.decodedAddress ?? "0x00",
        NFT_CONTRACT.PROGRAM_ID,
        { Approve: {
            transaction_id: 0,
            to: MAIN_CONTRACT.PROGRAM_ID,
            token_id: tokenId
        }},
        0,
        false,
        nftContractMetadata
    );

    console.log("Gas gastado para nft: ", gasToSpend(gasNftContract));
    console.log("Gas gastado para main contract: ", gasToSpend(gasMainContract));
    

    const messageMainContract: any = {
      destination: MAIN_CONTRACT.PROGRAM_ID, // programId
      payload: { PlayGame: [tokenId] }, // Add your data
      gasLimit: gasToSpend(gasMainContract),
      value: 0,
    };

    const messageNftContract: any = {
        destination: NFT_CONTRACT.PROGRAM_ID, // programId
        payload: { Approve: {
            transaction_id: 0,
            to: MAIN_CONTRACT.PROGRAM_ID,
            token_id: tokenId
        }}, // Add your data
        gasLimit: gasToSpend(gasNftContract),
        value: 0,
    }

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

      const transferExtrinsicNft = await api.message.send(messageNftContract, nftContractMetadata);

      try {
        await transferExtrinsicNft
        .signAndSend(
          account?.decodedAddress,
          { signer: injector.signer },
          ({ status }) => {
            if (status.isInBlock) {
              console.log(
                `Completed at block hash #${status.asInBlock.toString()}`
              );
              alert.success(`Block hash #${status.asInBlock.toString()}`);

              if (onJoiningGame) {
                onJoiningGame();
              }
            } else {
              console.log(`Current status: ${status.type}`);
              if (status.type === "Finalized") {
                console.log("SE TERMINO EL PROCESO DELNFT ----------------");
                
                alert.success(status.type);
              }
            }
          }
        )
      } catch(error) {
        console.log(":( transaction failed", error);
        onError();
        // return;
      }

      console.log("TERMINADO EL MENSAJE A NFT");
      

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
                
                alert.success(status.type);
              }
            }
          }
        )
      } catch(error) {
        console.log(":( transaction failed", error);
        onError();
      }

      console.log("TERMINADO EL MENSAJE AL MAIN CONTRACT");
      

    } else {
      alert.error("Account not available to sign");
    }
    console.log("PRESSED ");
    
  };

  return <Button text="Play" onClick={signer} />;
}
export { PlayButton };
