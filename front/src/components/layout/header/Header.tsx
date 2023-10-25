import React from 'react';
import { Logo } from './logo';
import { Account } from './account';
import styles from './Header.module.scss';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  const [isMenuOpen] = React.useState(false);

  return (
    <header className={styles.header}>
      <Logo />
      <div className={`${styles.nav} ${isMenuOpen ? styles.open : ''}`}>
        <ul className={`${styles.list} ${isMenuOpen ? styles.show : ''}`}>
          <li><a href="/#features">FEATURES</a></li>
          <li><a href="/#how-to-start">HOW TO START</a></li>
          <li><a href="/#faq">FAQ</a></li>
          <li><a href="/marketplace">MARKETPLACE</a></li>
        </ul>
      </div>
      <div className={styles.highlight}><a href="/play">PLAY</a></div>
      {isAccountVisible && <Account />}
    </header>
  );
}

export { Header };
