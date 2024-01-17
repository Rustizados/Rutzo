import React from 'react';
import { Logo } from './logo';
import { AccountInfo } from './account-info';
import styles from './header.module.scss';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";

type Props = {
  isAccountVisible: boolean;
};

export function Header({ isAccountVisible }: Props) {
  const [isMenuOpen] = React.useState(false);

  return (
    <header className={styles.header}>
      <Logo />
      <div className={`${styles.nav} ${isMenuOpen ? styles.open : ''}`}>
        <ul className={`${styles.list} ${isMenuOpen ? styles.show : ''}`}>
          <li>
            <a className='text-white' href="/#features">Features</a>
          </li>
          <li>
            <a href="/#how-to-start">How to start</a>
          </li>
          <li>
            <a href="/#faq">FAQ</a>
          </li>
          <li>
            <a href="/marketplace">Marketplace</a>
          </li>
        </ul>
      </div>
      <div className={styles.highlight}>
        <a href="/play">
          <GameController/>
          PLAY
        </a>
      </div>
      {isAccountVisible && <AccountInfo />}
    </header>
  );

  // return (
  //   <>
  //     <header className={styles.header}>
  //       <Container className={styles.header__container}>
  //         <Logo className={styles.header__logo} />
  //         <AccountInfo openWallet={openAndCloseChange} isOpen={isOpenChange} />
  //       </Container>
  //       {isOpenChange && (
  //         <Container>
  //           <WalletChange onClose={openAndCloseChange} openConnectWallet={openConnectWallet} />
  //         </Container>
  //       )}
  //     </header>

  //     <ModalBackground isOpen={isOpenChange} onClick={closeChange} />

  //     <AnimatePresence>{isOpenConnectWallet && <WalletConnect onClose={closConnectWallet} />}</AnimatePresence>
  //   </>
  // );
}
