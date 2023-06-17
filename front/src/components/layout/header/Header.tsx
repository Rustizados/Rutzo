import { Logo } from './logo';
import { Account } from './account';
import styles from './Header.module.scss';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  return (
    <header className={styles.header}>
      <Logo />
        <nav className={styles.nav}>
            <ul className={styles.list}>
                <li><a href="/about" className='link-light'>ABOUT</a></li>
                <li><a href="/caracteristicas" className='link-light'>FEATURES</a></li>
                <li><a href="/start" className='link-light'>START</a></li>
                <li><a href="faq" className='link-light'>MARKETPLACE</a></li>
                <li className={styles.highlight}><a href="faq">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={3.5} stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
                    </svg>
                    JUGAR
                </a></li>
            </ul>
        </nav>
      {isAccountVisible && <Account />}
    </header>
  );
}

export { Header };
