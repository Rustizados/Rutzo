import NFT1 from 'assets/images/features/NFT1.png';
import NFT2 from 'assets/images/features/NFT2.png';
import NFT3 from 'assets/images/features/NFT3.png';
import Container from 'react-bootstrap/Container';

function Features() {
  return ( 
      <Container style={{ marginBlock: '5%'}} className='text-light' id='Features'>
        <Container style={{ marginBlock: '5%', display: 'flex'}}>
        <img  src={NFT1} style={{width:'30%'}} alt="fireSpot"/>
          <Container style={{alignSelf: 'center', marginLeft: '10%'}}>
            <h2 style={{marginBottom: '5%'}}>Buy NFTs</h2>
            <p>We provide you with a wide range of unique NFTs with different abilities to destroy your enemies!  Each NFT in our collection possesses exceptional powers and attributes that can be harnessed to gain an edge in battle. From fierce warriors wielding legendary weapons to mystical creatures with awe-inspiring abilities, our NFTs offer an unparalleled opportunity to dominate your adversaries.</p>
          </Container>
        </Container>
        <Container style={{ margin: '5%', display: 'flex'}}>
          <Container style={{alignSelf: 'center', marginRight: '10%'}}>
            <h2 style={{marginBottom: '5%'}}>Win more NFTs</h2>
            <p>Defeat your enemies and conquer their NFTs to establish your dominance in the digital realm. As you emerge victorious in battles, seize the opportunity to claim the spoils of war by acquiring their valuable NFTs. Each conquered NFT becomes a symbol of your triumph, a testament to your superior strategy and prowess.</p>
          </Container>
          <img  src={NFT2} style={{width:'30%', height:'30%'}} alt="fireSpot"/>
        </Container>
        <Container style={{ margin: '5%', display: 'flex'}}>
        <img  src={NFT3} style={{width:'30%'}} alt="fireSpot"/>
          <Container style={{alignSelf: 'center', marginLeft: '10%'}}>
            <h2 style={{marginBottom: '5%'}}>Exchange and Upgrade Your NFTs</h2>
            <p>Maximize the potential of your NFT collection by exchanging and upgrading your tokens. Each NFT possesses a distinct level and a set of skills that contribute to its overall power. With our exchange platform, you can trade your NFTs for others of similar rank, enabling you to fine-tune your lineup and optimize your strategic advantages. Whether you&apos;re seeking to enhance your team&apos;s synergy or unlock new abilities, our exchange system offers a seamless way to evolve and improve your NFT roster. Stay ahead of the competition by constantly adapting and refining your collection through strategic exchanges.</p>
          </Container>
        </Container>
    </Container>
  );
}

export { Features };
