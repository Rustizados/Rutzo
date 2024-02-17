import React from 'react';
import { Logo } from './logo';
import { AccountInfo } from './account-info';
// import { Play } from '@/components/play/Play';
import { RedirectionButton } from '@/components/redirection-button/RedirectionButton';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import styles from './header.module.scss';

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
            <a href="/about">About us</a>
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
      <RedirectionButton style={{marginInline: "20px", marginRight: "15px"}} link="/play">
        <GameController/>
        PLAY
      </RedirectionButton>
      {isAccountVisible && <AccountInfo />}
    </header>
  );
}
