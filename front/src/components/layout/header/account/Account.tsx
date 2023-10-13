import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { AccountsModal } from './accounts-modal';
import { Wallet } from './wallet';

function Account() {
  const { account, accounts } = useAccount();
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  return (
    <>
      {account ? (
        <Wallet balance={account.balance} address={account.address} name={account.meta.name} onClick={openModal} />
      ) : (
        <button type="button" onClick={openModal}> Connect Your Wallet</button>
      )}
      {isModalOpen && <AccountsModal accounts={accounts} close={closeModal} />}
    </>
  );
}

export { Account };
