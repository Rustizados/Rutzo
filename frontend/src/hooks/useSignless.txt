import { GearKeyring, IUpdateVoucherParams, decodeAddress, HexString, CreateType, ProgramMetadata } from '@gear-js/api';
import { useAccount, useApi, useAlert, useSendMessage, SendMessageOptions } from '@gear-js/react-hooks';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { generatePassword, gasToSpend, sleepReact } from '@/app/utils'; 
import { ENABLED_ACTIONS, MAIN_CONTRACT, SIGNLESS_STORAGE_KEY } from '@/app/consts'; 
import { Keyring } from '@polkadot/api';
import useVoucherUtils from './useVoucherUtils';
import { useState } from 'react';
import { AnyJson } from '@polkadot/types/types';

type Session = {
    key: HexString;
    allowedActions: string[];
};

type Payload = Record<string, Record<string, AnyJson>>;

export default function useSignlessUtils() {
    const { api } = useApi();
    const { account } = useAccount();
    const [pair, setPair] = useState<KeyringPair | undefined>();
    const {
        createNewVoucher,
        voucherExists
    } = useVoucherUtils();
    const sendMessage = useSendMessage(
        MAIN_CONTRACT.PROGRAM_ID, 
        ProgramMetadata.from(MAIN_CONTRACT.METADATA), 
        { disableAlerts: true, pair }
    );

    const getSinglessPayload = (sessionForAccount: HexString | null | undefined, payload?: Payload) => {
        if (payload) {
            const [entry] = Object.entries(payload);
            const [key, value] = entry;
        
            return { ...payload, [key]: { ...value, sessionForAccount } };
        }

        return {
            "user_address": sessionForAccount
        }
        
    };

    const signlessAccountsFromLocalStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage; 

    const signlessActualAccount = (): string | undefined => {
        return account ? signlessAccountsFromLocalStorage()[account.address] : undefined;
    };

    const saveAccountPairToLocalStorage = (signlessJSONAccount: KeyringPair$Json | undefined) => {
        if (!account) throw new Error('No account address');
    
        const storage = { 
            ...signlessAccountsFromLocalStorage(), 
            [account.address]: signlessJSONAccount 
        };
    
        localStorage.setItem(SIGNLESS_STORAGE_KEY, JSON.stringify(storage));
    };

    const saveAccountSignlessPairInLocalStorage = (address: Keyring, password: string) => {
        if (!account) throw new Error('No account address');

        const signlessJSONAccount = address.toJson(password);

        saveAccountPairToLocalStorage(signlessJSONAccount);
    };

    const deleteAccountFromActualAccount = () => {
        if (!account) throw new Error('No account address');

        saveAccountPairToLocalStorage(undefined);
    };

    const createNewPairAddress = async (): Promise<KeyringPair> => {
        return new Promise(async (resolve, reject) => {
            try {
                const newPair = await GearKeyring.create('signlessPair');
                resolve(newPair.keyring as KeyringPair);
            } catch (e) {
                console.log("Error creating new account pair!");
                reject(e);
            }
        });
    };

    const createAndSaveAccountNewPair = async (
        password?: string,
        onMessageInBlock?: any,
        onMessageSendSuccessfull?: any,
        onMessageErrorWhileSending?: any
    ): Promise<[HexString, HexString]> => {
        if (!account) throw new Error('No account address');

        return new Promise(async (resolve, reject) => {
            const newKeyringPair = await createNewPairAddress();

            setPair(newKeyringPair);

            password = !password ? generatePassword() : password;

            // Primero se tiene que crear una sesion en el contrato
            /*
                Primero se tiene que crear la sesion con el tipo de dato Session,
                en el backend se tiene que crear este mismo tipo, y se le manda la cuenta
                signless, en el contrato es donde se tiene que controlar esta cuenta,
                se agrega junto con instrucciones que se indiquen, este mandara un delayed
                message, y borrara el mensaje cuando le llñegue este mensaje.
            */

            const newSignlessAccount = newKeyringPair.toJson(password);
            const signlessAddress = decodeAddress(newKeyringPair.address);

            console.log("Coded address:");
            console.log(newKeyringPair.address);
            console.log("Decoded address:"); 
            console.log(signlessAddress);

            // Seguido se crea un voucher para la sesion
            const voucherId = await createNewVoucher(
                signlessAddress
            );

            // Aqui se debe de manejar todo a la vez, para evitar errores en el contrato.
            // Como se tiene previsto, es que el usuario mande un mensaje inicial para 
            // poder guardar la "session" en el contrato (la sesion se maneja nivel contrato)
            // y aparte, crear el voucher para esta cuenta signless.
            // Sin embargo, esto no es aplicable para cuentas que no tienen tokens, entonces
            // la logica cambia un poco, tanto en el frontend como en el contrato.
            // 1.- Primero se tiene que crear la cuenta signless y crear un voucher para 
            //     esta con el mismo metodo con el que ya se aplicaba el voucher al usuario 
            //     a pesar de que no tengan tokens,
            // 2.- Se tiene que mandar el mensaje para establecer la sesion, con la misma
            //     cuenta la cual se almacenara como la "sesion", esto es asi para que 
            //     funcione en cuentas que no tienen tokens, esta es una accion "sensible"
            //     ya que la cuenta que se mande como payload, sera la principal del usuario
            //     entonces, es el unico mensaje en el cual se debe de tener cuidado.

            await sendMessageWithSignlessAccount(
                newKeyringPair,
                voucherId,
                0,
                {
                    Register: {
                        user_address: decodeAddress(account.address)
                    }
                }
            );

            // Se retorna el addres de la signless account y del voucher
            resolve([signlessAddress, voucherId as HexString]);
        });
    };

    const unlockActualPair = (password: string): KeyringPair => {
        const actualAccountPair = signlessActualAccount();

        if (!actualAccountPair) throw new Error('Pair not found');
        
        const result = GearKeyring.fromJson(actualAccountPair, password);

        return result;
    };

    const sendMessageWithSignlessAccount = async (
        pair: KeyringPair,
        signlessVoucherId: HexString, 
        value: number, 
        payload?: Payload, 
        onMessageInBlock?: any,
        onMessageSendSuccessfull?: any,
        onMessageErrorWhileSending?: any
    ): Promise<void> => {
        if (!account) throw new Error('No account address');
        if (!api) throw new Error('Api is not loaded');

        return new Promise(async (resolve, reject) => {
            const messagePayload = getSinglessPayload(account.decodedAddress, payload);
            
            const totalGas = await api.program.calculateGas.handle(
                decodeAddress(pair.address),
                MAIN_CONTRACT.PROGRAM_ID,
                payload,
                value,
                false,
                ProgramMetadata.from(MAIN_CONTRACT.METADATA)
            );

            const onSuccess = () => {
                if (onMessageSendSuccessfull) onMessageSendSuccessfull();
                resolve();
            }

            const onError = () => {
                if (onMessageErrorWhileSending) onMessageErrorWhileSending();
                reject("Error while sending message");
            }

            const onInBlock = () => {
                if (onMessageInBlock) onMessageInBlock();
            }

            const message: SendMessageOptions = {
                payload: messagePayload,
                gasLimit: gasToSpend(totalGas),
                value,
                voucherId: signlessVoucherId,
                onSuccess,
                onError,
                onInBlock
            };

            sendMessage(message);
        });
    };

    return {
        signlessAccountsFromLocalStorage,
        signlessActualAccount,
        saveAccountPairToLocalStorage,
        saveAccountSignlessPairInLocalStorage,
        deleteAccountFromActualAccount,
        createNewPairAddress,
        createAndSaveAccountNewPair,
        unlockActualPair,
        sendMessageWithSignlessAccount
    };
}


