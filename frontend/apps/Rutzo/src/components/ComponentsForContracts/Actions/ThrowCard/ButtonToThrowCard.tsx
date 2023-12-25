import { MAIN_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { ProgramMetadata } from "@gear-js/api";
import { useAlert, useApi, useAccount } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";

// El componente acepta solo un parametro, que es el id
// del nft que anteriormente se escogio, aqui solo se procesa esto
// el backend se encarga de todo lo demas, solo se tiene que asegurar 
// de que se mandara una carta que se selecciono anteriormente para la partida
// si no, el contrato no hara nada. (En su caso, checar el estado)

interface ButtonToJoinInAGameProps {
    cardId: number
}

// Cuando se tire la carta, se tiene que checar el estado del contrato
// 

export function ButtonToThrowCard({cardId}: ButtonToJoinInAGameProps) {
    const alert = useAlert();
    const { api } = useApi();
    const { account } = useAccount();
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const joinGame = async () => {
    if (!api) return;

    if (!account) {
      alert.error("Account not available to sign");
      return;
    }

    const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

    if (!voucherExists) {
      alert.error("Voucher does not exist!");
      return;
    }

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { 
        ThrowCard: [cardId]
      },
      0,
      false,
      mainContractMetadata
    );

    const { signer } = await web3FromSource(account.meta.source);

    const transferExtrinsic = api.message.send({
      destination: MAIN_CONTRACT.PROGRAM_ID,
      payload: { 
        ThrowCard: [cardId]
    },
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
  }

    return (
        <button onClick={joinGame}>
            Testing Throwing card
          </button>
    );
}