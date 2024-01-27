import { Member } from './Member';
import styles from './Members.module.scss'

function Members() {
return (
<div className={styles.members_container}>
    < Member 
        name="Brandon Herrera" 
        role="Frontend Developer & UI/UX Designer" 
        github="brandonhxrr" 
        linkedin="brandonhxrr" instagram="brandonhxrr"
        image="https://avatars.githubusercontent.com/u/39093860?v=4"
    />

    < Member 
        name="David Hernández" 
        role="Backend Developer" 
        github="David-HernandezM" 
        linkedin="" 
        instagram=""
        image="https://avatars.githubusercontent.com/u/67880616?v=4"
    />

    < Member 
        name="Juan M. Hernández" 
        role="Frontend Developer" 
        github="JuanH44" 
        linkedin="juanmhdez" 
        instagram="jma_hdz"
        image="https://avatars.githubusercontent.com/u/61924317?v=4" 
    />
</div>
);
}

export { Members };
