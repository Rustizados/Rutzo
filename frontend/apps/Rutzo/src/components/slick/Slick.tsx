import React, {ReactNode} from 'react';
import Slider from "react-slick";
import "slick-carousel/slick/slick.css";
import "slick-carousel/slick/slick-theme.css";
interface SlickProps {
	children: ReactNode;
}

const styles = {
	width: "100%",
	backgroundColor: "red"
}

const Slick = ({children}: SlickProps) => {
	const settings = {
		dots: true,
		infinite: true,
		speed: 500,
		slidesToShow: 1,
		slidesToScroll: 1
	};
	return (
		<Slider {...settings} >
			<div style={styles}>{children}</div>

		</Slider>
	);
}


export {Slick};
