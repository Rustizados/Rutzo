import { Socials } from './socials';
import { Copyright } from './copyright';
import { Logo } from '../header/logo';
import styles from './Footer.module.scss';

function Footer() {
return (
<footer className={styles.footer}>
  <div className={styles.footer_main}>
    <div className={styles.brand}>
      <Logo />
      <p>This growth plan will help you reach your resolutions and achieve the goals you have been striving towards.</p>
      <Socials />
    </div>
    <div className={styles.footer_sections}>
      <div className="marketplace">
        <h4>Marketplace</h4>
        <ul>
          <li>
            <a href="/marketplace">All cards</a>
          </li>
          <li>
            <a href="/marketplace">How we design NFTs</a>
          </li>
          <li>
            <a href="/marketplace">Your collection</a>
          </li>
        </ul>
      </div>
      <div className="about">
        <h4>Resources</h4>
        <ul>
          <li>
            <a href="/#features">About us</a>
          </li>
          <li>
            <a href="/#how-to-start">Game rules</a>
          </li>
          <li>
            <a href="/#faq">Tutorial</a>
          </li>
        </ul>
      </div>
      <div className="resources">
        <h4>Company</h4>
        <ul>
          <li>
            <a href="/#features">Media</a>
          </li>
          <li>
            <a href="/#how-to-start">Blog</a>
          </li>
          <li>
            <a href="/#faq">Pricing</a>
          </li>
        </ul>
      </div>
      <div className="company">
        <h4>Legal</h4>
        <ul>
          <li>
            <a href="/terms">Terms</a>
          </li>
          <li>
            <a href="/privacy">Privacy</a>
          </li>
          <li>
            <a href="mailto:rustizados@gmail.com?subject=Rutzo Support">Support</a>
          </li>
        </ul>
      </div>
    </div>
  </div>
  < Copyright />
</footer>
);
}

export { Footer };
