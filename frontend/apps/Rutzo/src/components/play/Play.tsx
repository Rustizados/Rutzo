import styles from './play.module.scss';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";

type Props = {
    style: React.CSSProperties;
    id?: string;
};

export function Play({ style, id}: Props) {
    return (
        <div className={styles.highlight} style={{ ...style}} id={id}>
            <a href="/play">
                <GameController/>
                PLAY
            </a>
        </div>
    );
}