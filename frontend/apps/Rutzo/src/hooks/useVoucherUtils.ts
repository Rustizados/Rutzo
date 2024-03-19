import { GearKeyring, HexString, IUpdateVoucherParams } from '@gear-js/api';
import { useAccount, useApi, useAlert, TemplateAlertOptions, useBalanceFormat } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, ONE_TVARA_VALUE, VOUCHER_MIN_LIMIT, seed } from '@/app/consts';

const useVoucherUtils = () => {
    const { getFormattedBalanceValue } = useBalanceFormat();
    const { api } = useApi();
    const { account } = useAccount();
    const alert = useAlert();

    const createNewVoucher = (account: HexString): Promise<HexString> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                console.log("No se inicio la api");
                reject("Error creating voucher");
                return;
            }

            const alertOptions: TemplateAlertOptions = {
                title: "Rutzo action"
            }

            const creatingVoucherAlertId = alert.loading("Creating voucher", alertOptions);

            // Se genera el "issue" para crear el voucher para el usuario
            // En este caso, para el main contract
            const voucherIssued =  await api.voucher.issue(
                account,
                ONE_TVARA_VALUE * 11, // 11 TVaras
                1_200, // An hour in blocks
                [MAIN_CONTRACT.PROGRAM_ID]
            );

            console.log("voucher issued");

            const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

            // Se firma el voucher con la cuenta del administrador para el main Contract

            try {
                await voucherIssued.extrinsic.signAndSend(
                    keyring,
                    async (event: any) => {
                        console.log(event.toHuman()); 
                        const extrinsicJSON: any = event.toHuman();
                        if (extrinsicJSON && extrinsicJSON.status !== "Ready") {
                            const objectKey = Object.keys(extrinsicJSON.status)[0];
                            if (objectKey === "Finalized") {
                                alert.remove(creatingVoucherAlertId);
                                alert.success("Voucher created");
                                console.log("Voucher created");
                                resolve(voucherIssued.voucherId);
                            }
                        }
                    }
                );
            } catch (error: any) {
                console.error(`${error.name}: ${error.message}`);
                alert.remove(creatingVoucherAlertId);
                alert.error("Error creating voucher");
                reject("Error creating voucher");
            }
        });
    }

    const updateVoucher = async (account: HexString, voucherId: string): Promise<void> => {
        return new Promise(async (resolve, reject) => {
            if (await voucherExpired(voucherId)) {
                await renewVoucherOneHour(voucherId);
            }
            
            const actualVoucherBalance = await voucherBalance(voucherId);

            if (actualVoucherBalance < VOUCHER_MIN_LIMIT) {
                await addTwoTokensToVoucher(voucherId);
            }

            resolve();
        });
    };

    const voucherExpired = async (voucherId: string): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api || !account) {
                console.log("Api or Account is not ready");
                reject(false);
                return;
            }
    
            const voucherData = await api.voucher.getDetails(account.decodedAddress, voucherId);
            const blockHash = await api.blocks.getFinalizedHead();
            const blocks = await api.blocks.getBlockNumber(blockHash as Uint8Array);
    
            resolve(blocks.toNumber() > voucherData.expiry);
        });
    }

    const voucherBalance = async (voucherId: string): Promise<number> => {
        return new Promise(async (resolve, reject) => {
            if (!api || !account) {
                console.log("api or account is not ready");
                reject(false);
                return;
            }

            const voucherBalance = await api.balance.findOut(voucherId);
            const voucherBalanceFormated = Number(getFormattedBalanceValue(voucherBalance.toString()).toFixed());

            resolve(voucherBalanceFormated);
        });
    }

    const voucherExists = async (account: HexString): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                console.log("api is not ready");
                reject(false);
                return;
            }

            const vouchers = await api.voucher.getAllForAccount(account, MAIN_CONTRACT.PROGRAM_ID);

            resolve(
                Object.keys(vouchers).length > 0
            );
        });
    }

    const accountVoucherId = async (account: HexString): Promise<string> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                console.log("api is not ready");
                reject(false);
                return;
            }

            const vouchersData = await api.voucher.getAllForAccount(account, MAIN_CONTRACT.PROGRAM_ID);
            const vouchersId = Object.keys(vouchersData);

            if (vouchersId.length < 1) {
                console.log("User does not has voucher");
                reject(false);
                return;
            }

            resolve(vouchersId[0]);
        });
    }

    const renewVoucherOneHour = async (voucherId: string): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api || !account) {
                console.log("Api or Account is not ready");
                reject(false);
                return;
            }

            const alertOptions: TemplateAlertOptions = {
                title: "Rutzo action"
            }

            const renewVoucherAlertId = alert.loading("Renewing voucher", alertOptions);

            const newVoucherData: IUpdateVoucherParams = {
                prolongDuration: 1_200 // one hour
            }

            const voucherUpdate = api.voucher.update(
                account.decodedAddress, 
                voucherId, 
                newVoucherData
            );

            const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

            try {
                await voucherUpdate.signAndSend(
                    keyring,
                    async (event: any) => {
                        console.log(event.toHuman()); 
                        const extrinsicJSON: any = event.toHuman();
                        if (extrinsicJSON && extrinsicJSON.status !== "Ready") {
                            const objectKey = Object.keys(extrinsicJSON.status)[0];
                            if (objectKey === "Finalized") {
                                alert.remove(renewVoucherAlertId);
                                alert.success("Voucher updated");
                                console.log("Voucher updated");
                                resolve(true);
                            }
                        }
                    }
                );
            } catch (error: any) {
                alert.remove(renewVoucherAlertId);
                alert.error("Error renewing voucher");
                console.error(`${error.name}: ${error.message}`);
                reject(false);
            }
        });
    }

    const addTwoTokensToVoucher = async (voucherId: string): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api || !account) {
                console.log("Api or Account is not ready");
                reject(false);
                return;
            }

            const alertOptions: TemplateAlertOptions = {
                title: "Rutzo action"
            }

            const renewVoucherAlertId = alert.loading("Adding tokens to voucher", alertOptions);

            const newVoucherData: IUpdateVoucherParams = {
                balanceTopUp: ONE_TVARA_VALUE * 2
            }

            const voucherUpdate = api.voucher.update(
                account.decodedAddress, 
                voucherId, 
                newVoucherData
            );

            const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

            try {
                await voucherUpdate.signAndSend(
                    keyring,
                    async (event: any) => {
                        console.log(event.toHuman()); 
                        const extrinsicJSON: any = event.toHuman();
                        if (extrinsicJSON && extrinsicJSON.status !== "Ready") {
                            const objectKey = Object.keys(extrinsicJSON.status)[0];
                            if (objectKey === "Finalized") {
                                alert.remove(renewVoucherAlertId);
                                alert.success("Voucher updated");
                                console.log("Voucher updated");
                                resolve(true);
                            }
                        }
                    }
                );
            } catch (error: any) {
                alert.remove(renewVoucherAlertId);
                alert.error("Error adding tokens to voucher");
                console.error(`${error.name}: ${error.message}`);
                reject(false);
            }
        });
    }

    return { 
        createNewVoucher, 
        voucherExpired, 
        voucherBalance, 
        voucherExists, 
        renewVoucherOneHour,
        accountVoucherId,
        addTwoTokensToVoucher,
        updateVoucher
    };
}

export default useVoucherUtils;