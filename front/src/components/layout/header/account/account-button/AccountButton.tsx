import Identicon from '@polkadot/react-identicon';
import { buttonStyles } from '@gear-js/ui';
import "./style.css";

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
  isActive?: boolean;
  block?: boolean;
};

function AccountButton({ address, name, onClick, isActive, block }: Props) {
  

  return (
    <button className='boton-ovalado' type="button" onClick={onClick}>
      <Identicon value={address} className={buttonStyles.icon} theme="polkadot" size={28} />
      {name}
    </button>
  );
}

export { AccountButton };
