import { Link } from 'react-router-dom';
import { ReactComponent as RutzoLogo } from  '@/assets/images/logo.svg'
// import './logo.module.scss';

const  Logo = () => {
  return (
    <div>
      <Link to="/">
        <div> <RutzoLogo /> </div>
      </Link>
    </div>
  );
}

export { Logo };
