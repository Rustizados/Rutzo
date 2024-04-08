import React from 'react';
import { Logo } from './logo';
import { AccountInfo } from './account-info';
// import { Play } from '@/components/play/Play';
import { Link } from 'react-router-dom';
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
            <Link to="/about">About</Link>
          </li>
          <li>
           <Link to="/#how-to-start">How to start</Link>
          </li>
          <li>
           <Link to={"/#faq"}>FAQ</Link>
          </li>
          <li>
            <Link to="/marketplace">Marketplace</Link>
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
