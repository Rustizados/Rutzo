import { GearKeyring, HexString, IUpdateVoucherParams, ProgramMetadata } from '@gear-js/api';
import { useAccount, useApi, useAlert, TemplateAlertOptions, useBalanceFormat } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, ONE_TVARA_VALUE, seed } from '@/app/consts';
import { Signer } from '@polkadot/types/types';
import useVoucherUtils from './useVoucherUtils';
import { gasToSpend } from '@/app/utils';

const useContractUtils = () => {
    const { getFormattedBalanceValue } = useBalanceFormat();
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
    const { api } = useApi();
    const { account } = useAccount();
    const alert = useAlert();
    const {
        voucherExists,
        accountVoucherId,
        updateVoucher
    } = useVoucherUtils();

    const sendMessage = async (signer: Signer): Promise<void> => {
        return new Promise (async (resolve, reject) => {

            const transferExtrinsic = api.message.send({
                destination: MAIN_CONTRACT.PROGRAM_ID,
                payload: { Register: null },
                gasLimit: gasToSpend(gas),
                value: 0,
                prepaid: true,
                account: account.decodedAddress
              }, mainContractMetadata);

        });
    }


    const sendMessageWithVoucher = async (account: HexString, payload: any): Promise<void> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                console.log("Api is not ready");
                reject("Apl is not ready");
                return;
            }

            if (!await voucherExists(account)) {
                reject("User dont have a voucher for main contract");
                return;
            }

            const voucherId = await accountVoucherId(account);

            await updateVoucher(account, voucherId);

            const gas = await api.program.calculateGas.handle(
                account,
                MAIN_CONTRACT.PROGRAM_ID,
                payload,
                0,
                true,
                mainContractMetadata
            );

            const transferExtrinsic = api.message.send({
                destination: MAIN_CONTRACT.PROGRAM_ID,
                payload: { Register: null },
                gasLimit: gasToSpend(gas),
                value: 0,
                prepaid: true,
                account
            }, mainContractMetadata);
        });
    }

    return 
}