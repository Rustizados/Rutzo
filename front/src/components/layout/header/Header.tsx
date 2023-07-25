import React from 'react';
import { Logo } from './logo';
import { Account } from './account';
import styles from './Header.module.scss';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  const [isMenuOpen, setIsMenuOpen] = React.useState(false);

  return (
    <header className={styles.header}>
      <Logo />
      <div className={`${styles.nav} ${isMenuOpen ? styles.open : ''}`}>
        <ul className={`${styles.list} ${isMenuOpen ? styles.show : ''}`}>
          {/* Menú para pantallas más grandes */}
          <li><a href="/#features" className='link-light'>FEATURES</a></li>
          <li><a href="/#how-to-start" className='link-light'>HOW TO START</a></li>
          <li><a href="/#faq" className='link-light'>FAQ</a></li>
          <li><a href="/marketplace" className='link-light'>MARKETPLACE</a></li>
        </ul>
      </div>
      <div className={styles.highlight}><a href="/play">PLAY</a></div>
      {isAccountVisible && <Account />}
    </header>
  );
}

export { Header };
