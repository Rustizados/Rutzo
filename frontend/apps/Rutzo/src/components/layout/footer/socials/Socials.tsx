import { FaGithub } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";
import { FaInstagram } from "react-icons/fa";
import styles from './Socials.module.scss';

const socials = [
  { href: 'https://github.com/brandonhxrr/Rutzo', icon: FaGithub },
  { href: 'https://x.com/Rustizados', icon: FaXTwitter },
  { href: 'https://instagram.com/rustizados', icon: FaInstagram }
];

interface SocialsProps {
  className?: string;
}

function Socials({ className = "" }: SocialsProps) {
  const getItems = () =>
    socials.map(({ href, icon: Icon }) => (
      <li key={href}>
        <a href={href} target="_blank" rel="noreferrer" >
          <Icon className={className}/>
        </a>
      </li>
    ));

  return <ul className={`${styles.socials} ${className}`}>{getItems()}</ul>;
}

export { Socials };
