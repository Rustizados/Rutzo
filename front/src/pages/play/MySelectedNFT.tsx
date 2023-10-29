import { Card } from '../../components/card/Card';

function InfoNFT({ name, description, media, reference }: any) {
  return (
    <Card image={media} title={name} type={description} value={reference} price={reference}/>
  );
}

function MySelectedNFT({ name, description, media, reference }: any) {
  return (
    <Card image={media} title={name} type={description} value={reference} price={reference}/>
  );
}

export { MySelectedNFT };
