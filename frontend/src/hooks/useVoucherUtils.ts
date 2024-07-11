import { GearKeyring, HexString } from '@gear-js/api';
import { useAccount, useApi, useAlert, TemplateAlertOptions, useBalanceFormat } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, ONE_TVARA_VALUE, VOUCHER_MIN_LIMIT, seed } from '@/app/consts';

const useVoucherUtils = () => {
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();

  const createNewVoucher = (account: HexString): Promise<HexString> => {
    return new Promise((resolve, reject) => {
      if (!api) {
        console.log("No se inicio la api");
        reject("Error creating voucher");
        return;
      }

      const alertOptions: TemplateAlertOptions = {
        title: "Rutzo action"
      };

      const creatingVoucherAlertId = alert.loading("Creating voucher", alertOptions);

      api.voucher.issue(
        account,
        ONE_TVARA_VALUE * 11,
        1_200,
        [MAIN_CONTRACT.PROGRAM_ID]
      ).then((voucherIssued) => {
        console.log("voucher issued");

        GearKeyring.fromSeed(seed, "AdminDavid").then((keyring) => {
          voucherIssued.extrinsic.signAndSend(
            keyring,
            (event: any) => {
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
          ).catch((error: any) => {
            console.error(`${error.name}: ${error.message}`);
            alert.remove(creatingVoucherAlertId);
            alert.error("Error creating voucher");
            reject("Error creating voucher");
          });
        });
      });
    });
  };

  const updateVoucher = (account: HexString, voucherId: string): Promise<void> => {
    return new Promise((resolve, reject) => {
      voucherExpired(voucherId).then((expired) => {
        if (expired) {
          renewVoucherOneHour(voucherId).then(() => {
            voucherBalance(voucherId).then((balance) => {
              if (balance < VOUCHER_MIN_LIMIT) {
                addTwoTokensToVoucher(voucherId).then(() => resolve()).catch(reject);
              } else {
                resolve();
              }
            }).catch(reject);
          }).catch(reject);
        } else {
          voucherBalance(voucherId).then((balance) => {
            if (balance < VOUCHER_MIN_LIMIT) {
              addTwoTokensToVoucher(voucherId).then(() => resolve()).catch(reject);
            } else {
              resolve();
            }
          }).catch(reject);
        }
      }).catch(reject);
    });
  };

  const voucherExpired = (voucherId: string): Promise<boolean> => {
    return new Promise((resolve, reject) => {
      if (!api || !account) {
        console.log("Api or Account is not ready");
        reject(false);
        return;
      }

      api.voucher.getDetails(account.decodedAddress, voucherId).then((voucherData) => {
        api.blocks.getFinalizedHead().then((blockHash) => {
          api.blocks.getBlockNumber(blockHash as Uint8Array).then((blocks) => {
            resolve(blocks.toNumber() > voucherData.expiry);
          }).catch(reject);
        }).catch(reject);
      }).catch(reject);
    });
  };

  const voucherBalance = (voucherId: string): Promise<number> => {
    return new Promise((resolve, reject) => {
      if (!api || !account) {
        console.log("api or account is not ready");
        reject(false);
        return;
      }

      api.balance.findOut(voucherId).then((voucherBalance) => {
        const voucherBalanceFormated = Number(getFormattedBalanceValue(voucherBalance.toString()).toFixed());
        resolve(voucherBalanceFormated);
      }).catch(reject);
    });
  };

  const voucherExists = (account: HexString): Promise<boolean> => {
    return new Promise((resolve, reject) => {
      if (!api) {
        console.log("api is not ready");
        reject(false);
        return;
      }

      api.voucher.getAllForAccount(account, MAIN_CONTRACT.PROGRAM_ID).then((vouchers) => {
        resolve(Object.keys(vouchers).length > 0);
      }).catch(reject);
    });
  };

  const accountVoucherId = (account: HexString): Promise<string> => {
    return new Promise((resolve, reject) => {
      if (!api) {
        console.log("api is not ready");
        reject(false);
        return;
      }

      api.voucher.getAllForAccount(account, MAIN_CONTRACT.PROGRAM_ID).then((vouchersData) => {
        const vouchersId = Object.keys(vouchersData);

        if (vouchersId.length < 1) {
          console.log("User does not has voucher");
          reject(false);
          return;
        }

        resolve(vouchersId[0]);
      }).catch(reject);
    });
  };

  const renewVoucherOneHour = (voucherId: string): Promise<boolean> => {
    return new Promise((resolve, reject) => {
      if (!api || !account) {
        console.log("Api or Account is not ready");
        reject(false);
        return;
      }

      const alertOptions: TemplateAlertOptions = {
        title: "Rutzo action"
      };

      const renewVoucherAlertId = alert.loading("Renewing voucher", alertOptions);

      const voucherUpdate = api.voucher.update(
        account.decodedAddress, 
        voucherId, 
        {
            prolongDuration: 1_200 // one hour
        }
      );

      GearKeyring.fromSeed(seed, "AdminDavid").then((keyring) => {
        voucherUpdate.signAndSend(
          keyring,
          (event: any) => {
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
        ).catch((error: any) => {
          alert.remove(renewVoucherAlertId);
          alert.error("Error renewing voucher");
          console.error(`${error.name}: ${error.message}`);
          reject(false);
        });
      }).catch(reject);
    });
  };

  const addTwoTokensToVoucher = (voucherId: string): Promise<boolean> => {
    return new Promise((resolve, reject) => {
      if (!api || !account) {
        console.log("Api or Account is not ready");
        reject(false);
        return;
      }

      const alertOptions: TemplateAlertOptions = {
        title: "Rutzo action"
      };

      const renewVoucherAlertId = alert.loading("Adding tokens to voucher", alertOptions);

      const voucherUpdate = api.voucher.update(
        account.decodedAddress, 
        voucherId, 
        {
            balanceTopUp: ONE_TVARA_VALUE * 2
        }
      );

      GearKeyring.fromSeed(seed, "AdminDavid").then((keyring) => {
        voucherUpdate.signAndSend(
          keyring,
          (event: any) => {
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
        ).catch((error: any) => {
          alert.remove(renewVoucherAlertId);
          alert.error("Error adding tokens to voucher");
          console.error(`${error.name}: ${error.message}`);
          reject(false);
        });
      }).catch(reject);
    });
  };

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
};

export default useVoucherUtils;
