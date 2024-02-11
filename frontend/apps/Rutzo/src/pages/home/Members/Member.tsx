import { FaGithub } from "react-icons/fa";
import { FaInstagram } from "react-icons/fa";
import { FaLinkedin } from "react-icons/fa";
import styles from './Members.module.scss'

interface MemberProps {
    name: string;
    role: string;
    github: string;
    linkedin: string;
    instagram: string;
    image: string;
}

function Member({ name, role, github, linkedin, instagram, image }: MemberProps) {
    return (
        <div className={styles.member_container}>
            <img src={image} alt="step" className={styles.step_img}/>
            <div className={styles.content}>
                <h4 className={styles.member_name}>{name}</h4>
                <p className={styles.member_role}>{role}</p>
                <div className={styles.member_links}>
                    { github !== "" ?
                        <a href={"https://github.com/" + github} target="_blank" rel="noreferrer">
                            <FaGithub />
                        </a> : ""
                    }
                    { linkedin !== "" ?
                        <a href={"https://linkedin.com/in/" + linkedin} target="_blank" rel="noreferrer">
                            <FaLinkedin />
                        </a> : ""
                    }
                    { instagram !== "" ?
                        <a href={"https://instagram.com/" + instagram} target="_blank" rel="noreferrer">
                            <FaInstagram />
                        </a> : ""    
                    }
                </div>
            </div>            
        </div>
        );
}

export { Member };