import './Feature.module.scss';

interface FeatureProps {
  image: string;
  title: string;
  content: string;
}

function Feature({ image, title, content }: FeatureProps) {
  return (
    <div className='feature'>
      <img src={image} style={{ width: '30%', height: '30%' }} alt="fireSpot" />
      <div className='content'>
        <h2>{title}</h2>
        <p>{content}</p>
      </div>
    </div>
  );
}

export { Feature };
