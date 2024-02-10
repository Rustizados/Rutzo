import styles from './play.module.scss';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";

type Props = {
    style: React.CSSProperties;
    id?: string;
    link: string;
};

export function Play({ style, id, link}: Props) {
    return (
        <div className={styles.highlight} style={{ ...style}} id={id}>
            <a href={link}>
                <GameController/>
                PLAY
            </a>
        </div>
    );
}