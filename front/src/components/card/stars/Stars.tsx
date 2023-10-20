import { v4 as uuidv4 } from 'uuid';
import styles from './Stars.module.scss';

function Stars({ num }: { num: number }) {
    return (
        <>
            {Array.from({ length: num }).map((n, index) => (
                <svg key={uuidv4()} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="yellow" className={styles.star}>
                    {/* ...contenido del svg... */}
                </svg>
            ))}
            {Array.from({ length: 3 - num }).map((n, index) => (
                <svg key={uuidv4()} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="yellow" className={styles.star}>
                    {/* ...contenido del svg... */}
                </svg>
            ))}
        </>
    );
}

export { Stars };
