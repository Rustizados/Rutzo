import './Features.css';

interface FeatureProps {
  image: string;
  title: string;
  content: string;
}

function Feature({ image, title, content }: FeatureProps) {
  return (
    <div className='feature'>
      <div className='image-container'>
        <img src={image} alt='fireSpot' />
      </div>
      <div className='content'>
        <h2>{title}</h2>
        <p>{content}</p>
      </div>
    </div>
  );
}

export { Feature };
