import { MAIN_CONTRACT } from "@/app/consts";
import { gasToSpend } from "@/app/utils";
import { ProgramMetadata } from "@gear-js/api";
import { useAlert, useApi, useAccount } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";

// El componente acepta dos parametros, uno un array de tres TokensId
// y el segundo es la opcion para poder jugar con el bot, esta siempre
// debe de quedar en falso por el momento.

interface ButtonToJoinInAGameProps {
    cardsId: [number, number, number];
    playWithBot: boolean;
}

export function ButtonToJoinInAGame({cardsId, playWithBot}: ButtonToJoinInAGameProps) {
    const alert = useAlert();
    const { api } = useApi();
    const { account } = useAccount();
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  // La funcion necesita 3 TokensId (de los n   fts del usuario) para poder funcionar,
  // y estos deben de pertenecer al usuario, en cualquier otro caso, el contrato
  // rechazara la peticion.
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
      { JoinGame: {
            cards_id: cardsId,
            play_with_bot: playWithBot
        }
      },
      0,
      false,
      mainContractMetadata
    );

    const { signer } = await web3FromSource(account.meta.source);

    const transferExtrinsic = api.message.send({
      destination: MAIN_CONTRACT.PROGRAM_ID,
      payload: { 
        JoinGame: {
            cards_id: cardsId,
            play_with_bot: playWithBot
        }
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
            TestJoiningGame
          </button>
    );
}