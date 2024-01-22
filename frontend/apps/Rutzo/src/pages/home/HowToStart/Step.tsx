import { IconType } from 'react-icons/lib';
import { TbExternalLink } from "react-icons/tb";
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
            <h2>
                {title}
                <span><a href={url} className="link-light">{url_text}
                {" "}
                {url != "" ? < TbExternalLink /> : ""}
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