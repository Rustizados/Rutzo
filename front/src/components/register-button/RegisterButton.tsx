import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import {
  GearKeyring,
  ProgramMetadata,
  VoucherIssued,
} from "@gear-js/api";
import { Button } from "@gear-js/ui";
import { MAIN_CONTRACT, NFT_CONTRACT, ONE_TVARA_VALUE } from "consts";
import { gasToSpend } from "utils";
import { useState } from "react";

function RegisterButton({ onRegister }: any) {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  // Datos de cuenta del administrador donde se efectuaran los pagos en los contratos
  // de los nfts y del main contract
  const mnemonic =
    "strong orchard plastic arena pyramid lobster lonely rich stomach label clog rubber";
  const { seed } = GearKeyring.generateSeed(mnemonic);

  // Function to register user
  const registerUser = async () => {
    if (!account) return;

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { Register: null },
      0,
      false,
      mainContractMetadata
    );

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      // Create a message extrinsic

      const transferExtrinsic = await api.message.send({
        destination: MAIN_CONTRACT.PROGRAM_ID,
        payload: { Register: null },
        gasLimit: gasToSpend(gas),
        value: 0,
        prepaid: false,
        account: account.decodedAddress
      }, mainContractMetadata);

      const injector = await web3FromSource(account.meta.source);

      try {
        await transferExtrinsic
        .signAndSend(
          account?.decodedAddress,
          { signer: injector.signer },
          ({ status, events }) => {
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
      } catch(error: any) {
        console.log(":( transaction failed", error);
      }
    } else {
      alert.error("Account not available to sign");
    }

    console.log("TERMINO LA FUNCION PARA REGISTRAR AL USUARIO");
  }


  // Function to create voucher to main contract
  const setMainContractVoucher = async () => {

    // Se genera el "issue" para crear el voucher para el usuario
    // En este caso, para el main contract
    const mainContractVoucher = api.voucher.issue(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      12000000000000
      // 20000000000000
    );

    const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

    // Se firma el voucher con la cuenta del administrador para el main Contract

    try {
      await mainContractVoucher.extrinsic.signAndSend(
        keyring,
        async (event) => {
          console.log(event.toHuman());
          console.log("Main Voucher Firmado <=============");
        }
      );
    } catch (error: any) {
      console.error(`${error.name}: ${error.message}`);
    }

    console.log("TERMINO LA FUNCION PARA CREAR UN VOUCHER PARA EL USUARIO (MAIN CONTRACT) !!");
    
  }


  // Function to create voucher to nft contract
  const setNftContractVoucher = async () => {
    // Se genera el "issue" para crear el voucher para el usuario
    // En este caso, para el contrato de nfts
    const nftContractVoucher = api.voucher.issue(
      account?.decodedAddress ?? "0x00",
      NFT_CONTRACT.PROGRAM_ID,
      12000000000000
    );

    const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

    // Se firma el voucher con la cuenta del administrador para el contrato de nfts

    try {
      await nftContractVoucher.extrinsic.signAndSend(keyring, async (event) => {
        console.log(event.toHuman());
        console.log("NFT Voucher Firmado <==================");
      });
    } catch (error: any) {
      console.error(`${error.name}: ${error.message}`);
    }

    console.log("TERMINO LA FUNCION PARA CREAR UN VOUCHER PARA EL USUARIO (NFT CONTRACT) !!");
  }


  const signer = async () => {
    if (!account || !accounts || !api) return;
    console.log("REGISTRANDO AL USUARIO!!!");
    const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);
    console.log("VOUCHER EXISTS: ", voucherExists);
    
    await registerUser();
    console.log("SE TERMINO TOOOOODOO EL PROCESO, CREACION DE VOUCHERS Y REGISTRO ------------");
  };

  return <Button text="Register" onClick={signer} /> // <Button text="Register" onClick={signer} className="alert" />;
}
export { RegisterButton };

/*
Pasos a seguir para poder tener todo el tema de los vouchers:

// Se debe de obtener el mnemonic de la cuenta del administrador, o en su caso
// de la cuenta que pagaria el vouchers de los usuarios.

const mnemonic = 'income humble pizza dice almost punch salt state upset figure boss page';

// Se genera la semilla a partir del mnmonic usando la funcion "generateSeed" de GearKeyring

const { seed } = GearKeyring.generateSeed(mnemonic);

// Se obtiene el keyring para poder hacer las firmas con el "nombre" de la cuenta
// del administrador o cuenta que pagara los vouchers

const keyring = await GearKeyring.fromSeed(seed, 'accountTesting');

// Se firma el voucher, provocando que el usuario tenga un nuevo voucher de 10 tokens.

try {
  await extrinsic.signAndSend(keyring, (event) => {
    console.log("FIRMADO!!");
    
    console.log(event.toHuman());
  });
} catch (error: any) {
  console.error(`${error.name}: ${error.message}`);
}

// Funcion para checar si existe algun voucher para el usuario actual

const voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account.decodedAddress);

// Ejemplo de mensaje que se debe de hacer en caso de que exista algun voucher:

const transferExtrinsic = await api.message.send({
  destination: MAIN_CONTRACT.PROGRAM_ID,
  payload: { Register: null },
  gasLimit: gasToSpend(gas),
  value: 1000,
  prepaid: true,
  account: account.decodedAddress
}, mainContractMetadata);

// Y al ultimo se firma el mensaje ya pagado por el voucher

    try {
        console.log("FIRMANDO");
        
        await extrinsic.signAndSend(
        account?.decodedAddress,
        { signer: injector.signer },
        ({ status, events }) => {
          console.log("EVENTOS EXTRINSIC");
          events.forEach(({ event: { method, data } }) => {
            console.log(`Evento: ${method}: ${data}`);
          });
          
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
    } catch (error) {
      console.log(":( transaction failed", error);
    }

*/
