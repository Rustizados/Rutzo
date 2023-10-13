
import { Account } from './account';
import styles from './Header.module.scss';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  return (
    <header className={styles.header}>
      <h1>Logo</h1>
      {isAccountVisible && <Account />}
    </header>
  );
}

export { Header };
