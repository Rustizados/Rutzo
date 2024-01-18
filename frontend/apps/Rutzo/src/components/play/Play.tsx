import styles from './play.module.scss';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";

type Props = {
    style: React.CSSProperties;
};

export function Play({ style }: Props) {
    return (
        <div className={styles.highlight} style={{ ...style}}>
            <a href="/play">
                <GameController/>
                PLAY
            </a>
        </div>
    );
}