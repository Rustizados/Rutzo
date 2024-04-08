import { IconType } from 'react-icons/lib';
import styles from './HowToStart.module.scss'

interface StepProps {
    icon: IconType;
    title: string;
    url_text: string;
    url: string;
    content: string;
    image: string;
}

function Step({ icon: Icon, title, url_text, url, content, image }: StepProps) {
    return (
        <div className={styles.step_container}>
            <Icon style={{ fontSize: "400%" }} className={styles.step_icon}/>
            <h2 className="mb-4 text-4xl md:text-3xl lg:text-5xl dark:text-white">
                {title}
                <span className="underline underline-offset-3 decoration-8 decoration-slate-50 dark:decoration-blue-600"><a href={url}>{url_text}
                </a></span>
            </h2>
            <p>
                {content}
            </p>
            <img src={image} alt="step" className={styles.step_img}/>
        </div>
        );
}

export { Step };