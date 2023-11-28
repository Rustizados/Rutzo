import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { GearKeyring, ProgramMetadata, VoucherIssued } from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, ONE_TVARA_VALUE } from "consts";
import { gasToSpend } from "utils";

function RegisterButton({ onRegister }: any) {
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

    if (!account || !accounts || !api) return;

    console.log("TESTING");
    
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

      console.log(gasToSpend(gas));


      const {extrinsic, voucherId} = api.voucher.issue(
        // account?.decodedAddress,
        "0x5c0202b5b3bb096aa3828849f8642fc1b77f693acff81eec799c396a1cfef86e",
        MAIN_CONTRACT.PROGRAM_ID,
        10000000000000 // 5 * ONE_TVARA_VALUE
      );

      console.log("Paso el issue");
      console.log("Voucher id: ", voucherId);
      
      
      // const testAccount: `0x${string}` = account?.decodedAddress;

      // extrinsic.signAndSend(testAccount, (events) => {
      //   const vouchetData: any = events.events.filter(({event: {method}}) => method === 'VoucherIssued');
      //   const voucherIssuedEvent = vouchetData  as VoucherIssued;
      //   console.log(voucherIssuedEvent.toJSON());
      // })

      // const transferExtrinsic = await api.message.send(message, metadata);

      










// PARA PODER MINTEAR AUTOMATICAMENTE DE LA CUENTA DEL ADMINISTRADOR 

      // const mnemonic = 'income humble pizza dice almost punch salt state upset figure boss page';

      // const { seed } = GearKeyring.generateSeed(mnemonic);

      // console.log("SEMILLA: ");
      // console.log(seed);
      

      // const keyring = await GearKeyring.fromSeed(seed, 'accountTesting');

      // console.log("KEYRING:");
      
      // console.log(keyring);

      // console.log("SE VA A FIRMAR EL VOUCHER");
      

      // try {
      //   await extrinsic.signAndSend(keyring, (event) => {
      //     console.log("FIRMADO!!");
          
      //     console.log(event.toHuman());
      //   });
      // } catch (error: any) {
      //   console.error(`${error.name}: ${error.message}`);
      // }




      // const voucherTx = api.voucher.call({ SendMessage: messageTx });
      // const voucherTx = api.voucher.









      // api.code.signAndSend(keyring, (events) => {
      //   events.forEach(({ event: { method, data } }) => {
      //     if (method === 'ExtrinsicFailed') {
      //       throw new Error(data.toString());
      //     } else if (method === 'CodeChanged') {
      //       console.log(data.toHuman());
      //     }
      //   });
      // });



      console.log("Mandando a firmar");
      
      // try {
      //     console.log("FIRMANDO");
          
      //    await extrinsic.signAndSend(
      //     account?.decodedAddress,
      //     { signer: injector.signer },
      //     ({ status, events }) => {
      //       console.log("EVENTOS EXTRINSIC");
      //       events.forEach(({ event: { method, data } }) => {
      //         console.log(`Evento: ${method}: ${data}`);
      //       });
            
      //       if (status.isInBlock) {
      //         console.log(
      //           `Completed at block hash #${status.asInBlock.toString()}`
      //         );
      //         alert.success(`Block hash #${status.asInBlock.toString()}`);

      //         if (onRegister) {
      //           onRegister();
      //         }
      //       } else {
      //         console.log(`Current status: ${status.type}`);
      //         if (status.type === "Finalized") {
      //           alert.success(status.type);
      //         }
      //       }
      //     }
      //   )
      // } catch (error) {
      //   console.log(":( transaction failed", error);
      // }

      console.log("Cuenta a la que se hara el voucher: ");
      console.log(account.decodedAddress);

      const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);
      console.log("El voucher existe?: ", voucherExists);



        const transferExtrinsic = await api.message.send({
          destination: MAIN_CONTRACT.PROGRAM_ID,
          payload: { Register: null },
          gasLimit: gasToSpend(gas),
          value: 0,
          prepaid: true,
          account: account.decodedAddress
        }, mainContractMetadata);

        

        const injector = await web3FromSource(account.meta.source);
      
        console.log("Ahora se mandara el mensaje sin necesidad de cobrarnos");

        transferExtrinsic.signAndSend(
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
export { RegisterButton };
