import { FaGithub } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";
import { FaInstagram } from "react-icons/fa";
import styles from './Socials.module.scss';

const socials = [
  { href: 'https://github.com/brandonhxrr/Rutzo', icon: FaGithub },
  { href: 'https://x.com/Rustizados', icon: FaXTwitter },
  { href: 'https://instagram.com/rustizados', icon: FaInstagram }
];

function Socials() {
  const getItems = () =>
    socials.map(({ href, icon: Icon }) => (
      <li key={href}>
        <a href={href} target="_blank" rel="noreferrer">
          <Icon />
        </a>
      </li>
    ));

  return <ul className={styles.socials}>{getItems()}</ul>;
}

export { Socials };
