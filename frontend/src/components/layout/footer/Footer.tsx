import { Socials } from './socials';
import { Copyright } from './copyright';
import { Logo } from '../header/logo';
import {Link} from 'react-router-dom'
import styles from './Footer.module.scss';

function Footer() {
  return (
    <footer className={styles.footer}>
      <div className={styles.footer_main}>
        <div className={styles.brand}>
          <Logo />
          <p className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl text-center px-2 py-1">
            Evolving Web3 gaming
          </p>
          <Socials />
        </div>
        <div className={styles.footer_sections}>
          <div>
            <h4>Resources</h4>
            <ul>
              <li>
                <Link to='/about'>About us</Link>
              </li>
              <li>
                <Link to='/rules'>Rules</Link>
              </li>
            </ul>
          </div>
          <div>
            <h4>NFTs</h4>
            <ul>
              <li>
                <Link to="/all">All cards</Link>
              </li>
              <li>
                <Link to="/marketplace">Marketplace</Link>
              </li>
            </ul>
          </div>
          <div>
            <h4>Legal</h4>
            <ul>
              <li>
                <a href="/terms">Terms</a>
              </li>
              <li>
                <Link to="/privacy">Privacy</Link>
              </li>
              <li>
                <a href="mailto:rustizados@gmail.com?subject=Rutzo Support">Support</a>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <Copyright />
    </footer>
  );
}

export { Footer };
