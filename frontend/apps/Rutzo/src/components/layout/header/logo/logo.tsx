import { Link } from 'react-router-dom';
import { ReactComponent as RutzoLogo } from  '@/assets/images/logo.svg'
import './logo.module.scss';

function Logo() {
  return (
    <Link to="/">
      <RutzoLogo />
    </Link>
  );
}

export { Logo };
