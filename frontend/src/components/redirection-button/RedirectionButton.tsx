import styles from './RedirectionButton.module.scss';
import {Link} from 'react-router-dom'

type Props = {
    style: React.CSSProperties;
    id?: string;
    link: string;
    onClick?: () => void;
    children?: any
};

export function RedirectionButton({style, id, link, children}: Props) {
    return (
        <div className={styles.highlight} style={{ ...style}} id={id}>
            <Link to={link}>
              {children}
            </Link>
        </div>
    );
}
