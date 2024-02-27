import styles from './RedirectionButton.module.scss';

type Props = {
    style: React.CSSProperties;
    id?: string;
    link: string;
    children?: any
};

export function RedirectionButton({style, id, link, children}: Props) {
    return (
        <div className={styles.highlight} style={{ ...style}} id={id}>
            <a href={link}>
                {children}
            </a>
        </div>
    );
}