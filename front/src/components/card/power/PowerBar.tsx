import styles from './PowerBar.module.scss';

function PowerBar({ progress }: { progress: number }) {
  return (
    <div className={styles.progressBar}>
      <div
        className={styles.progress}
        style={{ width: `${progress}%` }}
      />
    </div>
  );
};

export {PowerBar};