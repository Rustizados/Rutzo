import styles from './Card.module.scss';

function Card (){
    return (
    <div className={styles.container}>
        <div className={styles.card}>
            <img alt="nft" src="/hex.jpg" className={styles.hexagon} />

            <div className={styles.content}>
                <h2 className={styles.title}>Card</h2>
                <div>
                    <p>Title</p>
                </div>
                <div>
                    <p>Type</p>
                </div>
                <div>
                    <p>Value</p>
                </div>
            </div>
        </div>
    </div>
    );
}

export { Card };